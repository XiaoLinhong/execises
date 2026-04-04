use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone)]
pub struct Decimal {
    data: HashMap<i32, i8>,
    positive: i8,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let numbers: Vec<&str> = input.split('.').collect();

        let (part1, positive)  = match numbers[0].starts_with('-') {
            true => (numbers[0].replace('-', ""), -1),
            false => (numbers[0].replace('+', ""), 1)            
        };

        let part2 = if numbers.len() == 2 {
            numbers[1].to_string()
        } else {
            "".to_string()
        };

        let mut data: HashMap<i32, i8> = HashMap::new(); 

        for (i, c) in part1.chars().rev().enumerate() {
            data.insert(i as i32, c.to_digit(10).unwrap() as i8);
        }
        
        for (i, c) in part2.chars().enumerate() {
            data.insert(-(i as i32 + 1), c.to_digit(10).unwrap() as i8);
        }

        // dbg!(Some(Decimal {data, positive}))
       Some(Decimal {data, positive}) 
    }
}

impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Self) -> Self {
        let max = self.data.keys().max().unwrap().max(other.data.keys().max().unwrap()).clone();
        let min = self.data.keys().min().unwrap().min(other.data.keys().min().unwrap()).clone();

        let mut data: HashMap<i32, i8> = HashMap::new(); 

        let mut up: i8 = 0;

        for k in min..=max {
            let n = self.data.get(&k).map_or(0, |&i| i * self.positive) + other.data.get(&k).map_or(0, |&i| i * other.positive) + up;
            up = (n as i8).div_euclid(10);
            data.insert(k, n-up*10);
            // println!("{k}, {n}, {up}, {}", n-up*10);

        }

        let positive = if up < 0 {-1} else {1};

        if up > 0 {
            data.insert(max+1, up);
        }
        if up < 0 {
            let mut first = true;
            // println!("{:?}", data);
            for k in min..=max {
                if data.contains_key(&k) {
                    let v = data.get(&k).unwrap();
                    if (first && *v > 0) || !first{
                        if first{
                            first = false;
                            data.insert(k, 10 - v);
                        } else {
                            data.insert(k, 9 - v);
                        }
                    } 
                    // println!("{k} {:?}" , data.get(&k))
                }
            }
        }

        Self{data, positive}
    }
}

impl Sub for Decimal {
    type Output = Decimal;
        fn sub(self, other: Self) -> Self {
            let other = Self { data: other.data.clone(), positive: -1 * other.positive};
            self.add(other)
        }
}

use std::cmp::*;

impl PartialEq for Decimal {

    fn eq(&self, other: &Self) -> bool {
        if self.positive != other.positive {
            return false;
        }
        let max = self.data.keys().max().unwrap().max(other.data.keys().max().unwrap()).clone();
        let min = self.data.keys().min().unwrap().min(other.data.keys().min().unwrap()).clone();
        
        for k in min..=max {
            if self.data.get(&k).or(Some(&0)) != other.data.get(&k).or(Some(&0)) {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for Decimal {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal)
        }

        let n: Decimal = self.clone().sub(other.clone());

        if n.positive > 0 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
        
    }
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, rhs: Self) -> Self::Output {
        let positive = self.positive*rhs.positive;
        let mut data: HashMap<i32, i8> = HashMap::new(); 

        for (k1, v1) in &self.data {
            for (k2, v2) in &rhs.data {
                let n = v1*v2;
                let up = (n as i8).div_euclid(10);
                *data.entry(k1+k2).or_insert(0) += n - up*10;
                if up > 0 {
                    *data.entry(k1+k2+1).or_insert(0) += up;
                }
            }
        }
        Self{data, positive}
    }
}

