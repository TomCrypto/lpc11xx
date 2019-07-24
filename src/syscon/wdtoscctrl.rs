#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WDTOSCCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct DIVSELR {
    bits: u8,
}
impl DIVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FREQSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQSELR {
    #[doc = "0.5 MHz"]
    _0_5_MHZ,
    #[doc = "0.8 MHz"]
    _0_8_MHZ,
    #[doc = "1.1 MHz"]
    _1_1_MHZ,
    #[doc = "1.4 MHz"]
    _1_4_MHZ,
    #[doc = "1.6 MHz"]
    _1_6_MHZ,
    #[doc = "1.8 MHz"]
    _1_8_MHZ,
    #[doc = "2.0 MHz"]
    _2_0_MHZ,
    #[doc = "2.2 MHz"]
    _2_2_MHZ,
    #[doc = "2.4 MHz"]
    _2_4_MHZ,
    #[doc = "2.6 MHz"]
    _2_6_MHZ,
    #[doc = "2.7 MHz"]
    _2_7_MHZ,
    #[doc = "2.9 MHz"]
    _2_9_MHZ,
    #[doc = "3.1 MHz"]
    _3_1_MHZ,
    #[doc = "3.2 MHz"]
    _3_2_MHZ,
    #[doc = "3.4 MHz"]
    _3_4_MHZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FREQSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FREQSELR::_0_5_MHZ => 1,
            FREQSELR::_0_8_MHZ => 2,
            FREQSELR::_1_1_MHZ => 3,
            FREQSELR::_1_4_MHZ => 4,
            FREQSELR::_1_6_MHZ => 5,
            FREQSELR::_1_8_MHZ => 6,
            FREQSELR::_2_0_MHZ => 7,
            FREQSELR::_2_2_MHZ => 8,
            FREQSELR::_2_4_MHZ => 9,
            FREQSELR::_2_6_MHZ => 10,
            FREQSELR::_2_7_MHZ => 11,
            FREQSELR::_2_9_MHZ => 12,
            FREQSELR::_3_1_MHZ => 13,
            FREQSELR::_3_2_MHZ => 14,
            FREQSELR::_3_4_MHZ => 15,
            FREQSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FREQSELR {
        match value {
            1 => FREQSELR::_0_5_MHZ,
            2 => FREQSELR::_0_8_MHZ,
            3 => FREQSELR::_1_1_MHZ,
            4 => FREQSELR::_1_4_MHZ,
            5 => FREQSELR::_1_6_MHZ,
            6 => FREQSELR::_1_8_MHZ,
            7 => FREQSELR::_2_0_MHZ,
            8 => FREQSELR::_2_2_MHZ,
            9 => FREQSELR::_2_4_MHZ,
            10 => FREQSELR::_2_6_MHZ,
            11 => FREQSELR::_2_7_MHZ,
            12 => FREQSELR::_2_9_MHZ,
            13 => FREQSELR::_3_1_MHZ,
            14 => FREQSELR::_3_2_MHZ,
            15 => FREQSELR::_3_4_MHZ,
            i => FREQSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_5_MHZ`"]
    #[inline]
    pub fn is_0_5_mhz(&self) -> bool {
        *self == FREQSELR::_0_5_MHZ
    }
    #[doc = "Checks if the value of the field is `_0_8_MHZ`"]
    #[inline]
    pub fn is_0_8_mhz(&self) -> bool {
        *self == FREQSELR::_0_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_1_MHZ`"]
    #[inline]
    pub fn is_1_1_mhz(&self) -> bool {
        *self == FREQSELR::_1_1_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_4_MHZ`"]
    #[inline]
    pub fn is_1_4_mhz(&self) -> bool {
        *self == FREQSELR::_1_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_6_MHZ`"]
    #[inline]
    pub fn is_1_6_mhz(&self) -> bool {
        *self == FREQSELR::_1_6_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_8_MHZ`"]
    #[inline]
    pub fn is_1_8_mhz(&self) -> bool {
        *self == FREQSELR::_1_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_0_MHZ`"]
    #[inline]
    pub fn is_2_0_mhz(&self) -> bool {
        *self == FREQSELR::_2_0_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_2_MHZ`"]
    #[inline]
    pub fn is_2_2_mhz(&self) -> bool {
        *self == FREQSELR::_2_2_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_4_MHZ`"]
    #[inline]
    pub fn is_2_4_mhz(&self) -> bool {
        *self == FREQSELR::_2_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_6_MHZ`"]
    #[inline]
    pub fn is_2_6_mhz(&self) -> bool {
        *self == FREQSELR::_2_6_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_7_MHZ`"]
    #[inline]
    pub fn is_2_7_mhz(&self) -> bool {
        *self == FREQSELR::_2_7_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_9_MHZ`"]
    #[inline]
    pub fn is_2_9_mhz(&self) -> bool {
        *self == FREQSELR::_2_9_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_1_MHZ`"]
    #[inline]
    pub fn is_3_1_mhz(&self) -> bool {
        *self == FREQSELR::_3_1_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_2_MHZ`"]
    #[inline]
    pub fn is_3_2_mhz(&self) -> bool {
        *self == FREQSELR::_3_2_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_4_MHZ`"]
    #[inline]
    pub fn is_3_4_mhz(&self) -> bool {
        *self == FREQSELR::_3_4_MHZ
    }
}
#[doc = r" Proxy"]
pub struct _DIVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FREQSEL`"]
pub enum FREQSELW {
    #[doc = "0.5 MHz"]
    _0_5_MHZ,
    #[doc = "0.8 MHz"]
    _0_8_MHZ,
    #[doc = "1.1 MHz"]
    _1_1_MHZ,
    #[doc = "1.4 MHz"]
    _1_4_MHZ,
    #[doc = "1.6 MHz"]
    _1_6_MHZ,
    #[doc = "1.8 MHz"]
    _1_8_MHZ,
    #[doc = "2.0 MHz"]
    _2_0_MHZ,
    #[doc = "2.2 MHz"]
    _2_2_MHZ,
    #[doc = "2.4 MHz"]
    _2_4_MHZ,
    #[doc = "2.6 MHz"]
    _2_6_MHZ,
    #[doc = "2.7 MHz"]
    _2_7_MHZ,
    #[doc = "2.9 MHz"]
    _2_9_MHZ,
    #[doc = "3.1 MHz"]
    _3_1_MHZ,
    #[doc = "3.2 MHz"]
    _3_2_MHZ,
    #[doc = "3.4 MHz"]
    _3_4_MHZ,
}
impl FREQSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FREQSELW::_0_5_MHZ => 1,
            FREQSELW::_0_8_MHZ => 2,
            FREQSELW::_1_1_MHZ => 3,
            FREQSELW::_1_4_MHZ => 4,
            FREQSELW::_1_6_MHZ => 5,
            FREQSELW::_1_8_MHZ => 6,
            FREQSELW::_2_0_MHZ => 7,
            FREQSELW::_2_2_MHZ => 8,
            FREQSELW::_2_4_MHZ => 9,
            FREQSELW::_2_6_MHZ => 10,
            FREQSELW::_2_7_MHZ => 11,
            FREQSELW::_2_9_MHZ => 12,
            FREQSELW::_3_1_MHZ => 13,
            FREQSELW::_3_2_MHZ => 14,
            FREQSELW::_3_4_MHZ => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0.5 MHz"]
    #[inline]
    pub fn _0_5_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_0_5_MHZ)
    }
    #[doc = "0.8 MHz"]
    #[inline]
    pub fn _0_8_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_0_8_MHZ)
    }
    #[doc = "1.1 MHz"]
    #[inline]
    pub fn _1_1_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_1_1_MHZ)
    }
    #[doc = "1.4 MHz"]
    #[inline]
    pub fn _1_4_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_1_4_MHZ)
    }
    #[doc = "1.6 MHz"]
    #[inline]
    pub fn _1_6_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_1_6_MHZ)
    }
    #[doc = "1.8 MHz"]
    #[inline]
    pub fn _1_8_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_1_8_MHZ)
    }
    #[doc = "2.0 MHz"]
    #[inline]
    pub fn _2_0_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_2_0_MHZ)
    }
    #[doc = "2.2 MHz"]
    #[inline]
    pub fn _2_2_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_2_2_MHZ)
    }
    #[doc = "2.4 MHz"]
    #[inline]
    pub fn _2_4_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_2_4_MHZ)
    }
    #[doc = "2.6 MHz"]
    #[inline]
    pub fn _2_6_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_2_6_MHZ)
    }
    #[doc = "2.7 MHz"]
    #[inline]
    pub fn _2_7_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_2_7_MHZ)
    }
    #[doc = "2.9 MHz"]
    #[inline]
    pub fn _2_9_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_2_9_MHZ)
    }
    #[doc = "3.1 MHz"]
    #[inline]
    pub fn _3_1_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_3_1_MHZ)
    }
    #[doc = "3.2 MHz"]
    #[inline]
    pub fn _3_2_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_3_2_MHZ)
    }
    #[doc = "3.4 MHz"]
    #[inline]
    pub fn _3_4_mhz(self) -> &'a mut W {
        self.variant(FREQSELW::_3_4_MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline]
    pub fn divsel(&self) -> DIVSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVSELR { bits }
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)"]
    #[inline]
    pub fn freqsel(&self) -> FREQSELR {
        FREQSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline]
    pub fn divsel(&mut self) -> _DIVSELW {
        _DIVSELW { w: self }
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)"]
    #[inline]
    pub fn freqsel(&mut self) -> _FREQSELW {
        _FREQSELW { w: self }
    }
}
