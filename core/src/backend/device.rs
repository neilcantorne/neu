use std::sync::Arc;

pub struct Device(pub(super) Arc<dyn DeviceInner>);

impl Device {
    pub fn devices(backend: super::Backend) -> crate::Result<super::QueryDeviceIter> {
        super::QueryDeviceIter::new(backend)
    }
}

pub(super) trait DeviceInner {

}

pub(super) struct CudaDevice {
    pub(super) inner: cuda_driver_sys::CUdevice
}

impl DeviceInner for CudaDevice {

}

pub(super) struct ClDevice {
    pub(super) inner: opencl_sys::cl_device_id
}

impl DeviceInner for ClDevice {

}