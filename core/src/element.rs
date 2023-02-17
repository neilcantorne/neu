use std::ptr::NonNull;

pub struct Element<F: Copy> {
    channels: usize,
    buffer: NonNull<F>
}

impl<F: Copy> Element<F> {
    pub fn channels(&self) -> usize {
        self.channels
    }
    
    unsafe fn allocate(channels: usize) -> crate::Result<NonNull<F>> {
        use std::alloc::*;

        let layout = {
            let alignment = std::mem::align_of::<F>();
            let size = std::mem::size_of::<F>() * channels;

            Layout::from_size_align(size, alignment)
                .or(crate::Errors::ElementAllocationFailed.into())?
        };

        NonNull::new(alloc(layout) as _)
            .ok_or(crate::Errors::ElementAllocationFailed.into())
    }
}

impl TryFrom<f32> for Element<f32> {
    type Error = crate::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<f64> for Element<f64> {
    type Error = crate::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<u8> for Element<u8> {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<u16> for Element<u16> {
    type Error = crate::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<u32> for Element<u32> {
    type Error = crate::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<u64> for Element<u64> {
    type Error = crate::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<i8> for Element<i8> {
    type Error = crate::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<i16> for Element<i16> {
    type Error = crate::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<i32> for Element<i32> {
    type Error = crate::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl TryFrom<i64> for Element<i64> {
    type Error = crate::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 1,
            buffer: {
                let buffer = unsafe { Self::allocate(1)? };
                unsafe { buffer.as_ptr().write(value); }
                buffer
            }
        })
    }
}

impl<F: Copy> TryFrom<(F, F)> for Element<F> {
    type Error = crate::Error;

    fn try_from(value: (F, F)) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 2,
            buffer: {
                let buffer = unsafe { Self::allocate(2)? };

                unsafe {
                    buffer.as_ptr().write(value.0);
                    buffer.as_ptr().add(1).write(value.1);
                }
                
                buffer
            }
        })
    }
}

impl<F: Copy> TryFrom<(F, F, F)> for Element<F> {
    type Error = crate::Error;

    fn try_from(value: (F, F, F)) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 3,
            buffer: {
                let buffer = unsafe { Self::allocate(3)? };

                unsafe {
                    buffer.as_ptr().write(value.0);
                    buffer.as_ptr().add(1).write(value.1);
                    buffer.as_ptr().add(2).write(value.2);
                }
                
                buffer
            }
        })
    }
}

impl<F: Copy> TryFrom<(F, F, F, F)> for Element<F> {
    type Error = crate::Error;

    fn try_from(value: (F, F, F, F)) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: 4,
            buffer: {
                let buffer = unsafe { Self::allocate(4)? };

                unsafe {
                    buffer.as_ptr().write(value.0);
                    buffer.as_ptr().add(1).write(value.1);
                    buffer.as_ptr().add(2).write(value.2);
                    buffer.as_ptr().add(3).write(value.3);
                }
                
                buffer
            }
        })
    }
}

impl<F: Copy> Drop for Element<F> {
    fn drop(&mut self) {
        {
            let mut i = 0;

            while i < self.channels {
                unsafe {
                    self.buffer.as_ptr()
                        .add(i)
                        .drop_in_place();
                }

                i += 1;
            }
        }

        use std::alloc::*;

        let layout = unsafe {
            let alignment = std::mem::align_of::<F>();
            let size = std::mem::size_of::<F>();

            Layout::from_size_align_unchecked(size, alignment)
        };

        unsafe {
            dealloc(self.buffer.as_ptr() as _, layout);
        }
    }
}

impl<F: PartialEq + Copy> PartialEq for Element<F> {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.buffer;
        let mut b = other.buffer;
        let end = unsafe { self.buffer.as_ptr().add(self.channels) };

        while a.as_ptr() < end {
            unsafe {
                if F::ne(a.as_ref(), b.as_ref()) {
                    return false;
                }
            }

            a = unsafe { NonNull::new_unchecked( a.as_ptr().add(1)) };
            b = unsafe { NonNull::new_unchecked( b.as_ptr().add(1)) };
        }

        true
    }
}

pub trait ChannelCount {
    fn channels(&self) -> usize;
}