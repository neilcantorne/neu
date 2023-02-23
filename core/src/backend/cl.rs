
use bindgen::{
    dl_link,
    Handle
};

use super::Handle;

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
    fn get_device_info( 
        evice: DeviceId,
        param_name: DeviceInfoProperty,
        param_value_size: usize,
        param_value: *mut (),
        param_value_size_ret: *mut usize) -> Status;

    #[symbol(clCreateCommandQueueWithProperties)]
    fn create_command_queue_with_properties(
        context: Context,
        device: DeviceId,
        properties: *const QueueProperty,
        errcode_ret: *mut Status) -> CommandQueue;

    #[symbol(clReleaseDevice)]
    fn release_device(device: DeviceId) -> Status;
    
    #[symbol(clReleaseContext)]
    fn release_context(context: Context) -> Status;
    
    #[symbol(clReleaseCommandQueue)]
    fn release_command_queue(command_queue: CommandQueue) -> Status;
    
}


#[derive(Handle)]
#[derive(Clone, Copy)]
pub(super) struct PlatformId(usize);

#[derive(Handle)]
#[derive(Clone, Copy)]
pub(super) struct DeviceId(usize);

#[derive(Handle)]
#[derive(Clone, Copy)]
pub(super) struct Context(usize);

#[derive(Handle)]
#[derive(Clone, Copy)]
pub(super) struct CommandQueue(usize);

#[repr(i32)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[allow(unused)]
pub(super) enum Status {
    Success = 0,
    DeviceNotFound = -1,
    DeviceNotAvailable = -2,
    CompilerNotAvailable = -3,
    MemObjectAllocationFailure = -4,
    OutOfResources = -5,
    OutOfHostMemory = -6,
    ProfilingInfoNotAvailable = -7,
    MemCopyOverlap = -8,
    ImageFormatMismatch = -9,
    ImageFormatNotSupported = -10,
    BuildProgramFailure = -11,
    MapFailure = -12,
    MisalignedSubBufferOffset = -13,
    ExecStatusErrorForEventsInWaitList = -14,
    CompileProgramFailure = -15,
    LinkerNotAvailable = -16,
    LinkProgramFailure = -17,
    DevicePartitionFailed = -18,
    KernelArgInfoNotAvailable = -19,
    InvalidValue = -30,
    InvalidDeviceType = -31,
    InvalidPlatform = -32,
    InvalidDevice = -33,
    InvalidContext = -34,
    InvalidQueueProperties = -35,
    InvalidCommandQueue = -36,
    InvalidHostPtr = -37,
    InvalidMemObject = -38,
    InvalidImageFormatDescriptor = -39,
    InvalidImageSize = -40,
    InvalidSampler = -41,
    InvalidBinary = -42,
    InvalidBuildOptions = -43,
    InvalidProgram = -44,
    InvalidProgramExecutable = -45,
    InvalidKernelName = -46,
    InvalidKernelDefinition = -47,
    InvalidKernel = -48,
    InvalidArgIndex = -49,
    InvalidArgValue = -50,
    InvalidArgSize = -51,
    InvalidKernelArgs = -52,
    InvalidWorkDimension = -53,
    InvalidWorkGroupSize = -54,
    InvalidWorkItemSize = -55,
    InvalidGlobalOffset = -56,
    InvalidEventWaitList = -57,
    InvalidEvent = -58,
    InvalidOperation = -59,
    InvalidGLObject = -60,
    InvalidBufferSize = -61,
    InvalidMipLevel = -62,
    InvalidGlobalWorkSize = -63,
    InvalidProperty = -64,
    InvalidImageDescriptor = -65,
    InvalidCompilerOptions = -66,
    InvalidLinkerOptions = -67,
    InvalidDevicePartitionCount = -68,
    InvalidPipeSize = -69,
    InvalidDeviceQueue = -70,
    InvalidSpecId = -71,
    MaxSizeRestrictionExceeded = -72,
}

