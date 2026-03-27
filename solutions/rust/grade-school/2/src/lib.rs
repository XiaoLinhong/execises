use std::collections::{HashMap, HashSet};

pub struct School<'a> {
    roster: HashMap<&'a str, u32>
}

impl<'a> School<'a> {
    pub fn new() -> Self {
        Self{
            roster: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        if !self.roster.contains_key(student) {
            self.roster.insert(student, grade);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut list = self.roster.values().cloned().collect::<Vec<u32>>();
        list.sort();
        list.dedup();
        list
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut list = self.roster.iter()
                .filter(|&(_, &v)| v == grade)
                .map(|(&k, _)| k.to_string())
                .collect::<Vec<String>>();
    
        list.sort();
        list
    }
}
