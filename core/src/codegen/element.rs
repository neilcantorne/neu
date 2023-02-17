#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
pub struct Element {
    pub(super) channels: usize,
    pub(super) type_: super::ScalarType,
}
