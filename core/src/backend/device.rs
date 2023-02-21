use std::sync::Arc;

pub struct Device(pub(super) Arc<dyn DeviceInner>);

impl Device {
    pub fn devices(backend: super::Backend) -> crate::Result<super::QueryDeviceIter> {
        super::QueryDeviceIter::new(backend)
    }

    #[inline]
    pub fn create_engine(&self) -> crate::Result<super::Engine> {
        self.0.create_engine()
    }
}

pub(super) trait DeviceInner {
    fn create_engine(&self) -> crate::Result<super::Engine>;
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
}