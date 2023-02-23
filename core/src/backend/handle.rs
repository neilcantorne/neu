pub(super) trait Handle {
    const NULL: Self;

    fn is_null(&self) -> bool;
}