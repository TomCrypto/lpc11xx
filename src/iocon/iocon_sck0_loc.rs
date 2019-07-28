#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_SCK0_LOC {
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
#[doc = "Possible values of the field `SCKLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKLOCR {
    #[doc = "Function SCK0 is available for pin PIO0_10"]
    PIO0_10,
    #[doc = "Function SCK0 is available for pin PIO2_11"]
    PIO2_11,
    #[doc = "Function SCK0 is available for pin PIO0_6"]
    PIO0_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCKLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCKLOCR::PIO0_10 => 0,
            SCKLOCR::PIO2_11 => 1,
            SCKLOCR::PIO0_6 => 2,
            SCKLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCKLOCR {
        match value {
            0 => SCKLOCR::PIO0_10,
            1 => SCKLOCR::PIO2_11,
            2 => SCKLOCR::PIO0_6,
            i => SCKLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_10`"]
    #[inline]
    pub fn is_pio0_10(&self) -> bool {
        *self == SCKLOCR::PIO0_10
    }
    #[doc = "Checks if the value of the field is `PIO2_11`"]
    #[inline]
    pub fn is_pio2_11(&self) -> bool {
        *self == SCKLOCR::PIO2_11
    }
    #[doc = "Checks if the value of the field is `PIO0_6`"]
    #[inline]
    pub fn is_pio0_6(&self) -> bool {
        *self == SCKLOCR::PIO0_6
    }
}
#[doc = "Values that can be written to the field `SCKLOC`"]
pub enum SCKLOCW {
    #[doc = "Function SCK0 is available for pin PIO0_10"]
    PIO0_10,
    #[doc = "Function SCK0 is available for pin PIO2_11"]
    PIO2_11,
    #[doc = "Function SCK0 is available for pin PIO0_6"]
    PIO0_6,
}
impl SCKLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCKLOCW::PIO0_10 => 0,
            SCKLOCW::PIO2_11 => 1,
            SCKLOCW::PIO0_6 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCKLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCKLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Function SCK0 is available for pin PIO0_10"]
    #[inline]
    pub fn pio0_10(self) -> &'a mut W {
        self.variant(SCKLOCW::PIO0_10)
    }
    #[doc = "Function SCK0 is available for pin PIO2_11"]
    #[inline]
    pub fn pio2_11(self) -> &'a mut W {
        self.variant(SCKLOCW::PIO2_11)
    }
    #[doc = "Function SCK0 is available for pin PIO0_6"]
    #[inline]
    pub fn pio0_6(self) -> &'a mut W {
        self.variant(SCKLOCW::PIO0_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Selects pin location for SCK0 function."]
    #[inline]
    pub fn sckloc(&self) -> SCKLOCR {
        SCKLOCR::_from({
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
    #[doc = "Bits 0:1 - Selects pin location for SCK0 function."]
    #[inline]
    pub fn sckloc(&mut self) -> _SCKLOCW {
        _SCKLOCW { w: self }
    }
}
