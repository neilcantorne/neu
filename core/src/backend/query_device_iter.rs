use std::sync::Arc;
use super::cl;

pub struct QueryDeviceIter {
    cl: cl::OpenCl,
    cuda_count: i32,
    cuda_index: i32,
    cl_platforms: Option<Vec<cl::PlatformId>>,
    cl_platform_index: usize,
    cl_device_index: usize,
    cl_device_buffer: Option<Vec<cl::DeviceId>>
}

impl QueryDeviceIter {
    pub(super) fn new(backend: super::BackendApi) -> crate::Result<Self> {
        let cl= (cl::OpenCl::load()
            .ok_or(crate::Errors::UnableToLoadOpenCl.into()) as crate::Result::<cl::OpenCl>)?;

        let mut cuda_count = 0i32;
        let mut cl_platforms: Option<Vec::<cl::PlatformId>> = None;
        // Query CUDA device
        if matches!(backend, super::BackendApi::All | super::BackendApi::Cuda) {
            super::initialize_cuda()?;
            
            // Fill in the cuda device count
            unsafe { cuda_driver_sys::cuDeviceGetCount(&mut cuda_count); }
        }
        
        if matches!(backend, super::BackendApi::All | super::BackendApi::OpenCl) {
            let mut cl_platform_count = 0u32;

            unsafe {
                // Retrieve the number of OpenCL platforms currently available
                cl.get_platform_ids(0, std::ptr::null_mut(), &mut cl_platform_count);

                if cl_platform_count > 0 {
                    let mut buffer = Vec::with_capacity(cl_platform_count as _); // allocate buffer
                    
                    // Query platform IDs
                    cl.get_platform_ids(cl_platform_count, buffer.as_mut_ptr(), std::ptr::null_mut());
                    
                    buffer.set_len(cl_platform_count as _); // increase length to the platform count
                    cl_platforms = Some(buffer);
                }
            }
        }

        // Build an iterator
        Ok(Self {
            cl,
            cuda_count,
            cl_platforms,
            cuda_index: 0,
            cl_platform_index: 0,
            cl_device_index: 0,
            cl_device_buffer: None,
        })
    }
}

impl Iterator for QueryDeviceIter {
    type Item = crate::Result<crate::Device>;

    fn next(&mut self) -> Option<Self::Item> {

        // Starts with CUDA
        if self.cuda_index < self.cuda_count {
            unsafe {
                let mut device: cuda_driver_sys::CUdevice = Default::default();

                // Try retrieving device
                if cuda_driver_sys::cuDeviceGet(&mut device, self.cuda_index) != cuda_driver_sys::cudaError_enum::CUDA_SUCCESS {
                    return Some(crate::Errors::FailedToRetrieveCudaDevice.into());
                }

                self.cuda_index += 1;
    
                // Wrap the pointer 
                return Some(Ok(super::Device(Arc::new(super::CudaDevice {
                    inner: device
                }))));
            }
        }

        // Next is OpenCL
        if let Some(platforms) = &self.cl_platforms {
            if self.cl_platform_index > platforms.len() {
                return None; // If already at the end return None
            }

            // At the start of a new platform iteration create a buffer
            if self.cl_device_index == 0 {
                let mut device_count = 0u32;

                unsafe {
                    // Check the number of device in the current platform
                    if self.cl.get_device_ids(platforms[self.cl_platform_index], cl::DeviceType::All, 0, 
                        std::ptr::null_mut(), &mut device_count) != cl::Status::Success {
                        return Some(crate::Errors::FailedToRetrieveOpenClDevices.into());
                    }
                }
                
                if let Some(buffer) = &mut self.cl_device_buffer {
                    // If buffer already exists just resize it
                    buffer.resize(device_count as _ , cl::DeviceId::NULL);
                } else {
                    // Allocate a buffer if it does not exist yet
                    let mut buffer = Vec::<cl::DeviceId>::with_capacity(device_count as _);
                    buffer.resize(device_count as _, cl::DeviceId::NULL); // Set default size

                    self.cl_device_buffer = Some(buffer); // Update the device buffer
                };

                if let Some(devices) = &mut self.cl_device_buffer  {
                    unsafe {
                        // Try to retrieve all device in the platform and store it on devices buffer
                        if self.cl.get_device_ids(platforms[self.cl_platform_index], cl::DeviceType::All, device_count, 
                            devices.as_mut_ptr(), std::ptr::null_mut()) != cl::Status::Success {
                            return Some(crate::Errors::FailedToRetrieveOpenClDevices.into());
                        }
                    }
                }
            }

            if let Some(devices) = &self.cl_device_buffer  {
                // If at the end of the device buffer move to the next platform 
                if self.cl_device_index >= devices.len() {
                    self.cl_platform_index += 1;
                } else {
                    // otherwise move to the next device and return the current device
                    let device = super::Device(Arc::new(super::ClDevice {
                        id: devices[self.cl_device_index],
                        cl: self.cl.clone(),
                    }));

                    self.cl_device_index += 1;

                    return Some(Ok(device));
                }
            }
        }

        // Return if  even OpenCL platforms are not available
        None
    }
}