#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Class {
    OPEN,
    SPARE,
    STRIKE,
}

#[derive(Debug)]
struct Frame {
    pins: Vec<u16>,
    class: Option<Class>,
    completed: bool,
}

impl Frame {
    fn new() -> Self {
        Frame{
            pins: vec![],
            class: None,
            completed: false,
        }
    }
 
    fn set_class(&mut self, is_last: bool) {
        if self.completed {
            return;
        }

        match self.pins.len() {
            0 => return,
            1  => {
                if self.pins[0] == 10 {
                    if !is_last {self.completed = true};
                    self.class = Some(Class::STRIKE);
                }
            },
            2 => {
                match self.pins.iter().sum::<u16>() {
                    0..10 => {
                        self.completed = true;
                        self.class = Some(Class::OPEN);  
                    }

                    10 if self.pins[1] > 0 => {
                        if !is_last {self.completed = true};
                        self.class = Some(Class::SPARE);
                    }
                    _ => {} // last 10, 0
                    
                }
            },
            3 => self.completed = true, // last frame
            _ => unreachable!()
        }
}
}

pub struct BowlingGame {
    frames: Vec<Frame>,
} // 30 * 10

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame{
            frames: vec![],
        }
    }

    fn need_next_frame(&self) -> bool {
        if self.frames.is_empty() || self.frames.last().unwrap().completed{
            return true;
        } 
        false
    }
    
    fn completed(&self) -> bool {
        self.frames.len() == 10 && self.frames.last().unwrap().completed
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.completed() {
            return Err(Error::GameComplete);
        }
        if pins > 10_u16 {
            return Err(Error::NotEnoughPinsLeft);
        }
        // update frame
        if self.need_next_frame() {
            let mut frame: Frame = Frame::new();
            frame.pins.push(pins);
            self.frames.push(frame);
        } else {
            let frame = self.frames.last().unwrap();
            if self.frames.len() == 10 { // lase
                if frame.pins.len() == 1 && (frame.pins[0] != 10 && frame.pins[0] + pins > 10_u16) { // Open
                    return Err(Error::NotEnoughPinsLeft);
                }
                if frame.pins.len() == 2  { // 
                    if frame.pins[0] == 10 && (frame.pins[1] != 10 && frame.pins[1] + pins > 10_u16) {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
            } else {
                if frame.pins[0] + pins > 10_u16 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
            self.frames.last_mut().unwrap().pins.push(pins);
            
        }
        let is_last = self.frames.len() == 10;
        self.frames.last_mut().unwrap().set_class(is_last);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        println!("{:?}", self.frames);

        if !self.completed() {
            return None;
        };

        let mut s = 0;
        let mut next_two_throws = (0, 0);
        for frame in self.frames.iter().rev() {

            s += match frame.class {
                Some(Class::OPEN) => frame.pins.iter().sum::<u16>(),
                Some(Class::SPARE) => frame.pins.iter().sum::<u16>() + next_two_throws.0,
                Some(Class::STRIKE) => frame.pins.iter().sum::<u16>() + next_two_throws.0 + next_two_throws.1,
                None => return None
            };

            if frame.pins.len() == 1 {
                next_two_throws = (frame.pins[0], next_two_throws.0)
            } else {
                next_two_throws = (frame.pins[0], frame.pins[1])
            }
        }

        Some(s)

    }
}
