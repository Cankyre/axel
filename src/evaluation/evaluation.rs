use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::Neg,
};

#[derive(PartialEq, Clone, Copy)]
pub enum Evaluation {
    Centipawns(i32),
    Mate(i32),
    Max,
    Min,
}

impl Evaluation {
    pub fn from_deeper(value: &Self) -> Self {
        match value {
            Evaluation::Mate(0) => Evaluation::Mate(1),
            Evaluation::Mate(v) if *v > 0 => Evaluation::Mate(-*v - 1),
            Evaluation::Mate(v) => Evaluation::Mate(-*v + 1),
            Evaluation::Centipawns(v) => Evaluation::Centipawns(-v),
            Evaluation::Max => Evaluation::Min,
            Evaluation::Min => Evaluation::Max,
        }
    }

    pub fn to_deeper(self) -> Self {
        match self {
            Evaluation::Mate(v) if v > 0 => Evaluation::Mate(-v + 1),
            Evaluation::Mate(v) if v < 0 => Evaluation::Mate(-v - 1),
            Evaluation::Centipawns(v) => Evaluation::Centipawns(-v),
            Evaluation::Max => Evaluation::Min,
            Evaluation::Min => Evaluation::Max,
            _ => self,
        }
    }
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Evaluation::*;

        let to_tuple = |eval: &Evaluation| match eval {
            Centipawns(v) => (*v, 0),
            Mate(v) if *v > 0 => (i32::MAX - 1, *v),
            Mate(v) => (i32::MIN + 1, *v),
            Max => (i32::MAX, 0),
            Min => (i32::MIN, 0),
        };

        let self_tuple = to_tuple(self);
        let other_tuple = to_tuple(other);

        Some(self_tuple.cmp(&other_tuple))
    }
}

impl Neg for Evaluation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Centipawns(v) => Self::Centipawns(-v),
            Self::Mate(v) => Self::Mate(-v),
            Self::Max => Self::Min,
            Self::Min => Self::Max,
        }
    }
}

impl Debug for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Centipawns(v) => write!(f, "Exact({})", v),
            Self::Mate(v) => write!(f, "Mate({})", v),
            Self::Max => write!(f, "Max"),
            Self::Min => write!(f, "Min"),
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Evaluation::Centipawns(v) => write!(f, "cp {}", v),
            Evaluation::Mate(v) => write!(
                f,
                "mate {}",
                match v {
                    0 => 0,
                    v => v.signum() * (v.abs() + 1) / 2,
                }
            ),
            Evaluation::Max => write!(f, "cp {}", i32::MAX),
            Evaluation::Min => write!(f, "cp {}", i32::MIN),
        }
    }
}
