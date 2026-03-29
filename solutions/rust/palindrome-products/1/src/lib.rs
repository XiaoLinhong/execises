use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    v: u64,
    r1: u64,
    r2: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.v
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        // println!("{}", (self.v as f32).powf(0.5) as u64 );
        (1..= (self.v as f32).powf(0.5) as u64)
        .filter(|&i| dbg!(self.v % i == 0 && i >= self.r1 && i <= self.r2 && self.v/i  >= self.r1 && self.v/i  <= self.r2)) 
        .map(|i| (i, self.v/i ))
        .collect()
    }
}

fn is_palindrome(v: u64) -> bool {
    // println!("{}", v);
    v == v.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: Vec<Palindrome> = Vec::new();

    let mut boundary = 0;
    'outer: for i in min..=max {
        for j in min..=max{
            let v = i*j;
            if is_palindrome(v) {
                palindromes.push(Palindrome { v, r1: min, r2: max });
                if boundary == 0 {
                    boundary = j;
                }
                if i > boundary {break 'outer;}
            }
        }
    }

    let mut boundary = 0;
    'outer: for i in (min..=max).rev() {
        for j in (min..=max).rev(){
            let v: u64 = i*j;
            // println!("{}, {}", i, j);
            if is_palindrome(v) {
                palindromes.push(Palindrome { v, r1: min, r2: max });
                if boundary == 0 {
                    boundary = j;
                }
                if i < boundary {break 'outer;}
            }
        }
    }

    palindromes.sort_by(|a, b| a.v.cmp(&b.v));

    println!("{:?}", palindromes);    
    match palindromes.len() {
            0 => return None,
            1 => return Some( (palindromes[0].clone(), palindromes[0].clone()) ),
            _ => return Some( (palindromes[0].clone(), palindromes.last().unwrap().clone())),
        }

}
