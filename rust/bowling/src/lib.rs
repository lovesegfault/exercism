#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum Frame {
    NoRoll,
    Incomplete(u16),
    Open(u16, u16),
    Spare(u16, u16),
    Strike,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::with_capacity(12),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_finished() {
            return Err(Error::GameComplete);
        }

        if let Some(frame) = self.frames.last_mut() {
            if let Frame::Incomplete(ref previous) = frame {
                match previous + pins {
                    10 => *frame = Frame::Spare(*previous, pins),
                    0..=9 => *frame = Frame::Open(*previous, pins),
                    _ => return Err(Error::NotEnoughPinsLeft),
                }
                return Ok(());
            }
        }

        match pins {
            10 => self.frames.push(Frame::Strike),
            0..=9 => self.frames.push(Frame::Incomplete(pins)),
            _ => return Err(Error::NotEnoughPinsLeft),
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_finished() {
            return None;
        }

        use Frame::*;
        let score = self
            .frames
            .iter()
            .chain(std::iter::repeat(&Frame::NoRoll))
            .take(12)
            .collect::<Vec<_>>()
            .windows(3)
            .map(|w| match (w[0], w[1], w[2]) {
                (Open(a, b), _, _) => a + b,
                (Spare(a, b), Open(c, _), _) => a + b + c,
                (Spare(_, _), Spare(c, _), _) => 10 + c,
                (Spare(_, _), Strike, _) => 20,
                (Spare(_, _), Incomplete(a), _) => 10 + a,
                (Strike, Strike, Strike) => 30,
                (Strike, Strike, Open(a, _)) => 20 + a,
                (Strike, Strike, Spare(a, _)) => 20 + a,
                (Strike, Spare(_, _), _) => 20,
                (Strike, Open(a, b), _) => 10 + a + b,
                _ => 0,
            })
            .sum();

        Some(score)
    }

    fn is_finished(&self) -> bool {
        use Frame::*;
        matches!(
            (self.frames.get(9), self.frames.get(10), self.frames.get(11)),
            (Some(Open(_, _)), None, None)
                | (Some(Strike), Some(Strike), Some(Strike),)
                | (Some(Strike), Some(Open(_, _)), None)
                | (Some(Strike), Some(Spare(_, _)), None)
                | (Some(Spare(_, _)), Some(Incomplete(_)), None)
                | (Some(Spare(_, _)), Some(Strike), None)
        )
    }
}
