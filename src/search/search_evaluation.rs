use std::{cmp::Ordering, fmt::Debug, ops::Neg};

#[derive(PartialEq, Clone)]
pub enum Evaluation {
    Exact(i32),
    Mate(i32),
    LowerBound(i32),
    UpperBound(i32),
    Max,
    Min,
}

impl Evaluation {
    pub fn from_deeper(value: &Self) -> Self {
        match value {
            Evaluation::Mate(v) => {
                let s = match v.signum() {
                    0 => -1,
                    v => v,
                };
                Self::Mate(-s * (v.abs() + 1))
            }
            Evaluation::LowerBound(v) => Evaluation::UpperBound(-v),
            Evaluation::UpperBound(v) => Evaluation::LowerBound(-v),
            Evaluation::Exact(v) => Evaluation::Exact(-v),
            Evaluation::Max => Evaluation::Min,
            Evaluation::Min => Evaluation::Max,
        }
    }

    fn unpack(&self) -> Option<i32> {
        match self {
            Self::Exact(v) | Self::LowerBound(v) | Self::UpperBound(v) => Some(v.to_owned()),
            _ => None,
        }
    }
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Self::Max => Some(Ordering::Greater),
            Self::Min => Some(Ordering::Less),
            Self::Mate(0) => Some(Ordering::Less),
            Self::Mate(v) => match other {
                Self::Mate(v2) => {
                    if v < &0 && v2 < &0 {
                        v.partial_cmp(v2)
                    } else if v >= &0 && v2 >= &0 {
                        v2.partial_cmp(v)
                    } else {
                        Some(v.signum().cmp(&v2.signum()))
                    }
                }
                _ => {
                    if v <= &0 {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
            },
            _ => match other {
                Self::Max => Some(Ordering::Less),
                Self::Min => Some(Ordering::Greater),
                Self::Mate(0) => Some(Ordering::Greater),
                Self::Mate(v) => {
                    if v < &0 {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Less)
                    }
                }
                _ => self.unpack().unwrap().partial_cmp(&other.unpack().unwrap()),
            },
        }
    }
}

impl Neg for Evaluation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Exact(v) => Self::Exact(-v),
            Self::Mate(v) => Self::Mate(-v),
            Self::LowerBound(v) => Self::UpperBound(v),
            Self::UpperBound(v) => Self::LowerBound(v),
            Self::Max => Self::Min,
            Self::Min => Self::Max,
        }
    }
}

impl Debug for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exact(v) => write!(f, "Exact({})", v),
            Self::Mate(v) => write!(f, "Mate({})", v),
            Self::LowerBound(v) => write!(f, "LowerBound({})", v),
            Self::UpperBound(v) => write!(f, "UpperBound({})", v),
            Self::Max => write!(f, "Max"),
            Self::Min => write!(f, "Min"),
        }
    }
}