impl Default for Status {
    #[inline(always)]
    fn default() -> Self {
        Self::Success
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
#[allow(unused)]
pub(super) enum DeviceType {
    Default = (1 << 0),
    Cpu = (1 << 1),
    Gpu = (1 << 2),
    Accelerator = (1 << 3),
    Custom = (1 << 4),
    All = 0xFFFFFFFF
}

#[repr(u32)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[allow(unused)]
pub(super) enum DeviceInfoProperty {
    Name = 0x102B,
    Vendor = 0x102C,
    DriverVersion = 0x102D,
    Profile = 0x102E,
    Version = 0x102F,
    OpenCLCVersion = 0x103D,
    Extensions = 0x1030,
    Platform = 0x1031,
    DoubleFPConfig = 0x1032,
    HalfFPConfig = 0x1033,
    PreferredVectorWidthChar = 0x1034,
    PreferredVectorWidthShort = 0x1035,
    PreferredVectorWidthInt = 0x1036,
    PreferredVectorWidthLong = 0x1037,
    PreferredVectorWidthFloat = 0x1038,
    PreferredVectorWidthDouble = 0x1039,
    MaxClockFrequency = 0x100C,
    AddressBits = 0x100D,
    MaxReadImageArgs = 0x100E,
    MaxWriteImageArgs = 0x100F,
    MaxMemoryAllocationSize = 0x1010,
    Image2DMaxWidth = 0x1011,
    Image2DMaxHeight = 0x1012,
    Image3DMaxWidth = 0x1013,
    Image3DMaxHeight = 0x1014,
    Image3DMaxDepth = 0x1015,
    ImageSupport = 0x1016,
    MaxParameterSize = 0x1017,
    MaxSamplers = 0x1018,
    MemoryBaseAddressAlignment = 0x1019,
    MinDataTypeAlignSize = 0x101A,
    SingleFPConfig = 0x101B,
    GlobalMemoryCacheType = 0x101C,
    GlobalMemoryCacheLineSize = 0x101D,
    GlobalMemoryCacheSize = 0x101E,
    GlobalMemorySize = 0x101F,
    MaxConstantBufferSize = 0x1020,
    MaxConstantArgs = 0x1021,
    LocalMemoryType = 0x1022,
    LocalMemorySize = 0x1023,
    ErrorCorrectionSupport = 0x1024,
    ProfilingTimerResolution = 0x1025,
    EndianLittle = 0x1026,
    Available = 0x1027,
    CompilerAvailable = 0x1028,
    ExecutionCapabilities = 0x1029,
    QueueProperties = 0x102A,
    BuiltInKernels = 0x103E,
    PreferredInteropUserSync = 0x1048,
    PrintfBufferSize = 0x1049,
    ParentDevice = 0x1042,
    PartitionProperties = 0x1151,
    PartitionAffinityDomain = 0x1152,
    PartitionType = 0x1153,
    ReferenceCount = 0x1043,
    PreferredGlobalAtomicAlignment = 0x1044,
    PreferredLocalAtomicAlignment = 0x1045,
    MaxNumSubGroups = 0x1046,
    SubGroupIndependentForwardProgress = 0x1047,
}

#[repr(i32)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[allow(unused)]
pub(super) enum QueueProperty {
    Context = -0x1090,
    Device = -0x1091,
    ReferenceCount = -0x1092,
    Properties = -0x1093,
    Size = -0x1094,
    DeviceDefaultOrEnd = 0,
    Priority = -0x1096,
    Throttle = -0x1097,
    NodeMask = -0x1098,
    ContextOnDevice = -0x109A,
    PropertiesArray = -0x109B,
    SizeArray = -0x109C,
    Count = -0x109D,
    DeviceQueueCapabilities = -0x109E,
    MaxSize = -0x109F,
    MaxDeviceQueueSize = -0x10A0,
    MaxPipelines = -0x10A1,
    Pipelined = -0x10A2,
    DeviceQueueProperties = -0x10A3,
    CreateFlags = -0x10A4,
    PropertiesListBeginAmd = 0x40E0,
    PropertiesListEndAmd = 0x40E1,
    ThreadHandle = -0x40E2,
    PropertiesArraySizeAmd = 0x40E3,
    ThreadHandleKhr = -0x40E4,
    PropertiesArraySizeKhr = 0x40E5,
}