use gcd::binary_u64;

enum AspectRatio {
    _16x10 = b00,
    _4x3 = b01,
    _5x4 = b10,
    _16x9 = b11,
}

pub struct EdidInfo {
    aspect_ratio: AspectRatio,
    width: u64,
    height: u64,
    refresh_rate: u64,
}

impl From<&[u8]> for EdidInfo {
    fn compute_width_height(width_byte: u8, height_byte: u8) -> Option<(u64, u64)> {
        match (width_byte, height_byte) {
            (0, 0) => None,
            (0, _) =>  todo!(),
            (_, 0) => todo!(),
            (w, h) => (u64::from(w), u64::from(h)),
        }
    }
    fn from(data: &[u8]) -> Self {
        let aspect_ratio = (data[39] & (0b11 << 6) >> 6) as AspectRatio;
        let width = Self::compute_width_or_height(data[21], aspect_ratio);
        let height = Self::compute_width_or_height(data[22], aspect_ratio);
        let refresh_rate = u64::from(data[39] & 0x1f) + 60;
        Self {
            aspect_ratio,
            width,
            height,
            refresh_rate
        }
    }
}

impl EdidInfo {
    /// Computes the number of pixels per square centimeter of the display
    #[inline]
    pub fn px_per_square_cm(&self) -> usize {
        let sq_mm = self.width * self.height;
        let px = self.width_px * self.height_px;
        px / sq_mm
    }
    /// Computes the number of pixels per centimeter (width, height)
    #[inline]
    pub fn px_per_cm(&self) -> (u64, u64) {
        (self.width_px / self.width, self.height_px / self.height)
    }
    /// Computes the aspect ratio of the display in the form (width, height)
    #[inline]
    pub fn aspect_ratio(&self) -> (u64, u64) {
        let gcd = binary_u64(self.width_px, self.height_px);
        (self.width_px / gcd, self.height_px / gcd)
    }
}