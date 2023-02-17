use std::ptr::NonNull;

use crate::Errors;

pub struct Tensor<F> {
    buffer: NonNull<F>,
    dimension: crate::Dimension,
    channels: usize,
}

impl<F: Copy> Tensor<F> {

    unsafe fn allocate(dimension: &crate::Dimension) -> crate::Result<NonNull<F>> {
        use std::alloc::*;

        let layout = {
            let align  = std::mem::align_of::<F>();
            let size = std::mem::size_of::<F>();

            Layout::from_size_align(size * 
                dimension.0 as usize *
                dimension.1 as usize *
                dimension.2 as usize, align)
            .or(Errors::InvalidTensorLayout.into())?
        };

        NonNull::new(alloc(layout) as _)
            .ok_or(std::convert::Into::<crate::Error>::into(Errors::TensorAllocationFailed))
    }

    pub fn with_value(value: impl TryInto<F, Error = crate::Error> + crate::ChannelCount + Copy, dimension: crate::Dimension) -> crate::Result<Self> {
        let buffer;

        unsafe {
            buffer = Self::allocate(&dimension)?;

            let mut current = buffer.as_ptr();
            let end = current.offset(dimension.0 as isize * dimension.1 as isize * dimension.2 as isize);

            while current < end {
                current.write(value.try_into()?);
                current = current.add(1);
            }
        }

        Ok(Self {
            buffer,
            dimension,
            channels: value.channels(),
        })
    }

    pub fn with_value_from_fn<T>(generator: impl Fn(crate::Dimension) -> T, dimension: crate::Dimension) -> crate::Result<Self>
        where T: TryInto<F, Error = crate::Error> + crate::ChannelCount {
        let buffer;
        let mut channels = None;

        unsafe {
            buffer = Self::allocate(&dimension)?;
            let ptr = buffer.as_ptr();
            
            let w1x2 = dimension.1 as isize * dimension.2 as isize;

            for i2 in 0..dimension.2 {
                for i1 in 0..dimension.1 {
                    for i0 in 0..dimension.0 {
                        let index = i0 as isize
                            + i1 as isize * dimension.0 as isize
                            + i2 as isize * w1x2;

                        let value = (generator)(crate::Dimension(i0, i1, i2));

                        if let Some(channels) = channels {
                            if channels != value.channels() {
                                return Errors::TensorNonUniformChannel.into();
                            }
                        } else {
                            channels = Some(value.channels())
                        }

                        ptr.offset(index).write(value.try_into()?);
                    }
                }
            }
        }

        Ok(Self {
            buffer,
            dimension,
            channels: channels.unwrap()
        })
    }

    pub fn dimension(&self) -> &crate::Dimension {
        &self.dimension
    }

    pub fn channels(&self) -> usize {
        self.channels
    }
}

impl<F: PartialEq> PartialEq for Tensor<F> {
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