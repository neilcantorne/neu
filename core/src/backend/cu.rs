
use bindgen::{
    dl_link,
    Handle
};

use super::Handle;

#[dl_link(Cuda)]
#[libname("cuda")]
extern "C" {
    #[symbol(cuInit)]
    fn init(flags: InitFlag) -> Status;
    
    #[symbol(cuDeviceGetCount)]
    fn device_get_count(count: *mut i32) -> Status;

    #[symbol(cuDeviceGet)]
    fn device_get(device: *mut Device, index: i32) -> Status;
    
    #[symbol(cuDeviceGetName)]
    fn device_get_name(buffer: *mut i8, len: u32, device: Device) -> Status;

    #[symbol(cuCtxCreate_v2)]
    fn ctx_create_v2(pctx: *mut Context, flags: CtxSchedFlag,  dev: Device) -> Status;

    #[symbol(cuCtxDestroy_v2)]
    fn ctx_destroy_v2(pctx: Context) -> Status;

    #[symbol(cuModuleLoadDataEx)]
    fn module_load_data_ex(module: *mut Module, image: *mut u8, num_option: u32, options: *const JitOption) -> Status;

}

#[derive(Handle)]
#[derive(Clone, Copy)]
pub(super) struct Device(i32);

#[derive(Handle)]
#[derive(Clone, Copy)]
pub(super) struct Context(usize);

#[derive(Handle)]
#[derive(Clone, Copy)]
pub(super) struct Module(usize);

#[repr(i32)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[allow(unused)]
pub(super) enum Status {
    Success = 0,
    NotFound = 1,
    InitializationError = 2,
    DriverLoadError = 3,
    InvalidValue = 4,
    AlreadyInitialized = 5,
    UnknownError = 999,
}

impl Default for Status {
    #[inline]
    fn default() -> Self {
        Self::Success
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[allow(unused)]
pub(super) enum InitFlag {
    Default = 0,
    DisableScheduler = 1 << 0,
    ThreadAffinity = 1 << 1,
    ExternalMemoryHost = 1 << 2,
    ExternalMemoryCuda = 1 << 3,
    BlockingSync = 1 << 4,
    PrimaryContext = 1 << 5,
}

#[repr(u32)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[allow(unused)]
pub enum CtxSchedFlag {
    Auto = 0,
    Spin = 1,
    Yield = 2,
    BlockingSync = 4,
}


#[repr(u32)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[allow(unused)]
pub enum JitOption {
    MaxRegisters,
    ThreadsPerBlock,
    WallTime,
    InfoLogBuffer,
    InfoLogBufferSizeBytes,
    ErrorLogBuffer,
    ErrorLogBufferSizeBytes,
    OptimizationLevel,
    TargetFromCucontext,
    Target,
    FallbackStrategy,
    GenerateDebugInfo,
    LogVerbose,
    GenerateLineInfo,
    CacheMode,
    NewSm3xOpt,
    FastCompile,
    GlobalSymbolNames,
    GlobalSymbolAddresses,
    GlobalSymbolCount,
    NumOptions,
}

