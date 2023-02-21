pub struct Error(ErrorVariants);
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) enum ErrorVariants {
    FailedToInitializeCuda,
    FailedToRetrieveCudaDevice,
    FailedToRetrieveOpenClDevices,
    InvalidTensorLayout,
    TensorAllocationFailed,
    ElementAllocationFailed,
    DifferentOperandTypes,
    DifferentOperandDimensions,
    InvalidOperandTypes,
    IncompatibleOperandDimensions,
    IncompatibleOperandTypes,
    TensorNonUniformChannel,
    RequiresTensor,
    UnableToConvergeOperand,
    UnableToConvolve,
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

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.0 {
            ErrorVariants::FailedToInitializeCuda => "Failed to initialize CUDA backend",
            ErrorVariants::FailedToRetrieveCudaDevice => "Failed to retrieve CUDA device",
            ErrorVariants::FailedToRetrieveOpenClDevices => "Failed to retrieve OpenCL devices",
            ErrorVariants::InvalidTensorLayout => "Invalid tensor layout",
            ErrorVariants::TensorAllocationFailed => "Tensor allocation failed",
            ErrorVariants::ElementAllocationFailed => "Element allocation failed",
            ErrorVariants::DifferentOperandTypes => "Different operand types",
            ErrorVariants::DifferentOperandDimensions => "Different operand dimensions",
            ErrorVariants::InvalidOperandTypes => "Invalid operand types",
            ErrorVariants::IncompatibleOperandDimensions => "Incompatible operand dimensions",
            ErrorVariants::IncompatibleOperandTypes => "Incompatible operand types",
            ErrorVariants::TensorNonUniformChannel => "Tensor non-uniform channel",
            ErrorVariants::RequiresTensor => "Requires Tensor",
            ErrorVariants::UnableToConvergeOperand => "Unable to join operand",
            ErrorVariants::UnableToConvolve => "Unable to convolve operand",
        })
    }
}