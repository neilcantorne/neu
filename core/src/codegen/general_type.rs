#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
pub enum GeneralType {
    Tensor1(u32, super::Element),
    Tensor2(u32, u32, super::Element),
    Tensor3(u32, u32, u32, super::Element),
    Element(super::Element)
}
