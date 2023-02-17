pub struct Error(ErrorVariants);
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) enum ErrorVariants {
    InvalidTensorLayout,
    TensorAllocationFailed,
    ElementAllocationFailed,
    DifferentOperandTypes,
    DifferentOperandDimension,
    InvalidOperandTypes,
    IncompatibleOperandDimensions,
    TensorNonUniformChannel,
}

impl<T> From<ErrorVariants> for Result<T> {
    fn from(val: ErrorVariants) -> Self {
        Err(Error(val))
    }
}

impl From<ErrorVariants> for Error {
    fn from(val: ErrorVariants) -> Self {
        Error(val)
    }
}