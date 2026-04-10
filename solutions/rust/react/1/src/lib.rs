use std::collections::{HashMap, HashSet};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111); 
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {// 这里不能有 F和B的泛型
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    cells: HashMap<CellId, (T,  Option<Vec<CellId>>, Option<Box<dyn Fn(&[T]) -> T + 'a>>)>,
    pub downstream: HashMap<CellId, HashSet<CellId>>,
    callback: HashMap<CallbackId, Box<dyn FnMut(T) + 'a>>, // 主要是闭包抓取的可变引用的生命周期要比较长
    callback_of_compute: HashMap<CellId, HashSet<CallbackId>>,
    input_id: usize,
    compute_id: usize,
    callback_id: usize
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self { 
            cells: HashMap::new(),
            downstream: HashMap::new(),
            callback: HashMap::new(),
            callback_of_compute: HashMap::new(),
            input_id: 0,
            compute_id: 0,
            callback_id: 0,
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.input_id += 1;
        let input = InputCellId(self.input_id);
        self.cells.insert(CellId::Input(input), (initial, None, None));
        input
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {

        for dependency in dependencies {
            if  self.cells.get(dependency).is_none() {
                return Err(*dependency)
            }
        }

        self.compute_id += 1;
        let cumpute = ComputeCellId(self.compute_id);
        let mut args: Vec<T> = Vec::with_capacity(dependencies.len());
        for dependency in dependencies {
            if let Some(&(v, _, _)) = self.cells.get(dependency) {
                    self.downstream.entry(*dependency).or_insert(HashSet::new()).insert(CellId::Compute(cumpute));
                    args.push(v)
            }
        }
        let initial = compute_func(&args);
        // self.cells.insert(CellId::Compute(cumpute), (initial, Some(dependencies), Some(Box::(compute_func) as Box<dyn Fn(&[T] -> T)>)));
        self.cells.insert(
            CellId::Compute(cumpute),
            (
                initial,
                Some(dependencies.to_vec()),
                Some(Box::new(compute_func) as Box<dyn Fn(&[T]) -> T>),
            ),
        );
        Ok(cumpute)
    }


    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        self.cells.get(&id).map(|&(v, _, _)| v)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let cell_id = CellId::Input(id);
        match self.cells.get_mut(&cell_id) {
            Some(_) => {
                let mut callbacks = HashMap::<(CellId, CallbackId), T>::new();
                self.set_cell_value(cell_id, new_value, &mut callbacks);

                for ((cell_id, callback_id), old_value) in callbacks{
                    let new_value = self.cells.get(&cell_id).unwrap().0;
                    if new_value != old_value { // 更新 callback
                        // self.callback.get_mut(&callback_id).as_mut().unwrap()(new_value);// // 为什么这里要用as_mut ?
                        if let Some(cb) = self.callback.get_mut(&callback_id) {
                            cb(new_value);
                        }
                    }
                }
                true
            }
            None => false
        }
    }

    // 可能会被调用多次;
    fn set_cell_value(&mut self, id: CellId, value: T, callbacks: &mut HashMap<(CellId, CallbackId), T>){
       match self.cells.get_mut(&id) { // cells是HashMap<CellId, (T, Option<Vec<CellId, Global>>, Option<Box<dyn Fn(&[T]) -> T + 'a>, 可以对cells中不同的value保持多个可变借吗？
                                       // 对cells 的可变借用 和对Self的借用之间有什么关系，以及其他Self的字段的不可变引用有什么关系？
            Some((v, _, _)) => { // 持有cells某个value的可变引用
                if *v == value {
                    return;
                }
                // println!("{:?}", (*v, value));
                if self.callback_of_compute.contains_key(&id) {
                    for callback_id in self.callback_of_compute.get(&id).as_ref().unwrap().iter() {
                        if !callbacks.contains_key(&(id, *callback_id)) {
                            callbacks.insert((id, *callback_id), *v); // 记录老的值
                        }

                    }
                }
                 *v = value; // 解引用可变借用, v的使用结束，不影响下面不可变引用， 对cells的可变引用也结束了！

                if self.downstream.contains_key(&id) { // 即使 v 的使用不结束，这里是不是也不会有问题？
                    // for down_id in self.downstream.get(&id).unwrap() { // downstream 是 HashMap<CellId, Vec<CellId>>， 持有value的不可变引用，和后面的递归冲突，如何避免？
                    // 实际上只有cells会被改变，downstream不会被改变，只有多个downstream的不可变引用，和 一个cells的可变引用不影响
                    // println!("{:?}", self.downstream);
                    for down_id in self.downstream.get(&id).unwrap().clone().iter() { // downstream 是 HashMap<CellId, Vec<CellId>>， 强制复制，避免只有不可变引用
                        let mut args: Vec<T> = Vec::new();
                        let (_, deps, f) = self.cells.get(down_id).unwrap(); // 持有 cell 的 其他value的不可变引用, 与v不冲突？
                        for dep in deps.as_ref().unwrap().iter() {
                            match self.cells.get(dep) {
                                Some((arg, _, _)) => args.push(*arg),
                                None => continue
                            }
                        } // 对deps的不可变引用结束
                        if f.is_some(){
                            let data = f.as_ref().unwrap()(&args); // 不可变引用
                            self.set_cell_value(*down_id, data, callbacks); // 实际上只有cells会被改变，downstream不会被改变
                        }
                    }
                }
            }
            None => return
        }
    }
    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {

        let cell_id = CellId::Compute(id);

        match self.cells.get(&cell_id) {
            Some(_) => {
                self.callback_id += 1;
                let callback_id = CallbackId(self.callback_id);
                self.callback.insert(callback_id, Box::new(callback) as Box<dyn FnMut(T)>);
                // self.callback_of_compute.insert(id, callback_id);
                self.callback_of_compute.entry(cell_id).or_insert(HashSet::new()).insert(callback_id);
                Some(callback_id)
            }
                None => None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let cell_id = CellId::Compute(cell);
        if !self.callback.contains_key(&callback) {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        if !self.callback_of_compute.contains_key(&cell_id) {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        self.callback.remove(&callback);
        self.callback_of_compute.get_mut(&cell_id).unwrap().retain(|&x| x != callback);

        Ok(())
    }
}
