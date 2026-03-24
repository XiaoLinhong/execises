#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// fn check_equal(a: &[i32], b: &[i32]) -> bool {
//     a == b
// }

// fn check_contains(long: &[i32], short: &[i32]) -> bool {
//     let short_len = short.len();
//     if short_len == 0{
//         return true
//     }
//     let longer_len = long.len() - short_len;
//     for i in 0..=longer_len {
//         let chunck = &long[i..i+short_len];
//         if check_equal(chunck, short){
//             return true;
//         }
//     }
//     false
// }

// pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
//     let fisrt_num = first_list.len();
//     let second_num = second_list.len();

//     if fisrt_num > second_num {
//         if check_contains(first_list, second_list) {
//             return Comparison::Superlist;
//         }else {
//             return Comparison::Unequal;
//         }

//     } else if fisrt_num == second_num {
//         if check_equal(first_list, second_list) {
//             return Comparison::Equal;
//         }else {
//             return Comparison::Unequal;
//         }
//     } else {
//         if check_contains(second_list, first_list) {
//             return Comparison::Sublist;
//         }else {
//             return Comparison::Unequal;
//         }
//     }
// }


 fn is_sublist(long: &[i32], short: &[i32]) -> bool {
    if short.is_empty(){
        return true;
    }
    long.windows(short.len()).any(|w|w == short)
 }

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;
    match (first_list.len(), second_list.len()) {
        (m,n) if m == n && first_list == second_list => Equal,
        (m,n) if m > n  && is_sublist(first_list, second_list) => Superlist,
        (m,n) if m < n  && is_sublist(second_list, first_list) => Sublist,
        _ => Unequal
    }
}
