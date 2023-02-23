pub struct Error(ErrorVariants);
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) enum ErrorVariants {
    UnableToLoadOpenCl,
    UnableToLoadCuda,
    FailedToInitializeCuda,
    FailedToRetrieveCudaDevice,
    FailedToRetrieveOpenClDevices,
    UnableToCreateCudaContext,
    UnableToCreateOpenClContext,
    UnableToGetCudaDeviceName,
    UnableToGetOpenCLDeviceName,
    InvalidNameFormat,
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
            ErrorVariants::UnableToLoadOpenCl => "Unable to load OpenCL",
            ErrorVariants::UnableToLoadCuda => "Unable to load CUDA",
            ErrorVariants::FailedToInitializeCuda => "Failed to initialize CUDA backend",
            ErrorVariants::FailedToRetrieveCudaDevice => "Failed to retrieve CUDA device",
            ErrorVariants::FailedToRetrieveOpenClDevices => "Failed to retrieve OpenCL devices",
            ErrorVariants::UnableToCreateCudaContext => "Unable to create CUDA context",
            ErrorVariants::UnableToCreateOpenClContext => "Unable to create OpenCL context",
            ErrorVariants::UnableToGetCudaDeviceName => "Unable to get CUDA device name",
            ErrorVariants::UnableToGetOpenCLDeviceName => "Unable to get OpenCL device name",
            ErrorVariants::InvalidNameFormat => "Invalid name format",
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