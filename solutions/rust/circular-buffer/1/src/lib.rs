use std::{collections::VecDeque, fmt::Debug};

pub struct CircularBuffer<T: Debug> {
    len: usize,
    cap: usize,
    buffer: VecDeque<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

use Error::*;
impl<T: Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self { len: 0, cap: capacity, buffer: VecDeque::with_capacity(capacity)}
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.len == self.cap {
            return Err(FullBuffer);
        }
        self.buffer.push_front(element);
        self.len += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.buffer.pop_back() {
            Some(data) => {
                self.len -= 1;
                Ok(data)
            }
            _ => Err(EmptyBuffer),
        }
    }

    pub fn clear(&mut self) {
        for _ in 0..self.len {
            self.buffer.pop_back();
        }
        // self.buffer = VecDeque::with_capacity(self.cap);
        self.len = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        match self.len.cmp(&self.cap) {
            std::cmp::Ordering::Less => {
                self.buffer.push_front(element);
                self.len += 1;
            },
            _ => {
                self.buffer.pop_back();
                self.buffer.push_front(element);
            }
        }
    }
}
