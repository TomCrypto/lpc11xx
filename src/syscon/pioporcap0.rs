#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIOPORCAP0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CAPPIO0_NR {
    bits: u16,
}
impl CAPPIO0_NR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAPPIO1_NR {
    bits: u16,
}
impl CAPPIO1_NR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAPPIO2_NR {
    bits: u8,
}
impl CAPPIO2_NR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:11 - Raw reset status input PIO0_n: PIO0_11 to PIO0_0"]
    #[inline]
    pub fn cappio0_n(&self) -> CAPPIO0_NR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CAPPIO0_NR { bits }
    }
    #[doc = "Bits 12:23 - Raw reset status input PIO1_n: PIO1_11 to PIO1_0"]
    #[inline]
    pub fn cappio1_n(&self) -> CAPPIO1_NR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CAPPIO1_NR { bits }
    }
    #[doc = "Bits 24:31 - Raw reset status input PIO2_n: PIO2_7 to PIO2_0"]
    #[inline]
    pub fn cappio2_n(&self) -> CAPPIO2_NR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAPPIO2_NR { bits }
    }
}
