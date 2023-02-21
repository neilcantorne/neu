use std::sync::Arc;


pub struct QueryDeviceIter {
    pub(super) cuda_count: i32,
    pub(super) cuda_index: i32,
    pub(super) cl_platforms: Option<Vec<opencl_sys::cl_platform_id>>,
    pub(super) cl_platform_index: usize,
    pub(super) cl_device_index: usize,
    pub(super) cl_device_buffer: Option<Vec<opencl_sys::cl_device_id>>
}

impl QueryDeviceIter {
    pub(super) fn new(backend: super::Backend) -> crate::Result<Self> {
        let mut cuda_count = 0i32;
        let mut cl_platforms: Option<Vec::<opencl_sys::cl_platform_id>> = None;
        // Query CUDA device
        if matches!(backend, super::Backend::All | super::Backend::Cuda) {
            super::initialize_cuda()?;
            
            // Fill in the cuda device count
            unsafe { cuda_driver_sys::cuDeviceGetCount(&mut cuda_count); }
        }
        
        if matches!(backend, super::Backend::All | super::Backend::OpenCl) {
            let mut cl_platform_count = 0u32;

            unsafe {
                // Retrieve the number of OpenCL platforms currently available
                opencl_sys::clGetPlatformIDs(0, std::ptr::null_mut(), &mut cl_platform_count);

                if cl_platform_count > 0 {
                    let mut buffer = Vec::with_capacity(cl_platform_count as _); // allocate buffer
                    
                    // Query platform IDs
                    opencl_sys::clGetPlatformIDs(cl_platform_count, buffer.as_mut_ptr(), std::ptr::null_mut());
                    
                    buffer.set_len(cl_platform_count as _); // increase length to the platform count
                    cl_platforms = Some(buffer);
                }
            }
        }

        // Build an iterator
        Ok(Self {
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
                    println!("{:?}", platforms[self.cl_platform_index]);
                    // Check the number of device in the current platform
                    if opencl_sys::clGetDeviceIDs(platforms[self.cl_platform_index], opencl_sys::CL_DEVICE_TYPE_ALL, 0, 
                        std::ptr::null_mut(), &mut device_count) != opencl_sys::CL_SUCCESS {
                        return Some(crate::Errors::FailedToRetrieveOpenClDevices.into());
                    }
                }
                
                if let Some(buffer) = &mut self.cl_device_buffer {
                    // If buffer already exists just resize it
                    buffer.resize(device_count as _ , std::ptr::null_mut());
                } else {
                    // Allocate a buffer if it does not exist yet
                    let mut buffer = Vec::<opencl_sys::cl_device_id>::with_capacity(device_count as _);
                    buffer.resize(device_count as _, std::ptr::null_mut()); // Set default size

                    self.cl_device_buffer = Some(buffer); // Update the device buffer
                };

                if let Some(devices) = &mut self.cl_device_buffer  {
                    unsafe {
                        // Try to retrieve all device in the platform and store it on devices buffer
                        if opencl_sys::clGetDeviceIDs(platforms[self.cl_platform_index], opencl_sys::CL_DEVICE_TYPE_ALL, device_count, 
                            devices.as_mut_ptr(), std::ptr::null_mut()) != opencl_sys::CL_SUCCESS {
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
                    //otherwise move to the next device and return the current device
                    let device = super::Device(Arc::new(super::ClDevice {
                        inner: devices[self.cl_device_index]
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