#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
pub enum GeneralType {
    Tensor1(u32, super::ElementType),
    Tensor2(u32, u32, super::ElementType),
    Tensor3(u32, u32, u32, super::ElementType),
    Element(super::ElementType)
}