pub trait Element {
    const CHANNELS: usize;
}

impl Element for f32 {
    const CHANNELS: usize = 1;
}

impl Element for f64 {
    const CHANNELS: usize = 1;
}

impl Element for u8 {
    const CHANNELS: usize = 1;
}

impl Element for u16 {
    const CHANNELS: usize = 1;
}

impl Element for u32 {
    const CHANNELS: usize = 1;
}

impl Element for u64 {
    const CHANNELS: usize = 1;
}

impl Element for i8 {
    const CHANNELS: usize = 1;
}

impl Element for i16 {
    const CHANNELS: usize = 1;
}

impl Element for i32 {
    const CHANNELS: usize = 1;
}

impl Element for i64 {
    const CHANNELS: usize = 1;
}