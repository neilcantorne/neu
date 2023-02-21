use std::ptr::NonNull;

pub(super) struct DynamicLibrary(NonNull<()>);

impl DynamicLibrary {
    #[cfg(target_family = "unix")]
    pub(super) fn load(filename: &[u8]) -> Option<Self> {
        Some(Self(NonNull::new(unsafe { dlopen(filename.as_ptr(), 0) })?))
    }

    #[cfg(target_family = "windows")]
    pub(super) fn load(filename: &[u8]) -> Option<Self> {
        Some(Self(NonNull::new(unsafe { LoadLibraryA(filename.as_ptr()) })?))
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