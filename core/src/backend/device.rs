use std::{
    sync::Arc,
    ffi::CStr
};

/// Represents a CUDA or OpenCL device.
/// This struct provides a simple way to retrieve information about a device
/// that can be used for hardware accelerated computation.
/// 
/// This can be constructed by using `Device::devices` that returns an iterator for enumerating available devices.
///
/// # Examples
///
/// ```
/// // Enumerate all devices available
/// for device in Device::devices(BackendApi::All).unwrap().flatten() {
///     if device.backend() == BackendApi::Cuda {
///         let cuda_name = device.name().unwrap();
///         assert_eq!(cuda_name, "GeForce RTX 2080 Ti");
///     } else {
///         let cl_name = device.name().unwrap();
///         assert_eq!(cl_name, "AMD Radeon RX 5700 XT");
///     }
/// }
/// ```
///
/// Note that the names of the devices in these examples are just placeholders, and will
/// depend on the actual devices installed on the system.
pub struct Device(pub(super) Arc<dyn DeviceInner>);

impl Device {

    /// Returns an iterator over all available CUDA or OpenCL devices.
    ///
    /// This function takes a `BackendApi` parameter indicating the desired backend API (either CUDA or OpenCL).
    /// If the function returns `Result::Ok`, an iterator is returned alongside it that can be used to enumerate all devices
    /// that are available for the specified backend API.
    ///
    /// # Errors
    ///
    /// Returns an `Result::Err(Error)` if the specified backend API is not available or if an error occurs while querying devices.
    ///
    /// # Examples
    ///
    /// ```
    /// // Enumerate all CUDA devices available
    /// for device in Device::devices(BackendApi::Cuda).unwrap().flatten() {
    ///     let name = device.name().unwrap();
    ///     println!("Found CUDA device: {}", name);
    /// }
    ///
    /// // Enumerate all OpenCL devices available
    /// for device in Device::devices(BackendApi::OpenCl).unwrap().flatten() {
    ///     let name = device.name().unwrap();
    ///     println!("Found OpenCL device: {}", name);
    /// }
    /// ```
    pub fn devices(backend: super::BackendApi) -> crate::Result<super::QueryDeviceIter> {
        super::QueryDeviceIter::new(backend)
    }

    /// Creates an `Engine` from the device.
    /// # Returns
    /// A `Result::Ok` that contains a `Engine` object if the engine creation was successful.
    /// # Errors
    ///
    /// Returns an `Result::Err(Error)` if the underlying context was failed to be created.
    #[inline]
    pub fn create_engine(&self) -> crate::Result<super::Engine> {
        self.0.create_engine()
    }

    /// Returns the backend API of the device.
    ///
    /// # Returns
    ///
    /// - `BackendApi::Cuda` if the device is a Nvidia CUDA device.
    /// - `BackendApi::Cl` if the device is an OpenCL device.
    /// # Examples
    /// ```
    ///  match device.backend() {
    ///     BackendApi::Cuda => println!("This device is a Nvidia CUDA device."),
    ///     BackendApi::Cl => println!("This device is an OpenCL device."),
    ///  }
    /// ```
    #[inline]
    pub fn backend(&self) -> super::BackendApi {
        self.0.backend()
    }

    /// Returns the name of the device.
    ///
    /// # Returns
    ///
    /// The name of the device as a `String` if it is successfully retrieved.
    ///
    /// # Errors
    ///
    /// Returns an `Result::Err(Error)` if there was a problem retrieving the device name.
    /// ```
    /// match device.name() {
    ///     Ok(name) => println!("The name of the device is {}.", name),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    #[inline]
    pub fn name(&self) -> crate::Result<String> {
        self.0.name()
    }
}

pub(super) trait DeviceInner {
    fn create_engine(&self) -> crate::Result<super::Engine>;
    fn backend(&self) -> super::BackendApi;
    fn name(&self) -> crate::Result<String>;
}

pub(super) struct CudaDevice {
    pub(super) inner: cuda_driver_sys::CUdevice
}

impl DeviceInner for CudaDevice {
    fn create_engine(&self) -> crate::Result<super::Engine> {
        let mut context = std::ptr::null_mut();

        unsafe {
            // Try creating CUDA context with the current device
            if cuda_driver_sys::cuCtxCreate_v2(&mut context, 
                    cuda_driver_sys::CUctx_flags_enum::CU_CTX_SCHED_AUTO as _, self.inner) != cuda_driver_sys::cudaError_enum::CUDA_SUCCESS {
                return crate::Errors::UnableToCreateCudaContext.into();
            }
        }

        Ok(super::Engine(Arc::new(super::CudaEngine {
            context
        })))
    }

    fn backend(&self) -> super::BackendApi {
        super::BackendApi::Cuda
    }

    fn name(&self) -> crate::Result<String> {
        let mut buffer = [0i8; 256];
        
        unsafe {
            // Try retrieving name
            if cuda_driver_sys::cuDeviceGetName(buffer.as_mut_ptr(), 256, self.inner) != cuda_driver_sys::CUresult::CUDA_SUCCESS {
                return crate::Errors::UnableToGetCudaDeviceName.into();
            }

            // Convert into Rust string
            let cstr = CStr::from_ptr(buffer.as_ptr());
            
            Ok(cstr.to_str()
            .or(crate::Errors::InvalidNameFormat.into())?
            .to_owned())
        }
    }
}

pub(super) struct ClDevice {
    pub(super) inner: super::VoidPtr
}

impl DeviceInner for ClDevice {
    fn create_engine(&self) -> crate::Result<super::Engine> {
        let context;

        unsafe {
            let mut errcode_ret = 0i32;
            
            context = cl3::ext::clCreateContext(std::ptr::null(), 1, &self.inner, None, std::ptr::null_mut(), &mut errcode_ret);

            if errcode_ret != cl3::context::CL_SUCCESS {
                return crate::Errors::UnableToCreateOpenClContext.into();
            }
        }

        Ok(super::Engine(Arc::new(super::ClEngine {
            context
        })))
    }

    fn backend(&self) -> super::BackendApi {
        super::BackendApi::OpenCl
    }

    fn name(&self) -> crate::Result<String> {
        
        unsafe {
            let mut length = 0usize;

            // Query device name length            
            if cl3::ext::clGetDeviceInfo(self.inner, cl3::device::CL_DEVICE_NAME, 
                0, std::ptr::null_mut(), &mut length) != 0 {
                return crate::Errors::UnableToGetOpenCLDeviceName.into();
            }

            // Allocate buffer for the name
            let layout = std::alloc::Layout::from_size_align_unchecked(length, std::mem::align_of::<i8>());
            let buffer = std::alloc::alloc_zeroed(layout) as *mut i8;

            if buffer.is_null() {
                return crate::Errors::UnableToGetOpenCLDeviceName.into();
            }

            // Query device name
            if cl3::ext::clGetDeviceInfo(self.inner, cl3::device::CL_DEVICE_NAME, 
                length, buffer as _, std::ptr::null_mut()) != 0 {
                return crate::Errors::UnableToGetOpenCLDeviceName.into();
            }

            // Convert to rust string
            let cstr = CStr::from_ptr(buffer);
            let name = cstr.to_str()
                .or(crate::Errors::InvalidNameFormat.into())?
                .to_string();

            // Free buffer
            std::alloc::dealloc(buffer as _, layout);

            Ok(name)
        }
    }
}

impl Drop for ClDevice {
    fn drop(&mut self) {
        unsafe {
            cl3::ext::clReleaseDevice(self.inner);
        }
    }
}