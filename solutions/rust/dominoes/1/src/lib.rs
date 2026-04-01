use std::{collections::HashMap};

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut counts = HashMap::new();

    for (n1, n2) in input {
        *counts.entry(*n1).or_insert(0) += 1;
        *counts.entry(*n2).or_insert(0) += 1;
    }

    let numbers: Vec<u8> = counts.keys().cloned().collect();
    // println!("{:?}", counts);
    match numbers.len() {
        0 => Some(Vec::new()),
        1 => Some(input.iter().map(|(n1, n2)| (*n1, *n2)).collect()), 
        _ => {
            for (_, &v) in &counts { // 不成对出现
                if v % 2 != 0 {
                    return None;
                }
            }
            // 一定有解
            let mut answer = vec![];
            let mut list: Vec<(u8, u8)> = input.iter().cloned().collect::<Vec<(u8, u8)>>();
            let left = numbers[0];
            // *counts.get_mut(&right).unwrap() -= 1;
            if find(&mut list, &mut answer, left) {
                return Some(answer);
            }
            None
        }
    }
}


fn find(list: &mut Vec<(u8, u8)>, answer: &mut Vec<(u8, u8)>, left: u8) -> bool {
    if list.is_empty() { return true;}

    let mut left = left;
    let candidates: Vec<(u8, u8)> = list.iter().filter(|&&(n1, n2 )| n1 == left || n2 == left).copied().collect();

    if candidates.is_empty() {return false}

    for candidate in candidates {
        if candidate.0 == left {
            answer.push(candidate);
            left = candidate.1;
        } else {
            answer.push((candidate.1, candidate.0));
            left = candidate.0;
        }
        
        let idx = list.iter().position(|&x| x == candidate).unwrap();
        list.remove(idx); // 找到了当前 candidate
        if find(list, answer, left) {
            return true;
        } else { // 后面没办法继续 回退
            list.push(candidate);
            answer.pop();
        }

    }

    false

}



