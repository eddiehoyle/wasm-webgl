pub struct Viewport {
    width: u32,
    height: u32,
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Viewport{width, height}
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}