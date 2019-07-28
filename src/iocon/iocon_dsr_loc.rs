#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_DSR_LOC {
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
#[doc = "Possible values of the field `DSRLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRLOCR {
    #[doc = "Function DSR is available for pin PIO2_1"]
    PIO2_1,
    #[doc = "Function DSR is available for pin PIO3_1"]
    PIO3_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSRLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSRLOCR::PIO2_1 => 0,
            DSRLOCR::PIO3_1 => 1,
            DSRLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSRLOCR {
        match value {
            0 => DSRLOCR::PIO2_1,
            1 => DSRLOCR::PIO3_1,
            i => DSRLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_1`"]
    #[inline]
    pub fn is_pio2_1(&self) -> bool {
        *self == DSRLOCR::PIO2_1
    }
    #[doc = "Checks if the value of the field is `PIO3_1`"]
    #[inline]
    pub fn is_pio3_1(&self) -> bool {
        *self == DSRLOCR::PIO3_1
    }
}
#[doc = "Values that can be written to the field `DSRLOC`"]
pub enum DSRLOCW {
    #[doc = "Function DSR is available for pin PIO2_1"]
    PIO2_1,
    #[doc = "Function DSR is available for pin PIO3_1"]
    PIO3_1,
}
impl DSRLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSRLOCW::PIO2_1 => 0,
            DSRLOCW::PIO3_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSRLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSRLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSRLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Function DSR is available for pin PIO2_1"]
    #[inline]
    pub fn pio2_1(self) -> &'a mut W {
        self.variant(DSRLOCW::PIO2_1)
    }
    #[doc = "Function DSR is available for pin PIO3_1"]
    #[inline]
    pub fn pio3_1(self) -> &'a mut W {
        self.variant(DSRLOCW::PIO3_1)
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
    #[doc = "Bits 0:1 - Selects pin location for DSR function."]
    #[inline]
    pub fn dsrloc(&self) -> DSRLOCR {
        DSRLOCR::_from({
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
    #[doc = "Bits 0:1 - Selects pin location for DSR function."]
    #[inline]
    pub fn dsrloc(&mut self) -> _DSRLOCW {
        _DSRLOCW { w: self }
    }
}
