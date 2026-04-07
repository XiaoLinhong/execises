use std::collections::{HashMap, HashSet};

fn prise(books: &[u32]) -> u32 {
    books.iter().map(|n|{
        let discount = match n {
            2 => 5,
            3 => 10,
            4 => 20,
            5 => 25,
            _ => 0,
        };
        *n  * (100 - discount) * 8
    }).sum()

}

pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }
    let mut counts = HashMap::new();
    for v in books {
        *counts.entry(*v).or_insert(0) += 1;
    }
    let mut nbooks = counts.values().cloned().collect::<Vec<u32>>();
    nbooks.sort(); // 1,3,3,5
    let mut strategy: Vec<u32> = vec![]; // 最自然的组合方式
    for _ in (1..=*nbooks.last().unwrap()).rev() {
        strategy.push(nbooks.iter().filter(|&&v| v>0_u32).count() as u32);
        nbooks = nbooks.into_iter().filter(|&v| v>0).map(|v|v - 1).collect();
    }
    strategy.sort();

    // let mut strategys = vec![];
    let mut strategys = HashSet::new();
    let left = vec![];
    find(&mut strategys, &strategy, &left);

    println!("{:?}", strategys);

    // strategys.iter().map(|s| dbg!(prise(s))).min().unwrap()
    strategys.iter().map(|s| prise(s)).min().unwrap()
}

fn find(strategys: &mut HashSet<Vec<u32>>, strategy: &[u32], left: &[u32]) {
    // strategys.push([left, strategy].concat());
    let mut result = [left, strategy].concat();
    result.sort();
    strategys.insert(result);
    // if !strategys.insert(result) {
    //     return;
    // }
    if strategy.len() < 2 {
        return;
    }
    if strategy.iter().max().unwrap() - strategy.iter().min().unwrap() > 1 {
        for i in 1..strategy.len() {
            if strategy[i] - strategy[0] > 1 { // 有重复
                let mut new_strategy = strategy.to_vec();
                new_strategy[0] += 1;
                new_strategy[i] -= 1;
                new_strategy.sort();
                find(strategys, &new_strategy, left);
            }
        }
        find(strategys, &strategy[1..], &[left, &strategy[..1]].concat());
    }
}

