mod pre_implemented;

use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    // tail: Option<*mut Node<T>>,
    tail: Option<NonNull<Node<T>>>,
}

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    data: T,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    // current: Option<*mut Node<T>>,
    current: Option<NonNull<Node<T>>>,

}

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            count += 1;
            curr = node.next.as_ref();
        }
        count
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let head_ptr = self.head.as_mut().map(|node| NonNull::from(node.as_mut()));
        Cursor {
            current: head_ptr,
            list: self,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.tail,
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { current: self.head.as_deref() }
    }
}

impl<'a, T> Cursor<'a, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let mut ptr = self.current?;
        unsafe { Some(&mut (*ptr.as_mut()).data) }
    }

    // 向后移动 (Towards Tail)
    pub fn next(&mut self) -> Option<&mut T> {
        let mut ptr = self.current?;
        unsafe {
            let next_ptr = (*ptr.as_mut()).next.as_mut().map(|n| NonNull::from(n.as_mut()));
            self.current = next_ptr;
            self.peek_mut()
        }
    }

    // 向前移动 (Towards Head)
    pub fn prev(&mut self) -> Option<&mut T> {
        let mut ptr = self.current?;
        unsafe {
            let prev_ptr = (*ptr.as_mut()).prev;
            self.current = prev_ptr;
            self.peek_mut()
        }
    }

    pub fn take(&mut self) -> Option<T> {
        let mut ptr = self.current?;
        
        unsafe { // 需要同时获取多个可变引用 所以一定要用 unsafe, ptr + next_node + prev_node
            // 1. 确定下一个要指向的节点 (优先向后，否则向前)
            let next_node = (*ptr.as_mut()).next.as_mut().map(|n| NonNull::from(n.as_mut()));
            let prev_node = (*ptr.as_mut()).prev;

            // 2. 从链表中取出 Box<Node> 以获得所有权
            let mut node_box = if let Some(mut p) = prev_node {
                (*p.as_mut()).next.take().unwrap()
            } else {
                self.list.head.take().unwrap()
            };

            // 3. 重新链接
            let next_box = node_box.next.take();
            if let Some(mut next) = next_box {
                next.prev = prev_node;
                if let Some(mut p) = prev_node {
                    (*p.as_mut()).next = Some(next);
                } else {
                    self.list.head = Some(next);
                }
            } else {
                // 没有下一个，更新 tail
                self.list.tail = prev_node;
                if let Some(mut p) = prev_node {
                    (*p.as_mut()).next = None;
                }
            }

            // 更新 Cursor 位置
            self.current = next_node.or(prev_node);
            
            Some(node_box.data)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        if let Some(mut curr_ptr) = self.current {
            unsafe {
                let mut new_node = Box::new(Node {
                    data: element,
                    next: (*curr_ptr.as_mut()).next.take(),
                    prev: Some(curr_ptr),
                });
                let new_ptr = NonNull::from(new_node.as_mut());
                
                if let Some(next_node) = &mut new_node.next {
                    next_node.prev = Some(new_ptr);
                } else {
                    self.list.tail = Some(new_ptr);
                }
                (*curr_ptr.as_mut()).next = Some(new_node);
            }
        } else {
            // 如果列表为空，直接作为 head
            let mut new_node = Box::new(Node { data: element, next: None, prev: None });
            self.list.tail = Some(NonNull::from(new_node.as_mut()));
            self.list.head = Some(new_node);
            self.current = self.list.tail;
        }
    }

    pub fn insert_before(&mut self, element: T) {
        if let Some(mut curr_ptr) =  self.current {
            unsafe {
                let curr_node = if let Some(mut prev) = (*curr_ptr.as_mut()).prev {
                    (*prev.as_mut()).next.take()
                } else {
                    self.list.head.take()
                };

                let mut new_node = Box::new(Node{
                    data: element,
                    prev: (*curr_ptr.as_mut()).prev,
                    next: curr_node,
                });
                new_node.next.as_mut().unwrap().prev = Some(NonNull::from(new_node.as_mut()));
                if let Some(mut prev) =  new_node.as_mut().prev {
                    (*prev.as_mut()).next = Some(new_node);
                } else {
                    self.list.head = Some(new_node);
                }
            }
        } else {
            // 如果列表为空，直接作为 head
            let mut new_node = Box::new(Node { data: element, next: None, prev: None });
            self.list.tail = Some(NonNull::from(new_node.as_mut()));
            self.list.head = Some(new_node);
            self.current = self.list.tail;
        }
    }

}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = self.current?;
        self.current = node.next.as_deref();
        Some(&node.data)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr = self.tail;
        while let Some(mut curr_ptr) = curr{
            unsafe {
                let _ = if let Some(mut prev) = (*curr_ptr.as_mut()).prev {
                        curr = Some(prev);
                        (*prev.as_mut()).next.take()
                    } else {
                        curr = None;
                        self.head.take()
                };
            }
        }
        
    }
}


// 只要 T 可以在线程间转移，LinkedList<T> 就可以在线程间转移
unsafe impl<T: Send> Send for LinkedList<T> {}

// 只要 T 可以在线程间共享，LinkedList<T> 就可以在线程间共享
unsafe impl<T: Sync> Sync for LinkedList<T> {}
