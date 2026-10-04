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

impl core::fmt::Display for CapacityLimitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("capacity limit exceeded")
    }
}

impl core::fmt::Display for WriteAsciiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidAscii => f.write_str("invalid ASCII value"),
            Self::Capacity(_) => f.write_str("capacity limit exceeded"),
        }
    }
}

impl core::fmt::Display for InsertAsciiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IndexOutOfBounds => f.write_str("index out of bounds"),
            Self::Capacity(_) => f.write_str("capacity limit exceeded"),
        }
    }
}

impl core::fmt::Display for InsertRangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RangeOutOfBounds => f.write_str("range out of bounds"),
            Self::Capacity(_) => f.write_str("capacity limit exceeded"),
        }
    }
}

impl core::fmt::Display for RemoveRangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("remove range error")
    }
}

impl core::error::Error for CapacityLimitError {}

impl core::error::Error for WriteAsciiError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::InvalidAscii => None,
            Self::Capacity(err) => Some(err),
        }
    }
}

impl core::error::Error for InsertAsciiError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::IndexOutOfBounds => None,
            Self::Capacity(err) => Some(err),
        }
    }
}

impl core::error::Error for InsertRangeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::RangeOutOfBounds => None,
            Self::Capacity(err) => Some(err),
        }
    }
}

impl core::error::Error for RemoveRangeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_limit_error_display() {
        let err = CapacityLimitError;
        assert_eq!(format!("{err}"), "capacity limit exceeded");
    }

    #[test]
    fn write_ascii_error_display() {
        let err = WriteAsciiError::InvalidAscii;
        assert_eq!(format!("{err}"), "invalid ASCII value");

        let err = WriteAsciiError::Capacity(CapacityLimitError);
        assert_eq!(format!("{err}"), "capacity limit exceeded");
    }

    #[test]
    fn insert_ascii_error_display() {
        let err = InsertAsciiError::IndexOutOfBounds;
        assert_eq!(format!("{err}"), "index out of bounds");

        let err = InsertAsciiError::Capacity(CapacityLimitError);
        assert_eq!(format!("{err}"), "capacity limit exceeded");
    }

    #[test]
    fn insert_range_error_display() {
        let err = InsertRangeError::RangeOutOfBounds;
        assert_eq!(format!("{err}"), "range out of bounds");

        let err = InsertRangeError::Capacity(CapacityLimitError);
        assert_eq!(format!("{err}"), "capacity limit exceeded");
    }

    #[test]
    fn remove_range_error_display() {
        let err = RemoveRangeError;
        assert_eq!(format!("{err}"), "remove range error");
    }
}
