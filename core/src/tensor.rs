use std::ptr::NonNull;
use crate::Errors;

pub struct Tensor<F: crate::Element> {
    buffer: NonNull<F>,
    dimension: crate::Dimension,
}

impl<F: crate::Element + Copy> Tensor<F> {
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

        NonNull::new(alloc(layout) as *mut F)
            .ok_or(std::convert::Into::<crate::Error>::into(Errors::TensorAllocationFailed))
    }

    pub fn with_value(value: F, dimension: crate::Dimension) -> crate::Result<Self> {
        let buffer;

        unsafe {
            buffer = Self::allocate(&dimension)?;

            let mut current = buffer.as_ptr();
            let end = current.offset(dimension.0 as isize * dimension.1 as isize * dimension.2 as isize);

            while current < end {
                current.write(value);
                current = current.add(1);
            }
        }

        Ok(Self { buffer, dimension })
    }

    pub fn with_value_from_fn(generator: impl Fn(crate::Dimension) -> F, dimension: crate::Dimension) -> crate::Result<Self> {
        let buffer;

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

                        ptr.offset(index).write(value);
                    }
                }
            }
        }

        Ok(Self { buffer, dimension })
    }
}