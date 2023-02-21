use std::{sync::Arc, ffi::CStr};

pub struct Device(pub(super) Arc<dyn DeviceInner>);

impl Device {
    pub fn devices(backend: super::BackendApi) -> crate::Result<super::QueryDeviceIter> {
        super::QueryDeviceIter::new(backend)
    }

    #[inline]
    pub fn create_engine(&self) -> crate::Result<super::Engine> {
        self.0.create_engine()
    }

    #[inline]
    pub fn backend(&self) -> super::BackendApi {
        self.0.backend()
    }

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
    pub(super) inner: opencl_sys::cl_device_id
}

impl DeviceInner for ClDevice {
    fn create_engine(&self) -> crate::Result<super::Engine> {
        let context;

        unsafe {
            let mut errcode_ret = 0i32;
            
            context = opencl_sys::clCreateContext(std::ptr::null(), 1, &self.inner, None, std::ptr::null_mut(), &mut errcode_ret);

            if errcode_ret != opencl_sys::CL_SUCCESS {
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
            if opencl_sys::clGetDeviceInfo(self.inner, opencl_sys::CL_DEVICE_NAME, 
                0, std::ptr::null_mut(), &mut length) != opencl_sys::CL_SUCCESS {
                return crate::Errors::UnableToGetOpenCLDeviceName.into();
            }

            // Allocate buffer for the name
            let layout = std::alloc::Layout::from_size_align_unchecked(length, std::mem::align_of::<i8>());
            let buffer = std::alloc::alloc_zeroed(layout) as *mut i8;

            if buffer.is_null() {
                return crate::Errors::UnableToGetOpenCLDeviceName.into();
            }

            // Query device name
            if opencl_sys::clGetDeviceInfo(self.inner, opencl_sys::CL_DEVICE_NAME, 
                length, buffer as _, std::ptr::null_mut()) != opencl_sys::CL_SUCCESS {
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
            opencl_sys::clReleaseDevice(self.inner);
        }
    }
}