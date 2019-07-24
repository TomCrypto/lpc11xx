#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_RXD_LOC {
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
#[doc = "Possible values of the field `RXDLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDLOCR {
    #[doc = "Function RXD is available for pin PIO1_6"]
    PIO1_6,
    #[doc = "Function RXD is available for pin PIO2_7"]
    PIO2_7,
    #[doc = "Function RXD is available for pin PIO3_1"]
    PIO3_1,
    #[doc = "Function RXD is available for pin PIO3_4"]
    PIO3_4,
}
impl RXDLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXDLOCR::PIO1_6 => 0,
            RXDLOCR::PIO2_7 => 1,
            RXDLOCR::PIO3_1 => 2,
            RXDLOCR::PIO3_4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXDLOCR {
        match value {
            0 => RXDLOCR::PIO1_6,
            1 => RXDLOCR::PIO2_7,
            2 => RXDLOCR::PIO3_1,
            3 => RXDLOCR::PIO3_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIO1_6`"]
    #[inline]
    pub fn is_pio1_6(&self) -> bool {
        *self == RXDLOCR::PIO1_6
    }
    #[doc = "Checks if the value of the field is `PIO2_7`"]
    #[inline]
    pub fn is_pio2_7(&self) -> bool {
        *self == RXDLOCR::PIO2_7
    }
    #[doc = "Checks if the value of the field is `PIO3_1`"]
    #[inline]
    pub fn is_pio3_1(&self) -> bool {
        *self == RXDLOCR::PIO3_1
    }
    #[doc = "Checks if the value of the field is `PIO3_4`"]
    #[inline]
    pub fn is_pio3_4(&self) -> bool {
        *self == RXDLOCR::PIO3_4
    }
}
#[doc = "Values that can be written to the field `RXDLOC`"]
pub enum RXDLOCW {
    #[doc = "Function RXD is available for pin PIO1_6"]
    PIO1_6,
    #[doc = "Function RXD is available for pin PIO2_7"]
    PIO2_7,
    #[doc = "Function RXD is available for pin PIO3_1"]
    PIO3_1,
    #[doc = "Function RXD is available for pin PIO3_4"]
    PIO3_4,
}
impl RXDLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXDLOCW::PIO1_6 => 0,
            RXDLOCW::PIO2_7 => 1,
            RXDLOCW::PIO3_1 => 2,
            RXDLOCW::PIO3_4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDLOCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Function RXD is available for pin PIO1_6"]
    #[inline]
    pub fn pio1_6(self) -> &'a mut W {
        self.variant(RXDLOCW::PIO1_6)
    }
    #[doc = "Function RXD is available for pin PIO2_7"]
    #[inline]
    pub fn pio2_7(self) -> &'a mut W {
        self.variant(RXDLOCW::PIO2_7)
    }
    #[doc = "Function RXD is available for pin PIO3_1"]
    #[inline]
    pub fn pio3_1(self) -> &'a mut W {
        self.variant(RXDLOCW::PIO3_1)
    }
    #[doc = "Function RXD is available for pin PIO3_4"]
    #[inline]
    pub fn pio3_4(self) -> &'a mut W {
        self.variant(RXDLOCW::PIO3_4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Selects pin location for the RXD function"]
    #[inline]
    pub fn rxdloc(&self) -> RXDLOCR {
        RXDLOCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Selects pin location for the RXD function"]
    #[inline]
    pub fn rxdloc(&mut self) -> _RXDLOCW {
        _RXDLOCW { w: self }
    }
}
