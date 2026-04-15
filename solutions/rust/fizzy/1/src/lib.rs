
use std::ops::Rem;

pub struct Matcher<T>{
    func: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: ToString>(matcher: F, subs: S) -> Matcher<T> {
        Self { func: Box::new(matcher), subs: subs.to_string() }
    } 
}

pub struct Fizzy<T>{
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Clone> Fizzy<T> {
    pub fn new() -> Self {
        Self { matchers: Vec::new()}
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item=T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map( move |v| {
            let mut s = "".to_string();
            for matcher in &self.matchers {
                if matcher.func.as_ref() (v.clone()) {
                    s.push_str(&matcher.subs)
                }
            }
            if !s.is_empty() {
                return s;
            }
            v.to_string()
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Rem<Output =T> + From<u8> + PartialEq + Clone + ToString>() -> Fizzy<T> {
    Fizzy::new().add_matcher(Matcher::new(|n| n % 3.into()  == 0.into(), "fizz"))
                .add_matcher(Matcher::new(|n| n % 5.into()  == 0.into(), "buzz"))
}
