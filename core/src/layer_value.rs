pub trait LayerValue where Self::Element: crate::Element {
    type Element;

    fn dimension(&self) -> crate::Dimension;
}
