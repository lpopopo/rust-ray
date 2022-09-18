pub struct Image {
    height: u32,
    width: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image { height, width }
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
}
