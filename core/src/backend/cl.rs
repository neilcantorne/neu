
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
    fn get_device_info( 
        evice: DeviceId,
        param_name: DeviceInfoProperty,
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
