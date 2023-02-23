use std::ptr::NonNull;

pub(super) struct DynamicLibrary(NonNull<()>);

impl DynamicLibrary {
    #[cfg(target_os = "macos")]
    pub(super) fn load(filename: &[u8]) -> Option<Self> {
        let filename = format!("lib{filename}.dylib\0");

        NonNull::new(unsafe { dlopen(filename.as_ptr(), 1) }).map(Self)
    }

    #[cfg(target_os = "linux")]
    pub(super) fn load(filename: &str) -> Option<Self> {
        let filename = format!("lib{filename}.so\0");

        NonNull::new(unsafe { dlopen(filename.as_ptr(), 1) }).map(Self)
    }

    #[cfg(target_family = "windows")]
    pub(super) fn load(filename: &[u8]) -> Option<Self> {
        let filename = format!("{filename}.dll\0");
        
        NonNull::new(unsafe { dlopen(filename.as_ptr(), 1) }).map(Self)
    }

    pub(super) fn get_function(&self, symbol: &[u8]) -> Option<NonNull<()>> {
        unsafe {
            NonNull::new(dlsym(self.0.as_ptr(), symbol.as_ptr()))
        }
    }
}

impl Drop for DynamicLibrary {
    #[cfg(target_family = "unix")]
    fn drop(&mut self) {
        unsafe {
            dlclose(self.0.as_ptr())
        }
    }

    #[cfg(target_family = "windows")]
    fn drop(&mut self) {
        unsafe {
            FreeLibrary(self.0.as_ptr())
        }
    }
}

#[cfg(target_family = "unix")]
#[link(name = "dl")]
extern "C" {
   fn dlopen(filename: *const u8,  flag: i8) -> *mut ();
   fn dlsym(handle: *mut (), symbol: *const u8) -> *mut (); 
   fn dlclose(handle: *mut ());
}

#[cfg(target_family = "windows")]
#[link(name = "kernel32")]
extern "C" {
    fn LoadLibraryA(filename: *const u8) -> *mut ();
    fn GetProcAddress(handle: *mut (), symbol: *const u8) -> *mut (); 
    fn FreeLibrary(handle: *mut ());
}