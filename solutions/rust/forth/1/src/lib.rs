use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

use Error::*;

pub struct Forth {
    stack: Vec<Value>,
    definitions: HashMap<String, Vec<String>>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let tokens: Vec<String> = input
            .split_whitespace()
            .map(|token| token.to_lowercase())
            .collect();

        let mut index = 0;
        while index < tokens.len() {
            let token = &tokens[index];
            if token == ":" {
                index += 1;
                self.parse_definition(&tokens, &mut index)?;
            } else {
                // println!("{:?}", self.definitions);
                // println!("{:?}", token);
                self.execute_token(token)?;
                index += 1;
            }
        }

        Ok(())
    }

    fn parse_definition(&mut self, tokens: &[String], index: &mut usize) -> Result {
        let name = tokens.get(*index).ok_or(InvalidWord)?;
        *index += 1;

        if name.parse::<Value>().is_ok() {
            return Err(InvalidWord);
        }

        let mut body = Vec::new();
        while *index < tokens.len() {
            let token = &tokens[*index];
            *index += 1;
            if token == ";" {
                // let expansion = self.expand_definition(&body, name)?;
                let expansion = self.expand_definition(&body, name)?;
                
                // 重新定义某个变量的时候，有可能这个变量已经赋值给其他值了
                if let Some(previous) = self.definitions.get(name).cloned() { // 可不可以不用引用
                    for (key, tokens ) in &mut self.definitions {
                        if key != name { // 
                            if tokens.contains(name){
                                let idx = tokens.iter().position(|token|token == name).unwrap();
                                tokens.splice(idx..=idx, previous.clone());
                            }
                        }
                    }
                }

                self.definitions.insert(name.clone(), expansion);
                return Ok(());
            }
            body.push(token.clone());
        }

        Err(InvalidWord)
    }

    fn expand_definition(&self, body: &[String], name: &str) -> std::result::Result<Vec<String>, Error> {
        let mut expansion = Vec::new();

        for token in body { // 覆盖自己
            if token == name {
                if let Some(previous) = self.definitions.get(token) {
                    expansion.extend(previous.iter().cloned());
                    continue;
                }
            }

            if token.parse::<Value>().is_ok() { // 变量
                expansion.push(token.clone());
                continue;
            }

            if self.definitions.contains_key(token) { // 已经定义的变量
                expansion.push(token.clone());
                // expansion.extend(self.definitions.get(token).unwrap().iter().cloned());
                continue;
            }

            if self.is_builtin(token) { // 内部操作，最终都要转换为内部操作
                expansion.push(token.clone());
                continue;
            }

            return Err(UnknownWord);
        }

        Ok(expansion)
    }

    fn execute_token(&mut self, token: &str) -> Result { // 所以有的都是操作

        if let Ok(number) = token.parse::<Value>() {
            self.stack.push(number);
            return Ok(());
        }

        if let Some(definition) = self.definitions.get(token) {
            let definition: Vec<String> = definition.clone(); // 可不可以用更优的处理方式
            for token in definition {
                self.execute_token(&token)?;
            }
            return Ok(());
        }

        match token {
            "+" => self.binary_op(|a, b| a + b),
            "-" => self.binary_op(|a, b| a - b),
            "*" => self.binary_op(|a, b| a * b),
            "/" => self.divide(),
            "dup" => self.dup(),
            "drop" => self.drop(),
            "swap" => self.swap(),
            "over" => self.over(),
            _ => Err(UnknownWord),
        }
    }

    fn is_builtin(&self, token: &str) -> bool {
        matches!(token, "+" | "-" | "*" | "/" | "dup" | "drop" | "swap" | "over")
    }

    fn binary_op<F>(&mut self, op: F) -> Result
    where
        F: Fn(Value, Value) -> Value,
    {
        if self.stack.len() < 2 {
            return Err(StackUnderflow);
        }

        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(op(a, b));
        Ok(())
    }

    fn divide(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(StackUnderflow);
        }

        let b = self.stack.pop().unwrap();
        if b == 0 {
            return Err(DivisionByZero);
        }

        let a = self.stack.pop().unwrap();
        self.stack.push(a / b);
        Ok(())
    }

    fn dup(&mut self) -> Result {
        if let Some(&value) = self.stack.last() {
            self.stack.push(value);
            Ok(())
        } else {
            Err(StackUnderflow)
        }
    }

    fn drop(&mut self) -> Result {
        self.stack.pop().map(|_| ()).ok_or(StackUnderflow)
    }

    fn swap(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(StackUnderflow);
        }

        let len = self.stack.len();
        self.stack.swap(len - 1, len - 2);
        Ok(())
    }

    fn over(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(StackUnderflow);
        }

        let len = self.stack.len();
        let value = self.stack[len - 2];
        self.stack.push(value);
        Ok(())
    }
}
