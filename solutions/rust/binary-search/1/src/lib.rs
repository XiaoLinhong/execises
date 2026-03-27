use std::fmt::{Debug};


// pub fn find<T: PartialEq + Ord + Copy + Debug>(array: &[T], key: T) -> Option<usize> 
pub fn find<S, T>(array: S, key: T) -> Option<usize> 
where 
   S: AsRef<[T]>,
   T: PartialEq + Ord + Copy + Debug,
{
    let array = array.as_ref();
    if array.is_empty() {return None;}

    let middle = array.len()/2;
    // println!("{:?}, {}", array, middle);

    match key.cmp(&array[middle]) {
        std::cmp::Ordering::Equal => Some(middle),
        std::cmp::Ordering::Greater => find(&array[middle+1..], key).map(|i| middle + 1 + i),
        std::cmp::Ordering::Less => find(&array[..middle], key),
    }
}
