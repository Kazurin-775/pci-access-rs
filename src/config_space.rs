use crate::{enumerable::Enumerable, types::ConfigAddr};

pub trait ConfigSpace {
    unsafe fn read_unchecked(&mut self, addr: u32) -> u32;
    unsafe fn write_unchecked(&mut self, addr: u32, value: u32);

    fn read_u32(&mut self, addr: ConfigAddr) -> u32 {
        addr.enforce_aligned();
        unsafe { self.read_unchecked(addr.as_u32()) }
    }

    fn write_u32(&mut self, addr: ConfigAddr, value: u32) {
        addr.enforce_aligned();
        unsafe { self.write_unchecked(addr.as_u32(), value) }
    }

    fn read_u8(&mut self, addr: ConfigAddr) -> u8 {
        let u32_data = unsafe { self.read_unchecked(addr.u32_aligned().as_u32()) };
        (match addr.byte_offset() {
            0 => u32_data & 0xFF,
            1 => (u32_data >> 8) & 0xFF,
            2 => (u32_data >> 16) & 0xFF,
            3 => (u32_data >> 24) & 0xFF,
            _ => unreachable!(),
        }) as u8
    }

    fn read_u16(&mut self, addr: ConfigAddr) -> u16 {
        let u32_data = unsafe { self.read_unchecked(addr.u32_aligned().as_u32()) };
        (match addr.byte_offset() {
            0 => u32_data & 0xFFFF,
            2 => (u32_data >> 16) & 0xFFFF,
            1 | 3 => panic!("address {:?} is not aligned to u16 boundary", addr),
            _ => unreachable!(),
        }) as u16
    }

    fn enumerate_all(&mut self, mut callback: impl FnMut(ConfigAddr, &mut Self))
    where
        Self: Sized,
    {
        Enumerable::new(self).enumerate_all(|addr, inner| callback(addr, inner));
    }
}
