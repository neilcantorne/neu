
#[derive(PartialEq, Eq)]
pub enum BackendApi {
    Cuda,
    OpenCl,
    All,
}

impl std::fmt::Debug for BackendApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BackendApi::Cuda => "Nvidia CUDA",
            BackendApi::OpenCl => "OpenCL",
            BackendApi::All => "All",
        })
    }
}

impl std::fmt::Display for BackendApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BackendApi::Cuda => "Nvidia CUDA",
            BackendApi::OpenCl => "OpenCL",
            BackendApi::All => "All",
        })
    }
}