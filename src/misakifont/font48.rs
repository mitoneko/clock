/// 半角フォント
pub struct Font48 ([Font48Data; 256]);

struct Font48Data([u8; 8]);

impl Font48 {
    pub fn get_char(&self, code: u8) -> &[u8; 8] {
        &self.0[code as usize].0
    }
}

pub const FONT48:Font48 = Font48 ( [
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x4,0x4,0x4,0x4,0x4,0x0,0x4,0x0]),
Font48Data([0xa,0xa,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0xa,0xe,0xa,0xa,0xa,0xe,0xa,0x0]),
Font48Data([0x4,0xe,0x8,0xe,0x2,0xe,0x4,0x0]),
Font48Data([0x0,0x8,0x2,0x4,0x8,0x2,0x0,0x0]),
Font48Data([0x4,0xa,0xa,0x4,0xa,0x8,0x6,0x0]),
Font48Data([0x4,0x4,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x2,0x4,0x4,0x4,0x4,0x4,0x2,0x0]),
Font48Data([0x8,0x4,0x4,0x4,0x4,0x4,0x8,0x0]),
Font48Data([0x0,0xa,0x4,0xe,0x4,0xa,0x0,0x0]),
Font48Data([0x0,0x4,0x4,0xe,0x4,0x4,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x4,0x8,0x0]),
Font48Data([0x0,0x0,0x0,0xe,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x4,0x0]),
Font48Data([0x0,0x0,0x2,0x4,0x8,0x0,0x0,0x0]),
Font48Data([0x4,0xa,0xa,0xe,0xa,0xa,0x4,0x0]),
Font48Data([0x4,0xc,0x4,0x4,0x4,0x4,0xe,0x0]),
Font48Data([0x4,0xa,0x2,0x4,0x4,0x8,0xe,0x0]),
Font48Data([0x4,0xa,0x2,0x4,0x2,0xa,0x4,0x0]),
Font48Data([0x2,0x6,0xa,0xa,0xe,0x2,0x2,0x0]),
Font48Data([0xe,0x8,0xc,0xa,0x2,0xa,0x4,0x0]),
Font48Data([0x4,0xa,0x8,0xc,0xa,0xa,0x4,0x0]),
Font48Data([0xe,0xa,0x2,0x4,0x4,0x4,0x4,0x0]),
Font48Data([0x4,0xa,0xa,0x4,0xa,0xa,0x4,0x0]),
Font48Data([0x4,0xa,0xa,0x6,0x2,0xa,0x4,0x0]),
Font48Data([0x0,0x0,0x4,0x0,0x0,0x4,0x0,0x0]),
Font48Data([0x0,0x0,0x4,0x0,0x0,0x4,0x8,0x0]),
Font48Data([0x0,0x2,0x4,0x8,0x4,0x2,0x0,0x0]),
Font48Data([0x0,0x0,0xe,0x0,0xe,0x0,0x0,0x0]),
Font48Data([0x0,0x8,0x4,0x2,0x4,0x8,0x0,0x0]),
Font48Data([0x4,0xa,0x2,0x4,0x4,0x0,0x4,0x0]),
Font48Data([0x4,0xa,0xe,0xa,0xc,0x8,0x6,0x0]),
Font48Data([0x4,0xa,0xa,0xa,0xe,0xa,0xa,0x0]),
Font48Data([0xc,0xa,0xa,0xc,0xa,0xa,0xc,0x0]),
Font48Data([0x4,0xa,0x8,0x8,0x8,0xa,0x4,0x0]),
Font48Data([0xc,0xa,0xa,0xa,0xa,0xa,0xc,0x0]),
Font48Data([0xe,0x8,0x8,0xc,0x8,0x8,0xe,0x0]),
Font48Data([0xe,0x8,0x8,0xc,0x8,0x8,0x8,0x0]),
Font48Data([0x4,0xa,0x8,0xa,0xa,0xa,0x6,0x0]),
Font48Data([0xa,0xa,0xa,0xe,0xa,0xa,0xa,0x0]),
Font48Data([0xe,0x4,0x4,0x4,0x4,0x4,0xe,0x0]),
Font48Data([0x2,0x2,0x2,0x2,0x2,0xa,0x4,0x0]),
Font48Data([0xa,0xa,0xc,0xc,0xa,0xa,0xa,0x0]),
Font48Data([0x8,0x8,0x8,0x8,0x8,0x8,0xe,0x0]),
Font48Data([0xa,0xe,0xe,0xe,0xa,0xa,0xa,0x0]),
Font48Data([0xc,0xa,0xa,0xa,0xa,0xa,0xa,0x0]),
Font48Data([0x4,0xa,0xa,0xa,0xa,0xa,0x4,0x0]),
Font48Data([0xc,0xa,0xa,0xc,0x8,0x8,0x8,0x0]),
Font48Data([0x4,0xa,0xa,0xa,0xe,0xa,0x6,0x0]),
Font48Data([0xc,0xa,0xa,0xc,0xa,0xa,0xa,0x0]),
Font48Data([0x4,0xa,0x8,0x4,0x2,0xa,0x4,0x0]),
Font48Data([0xe,0x4,0x4,0x4,0x4,0x4,0x4,0x0]),
Font48Data([0xa,0xa,0xa,0xa,0xa,0xa,0xe,0x0]),
Font48Data([0xa,0xa,0xa,0xa,0xa,0x4,0x4,0x0]),
Font48Data([0xa,0xa,0xa,0xe,0xe,0xe,0xa,0x0]),
Font48Data([0xa,0xa,0x4,0x4,0x4,0xa,0xa,0x0]),
Font48Data([0xa,0xa,0xa,0x4,0x4,0x4,0x4,0x0]),
Font48Data([0xe,0x2,0x4,0x4,0x4,0x8,0xe,0x0]),
Font48Data([0x6,0x4,0x4,0x4,0x4,0x4,0x6,0x0]),
Font48Data([0xa,0x4,0xe,0x4,0xe,0x4,0x4,0x0]),
Font48Data([0xc,0x4,0x4,0x4,0x4,0x4,0xc,0x0]),
Font48Data([0x4,0xa,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0xe,0x0]),
Font48Data([0x4,0x2,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0xc,0x2,0x6,0xa,0x6,0x0]),
Font48Data([0x8,0x8,0xc,0xa,0xa,0xa,0xc,0x0]),
Font48Data([0x0,0x0,0x6,0x8,0x8,0x8,0x6,0x0]),
Font48Data([0x2,0x2,0x6,0xa,0xa,0xa,0x6,0x0]),
Font48Data([0x0,0x0,0x4,0xa,0xe,0x8,0x6,0x0]),
Font48Data([0x6,0x4,0xe,0x4,0x4,0x4,0x4,0x0]),
Font48Data([0x0,0x0,0x6,0xa,0x6,0x2,0xc,0x0]),
Font48Data([0x8,0x8,0xc,0xa,0xa,0xa,0xa,0x0]),
Font48Data([0x4,0x0,0x4,0x4,0x4,0x4,0x4,0x0]),
Font48Data([0x2,0x0,0x2,0x2,0x2,0xa,0x4,0x0]),
Font48Data([0x8,0x8,0xa,0xa,0xc,0xa,0xa,0x0]),
Font48Data([0xc,0x4,0x4,0x4,0x4,0x4,0x4,0x0]),
Font48Data([0x0,0x0,0xc,0xe,0xe,0xe,0xa,0x0]),
Font48Data([0x0,0x0,0xc,0xa,0xa,0xa,0xa,0x0]),
Font48Data([0x0,0x0,0x4,0xa,0xa,0xa,0x4,0x0]),
Font48Data([0x0,0x0,0xc,0xa,0xa,0xc,0x8,0x0]),
Font48Data([0x0,0x0,0x6,0xa,0xa,0x6,0x2,0x0]),
Font48Data([0x0,0x0,0xa,0xc,0x8,0x8,0x8,0x0]),
Font48Data([0x0,0x0,0x6,0x8,0x4,0x2,0xc,0x0]),
Font48Data([0x0,0x4,0xe,0x4,0x4,0x4,0x6,0x0]),
Font48Data([0x0,0x0,0xa,0xa,0xa,0xa,0xe,0x0]),
Font48Data([0x0,0x0,0xa,0xa,0xa,0x4,0x4,0x0]),
Font48Data([0x0,0x0,0xa,0xa,0xe,0xe,0xa,0x0]),
Font48Data([0x0,0x0,0xa,0xa,0x4,0xa,0xa,0x0]),
Font48Data([0x0,0x0,0xa,0xa,0x6,0x2,0xc,0x0]),
Font48Data([0x0,0x0,0xe,0x2,0x4,0x8,0xe,0x0]),
Font48Data([0x2,0x4,0x4,0x8,0x4,0x4,0x2,0x0]),
Font48Data([0x4,0x4,0x4,0x4,0x4,0x4,0x4,0x0]),
Font48Data([0x8,0x4,0x4,0x2,0x4,0x4,0x8,0x0]),
Font48Data([0xe,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x4,0xa,0x4,0x0]),
Font48Data([0xe,0x8,0x8,0x8,0x8,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x2,0x2,0x2,0x2,0xe,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x8,0x4,0x0]),
Font48Data([0x0,0x0,0x0,0x4,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0xe,0x2,0xe,0x2,0x4,0x8,0x0]),
Font48Data([0x0,0x0,0x0,0xe,0x6,0x4,0x8,0x0]),
Font48Data([0x0,0x0,0x0,0x2,0x4,0xc,0x4,0x0]),
Font48Data([0x0,0x0,0x0,0x4,0xe,0xa,0x2,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0xe,0x4,0xe,0x0]),
Font48Data([0x0,0x0,0x0,0x2,0xe,0x6,0xa,0x0]),
Font48Data([0x0,0x0,0x0,0x4,0xe,0x6,0x4,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0xc,0x4,0xe,0x0]),
Font48Data([0x0,0x0,0x0,0xe,0x6,0x2,0xe,0x0]),
Font48Data([0x0,0x0,0x0,0xe,0xe,0x2,0xc,0x0]),
Font48Data([0x0,0x0,0x8,0x6,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0xe,0x2,0x6,0x4,0x4,0x8,0x0]),
Font48Data([0x2,0x2,0x4,0xc,0x4,0x4,0x4,0x0]),
Font48Data([0x4,0xe,0xa,0xa,0x2,0x2,0x4,0x0]),
Font48Data([0x0,0xe,0x4,0x4,0x4,0x4,0xe,0x0]),
Font48Data([0x2,0xe,0x2,0x6,0xa,0x2,0x6,0x0]),
Font48Data([0x4,0xe,0x6,0x6,0xa,0xa,0xa,0x0]),
Font48Data([0x4,0xe,0x4,0x4,0xe,0x4,0x4,0x0]),
Font48Data([0x4,0x6,0xa,0x2,0x2,0x4,0x8,0x0]),
Font48Data([0x8,0xe,0xc,0x4,0x4,0x4,0x8,0x0]),
Font48Data([0x0,0xe,0x2,0x2,0x2,0x2,0xe,0x0]),
Font48Data([0xa,0xe,0xa,0xa,0x2,0x4,0x8,0x0]),
Font48Data([0xc,0x0,0xc,0x0,0x2,0x2,0xc,0x0]),
Font48Data([0x0,0xe,0x2,0x4,0x4,0xa,0xa,0x0]),
Font48Data([0x8,0xe,0xa,0xa,0x8,0x8,0xe,0x0]),
Font48Data([0xa,0x6,0x6,0x2,0x2,0x4,0x8,0x0]),
Font48Data([0x4,0x6,0xa,0x6,0x2,0x4,0x8,0x0]),
Font48Data([0x2,0xc,0x4,0xe,0x4,0x4,0x8,0x0]),
Font48Data([0x0,0xe,0xe,0xe,0x2,0x2,0xc,0x0]),
Font48Data([0x0,0xe,0x0,0xe,0x4,0x4,0x8,0x0]),
Font48Data([0x8,0x8,0x8,0xc,0xa,0x8,0x8,0x0]),
Font48Data([0x4,0x4,0xe,0x4,0x4,0x4,0x8,0x0]),
Font48Data([0x0,0x6,0x0,0x0,0x0,0x0,0xe,0x0]),
Font48Data([0x0,0xe,0x2,0xa,0x4,0x4,0xa,0x0]),
Font48Data([0x4,0xe,0x2,0x4,0xe,0x4,0x4,0x0]),
Font48Data([0x2,0x2,0x2,0x2,0x2,0x4,0x8,0x0]),
Font48Data([0x4,0x2,0xa,0xa,0xa,0xa,0xa,0x0]),
Font48Data([0x8,0x8,0xe,0x8,0x8,0x8,0x6,0x0]),
Font48Data([0x0,0xe,0x2,0x2,0x2,0x4,0x8,0x0]),
Font48Data([0x0,0x4,0x4,0xa,0xa,0x2,0x2,0x0]),
Font48Data([0x4,0xe,0x4,0xe,0xe,0x4,0x4,0x0]),
Font48Data([0x0,0xe,0x2,0x2,0xc,0x4,0x2,0x0]),
Font48Data([0x0,0xe,0x0,0x6,0x0,0x0,0xe,0x0]),
Font48Data([0x4,0x4,0x4,0x8,0xa,0xa,0xe,0x0]),
Font48Data([0x2,0xa,0x4,0x4,0x4,0xa,0x8,0x0]),
Font48Data([0x0,0xe,0x4,0xe,0x4,0x4,0x6,0x0]),
Font48Data([0x4,0xe,0x6,0x6,0x4,0x4,0x4,0x0]),
Font48Data([0x0,0xc,0x4,0x4,0x4,0x4,0xe,0x0]),
Font48Data([0x0,0xe,0x2,0xe,0x2,0x2,0xe,0x0]),
Font48Data([0x0,0xe,0x0,0xe,0x2,0x2,0xc,0x0]),
Font48Data([0xa,0xa,0xa,0xa,0x2,0x2,0x4,0x0]),
Font48Data([0x4,0x4,0xc,0xc,0xc,0xc,0xe,0x0]),
Font48Data([0x8,0x8,0x8,0x8,0x8,0xa,0xc,0x0]),
Font48Data([0x0,0xe,0xa,0xa,0xa,0xa,0xe,0x0]),
Font48Data([0x0,0xe,0xa,0x2,0x2,0x4,0x8,0x0]),
Font48Data([0x0,0xc,0x0,0x0,0x2,0x2,0xc,0x0]),
Font48Data([0xa,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x4,0xa,0x4,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
Font48Data([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]),
] );
