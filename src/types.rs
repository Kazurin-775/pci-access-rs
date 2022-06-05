use modular_bitfield::prelude::*;

#[modular_bitfield::bitfield]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfigAddr {
    pub offset: B8,
    pub func: B3,
    pub dev: B5,
    pub bus: B8,
    #[skip]
    reserved: B7,
    pub enable: bool,
}

impl ConfigAddr {
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        u32::from_le_bytes(self.bytes)
    }

    #[inline]
    pub fn enforce_aligned(&self) {
        assert_eq!(self.offset() & 3, 0);
    }

    #[inline]
    pub fn byte_offset(&self) -> u8 {
        self.offset() & 3
    }

    #[inline]
    pub fn u32_aligned(self) -> Self {
        self.with_offset(self.offset() & 0xFC)
    }
}

#[test]
fn it_works() {
    let addr = ConfigAddr::new()
        .with_bus(2)
        .with_dev(3)
        .with_func(4)
        .with_offset(0x42)
        .with_enable(true);
    assert_eq!(addr.as_u32(), 0x8002_1C42);
}
