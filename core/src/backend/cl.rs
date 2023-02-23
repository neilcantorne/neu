
use bindgen::dl_link;


#[dl_link(OpenCl)]
#[libname("OpenCL")]
extern "C" {
    #[symbol(clGetPlatformIDs)]
    fn get_platform_ids(num_entries: u32,
                    platforms: *mut PlatformId,
                    num_platform: *mut u32) -> Status;

    #[symbol(clGetDeviceIDs)]
    fn get_device_ids(
        platform: PlatformId,
        device_type: DeviceType,
        num_entries: u32,
        devices: *mut DeviceId,
        num_devices: *mut u32) -> Status;

    #[symbol(clCreateContext)]
    fn create_context(properties: *const u64,
        num_devices: u32,
        cl_device_id: *const DeviceId,
        pfn_notify: Option<extern "C" fn (errinfo: *const i8, private_info: *const (), cb: usize, user_data: *const ())>,
        user_data: *const (),
        errcode_ret: *mut Status) -> Context;

    #[symbol(clGetDeviceInfo)]
    fn get_device_info( device: DeviceId,
        param_name: u32,
        param_value_size: usize,
        param_value: *mut (),
        param_value_size_ret: *mut usize) -> Status;

    #[symbol(clReleaseDevice)]
    fn release_device(device: DeviceId) -> Status;
    
    #[symbol(clReleaseContext)]
    fn release_context(context: Context) -> Status;
    
}


#[derive(Clone, Copy)]
pub(super) struct PlatformId(usize);

impl PlatformId {
    pub const NULL: Self = Self(0);
}

#[derive(Clone, Copy)]
pub(super) struct DeviceId(usize);

impl DeviceId {
    pub const NULL: Self = Self(0);
}

#[derive(Clone, Copy)]
pub(super) struct Context(usize);

impl Context {
    pub const NULL: Self = Self(0);
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub(super) enum DeviceType {
    Default = (1 << 0),
    Cpu = (1 << 1),
    Gpu = (1 << 2),
    Accelerator = (1 << 3),
    Custom = (1 << 4),
    All = 0xFFFFFFFF
}

#[repr(i32)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub(super) enum Status {
    Success = 0
}

impl Default for Status {
    #[inline(always)]
    fn default() -> Self {
        Self::Success
    }
}