#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
pub enum GeneralType {
    Tensor(u32, u32, u32, super::ElementType),
    Element(super::ElementType)
}