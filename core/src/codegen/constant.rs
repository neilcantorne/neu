#[derive(PartialEq)]
pub enum Constant {
    ElementF32(crate::Element<f32>),
    ElementF64(crate::Element<f64>),
    ElementU8(crate::Element<u8>),
    ElementU16(crate::Element<u16>),
    ElementU32(crate::Element<u32>),
    ElementU64(crate::Element<u64>),
    ElementI8(crate::Element<i8>),
    ElementI16(crate::Element<i16>),
    ElementI32(crate::Element<i32>),
    ElementI64(crate::Element<i64>),

    TensorF32(crate::Tensor<f32>),
    TensorF64(crate::Tensor<f64>),
    TensorU8(crate::Tensor<u8>),
    TensorU16(crate::Tensor<u16>),
    TensorU32(crate::Tensor<u32>),
    TensorU64(crate::Tensor<u64>),
    TensorI8(crate::Tensor<i8>),
    TensorI16(crate::Tensor<i16>),
    TensorI32(crate::Tensor<i32>),
    TensorI64(crate::Tensor<i64>),
}

impl Constant {
    pub(super) fn general_type(&self) -> super::GeneralType {
        match self {
            Constant::ElementF32(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::F32)),
            Constant::ElementF64(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::F64)),
            Constant::ElementU8(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::U8)),
            Constant::ElementU16(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::U16)),
            Constant::ElementU32(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::U32)),
            Constant::ElementU64(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::U64)),
            Constant::ElementI8(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::U8)),
            Constant::ElementI16(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::U16)),
            Constant::ElementI32(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::U32)),
            Constant::ElementI64(element)
                => super::GeneralType::Element(
                    super::ElementType(element.channels() as _,
                    super::ScalarType::U64)),
            Constant::TensorF32(tensor)
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::F32)),
            Constant::TensorF64(tensor)
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::F64)),
            Constant::TensorU8(tensor)
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::U8)),
            Constant::TensorU16(tensor)
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::U16)),
            Constant::TensorU32(tensor) 
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::U32)),
            Constant::TensorU64(tensor)
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::U64)),
            Constant::TensorI8(tensor)
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::I8)),
            Constant::TensorI16(tensor) 
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::I16)),
            Constant::TensorI32(tensor)
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::I32)),
            Constant::TensorI64(tensor)
                => super::GeneralType::Tensor(
                        tensor.dimension().0,
                        tensor.dimension().1,
                        tensor.dimension().2,
                        super::ElementType(tensor.channels() as _,
                        super::ScalarType::I64))
        }
    }
}