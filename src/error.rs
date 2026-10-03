#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct CapacityLimitError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WriteAsciiError {
    InvalidAscii,
    Capacity(CapacityLimitError),
}

impl From<CapacityLimitError> for WriteAsciiError {
    fn from(value: CapacityLimitError) -> Self {
        Self::Capacity(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InsertAsciiError {
    IndexOutOfBounds,
    Capacity(CapacityLimitError),
}

impl From<CapacityLimitError> for InsertAsciiError {
    fn from(value: CapacityLimitError) -> Self {
        Self::Capacity(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InsertRangeError {
    RangeOutOfBounds,
    Capacity(CapacityLimitError),
}

impl From<CapacityLimitError> for InsertRangeError {
    fn from(value: CapacityLimitError) -> Self {
        Self::Capacity(value)
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct RemoveRangeError;
