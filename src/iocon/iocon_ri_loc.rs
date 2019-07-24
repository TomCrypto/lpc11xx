#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_RI_LOC {
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
#[doc = "Possible values of the field `RILOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RILOCR {
    #[doc = "Function RI is available for pin PIO2_3"]
    PIO2_3,
    #[doc = "Function RI is available for pin PIO3_3"]
    PIO3_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RILOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RILOCR::PIO2_3 => 0,
            RILOCR::PIO3_3 => 1,
            RILOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RILOCR {
        match value {
            0 => RILOCR::PIO2_3,
            1 => RILOCR::PIO3_3,
            i => RILOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_3`"]
    #[inline]
    pub fn is_pio2_3(&self) -> bool {
        *self == RILOCR::PIO2_3
    }
    #[doc = "Checks if the value of the field is `PIO3_3`"]
    #[inline]
    pub fn is_pio3_3(&self) -> bool {
        *self == RILOCR::PIO3_3
    }
}
#[doc = "Values that can be written to the field `RILOC`"]
pub enum RILOCW {
    #[doc = "Function RI is available for pin PIO2_3"]
    PIO2_3,
    #[doc = "Function RI is available for pin PIO3_3"]
    PIO3_3,
}
impl RILOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RILOCW::PIO2_3 => 0,
            RILOCW::PIO3_3 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RILOCW<'a> {
    w: &'a mut W,
}
impl<'a> _RILOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RILOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Function RI is available for pin PIO2_3"]
    #[inline]
    pub fn pio2_3(self) -> &'a mut W {
        self.variant(RILOCW::PIO2_3)
    }
    #[doc = "Function RI is available for pin PIO3_3"]
    #[inline]
    pub fn pio3_3(self) -> &'a mut W {
        self.variant(RILOCW::PIO3_3)
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
    #[doc = "Bits 0:1 - Selects pin location for RI function"]
    #[inline]
    pub fn riloc(&self) -> RILOCR {
        RILOCR::_from({
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
    #[doc = "Bits 0:1 - Selects pin location for RI function"]
    #[inline]
    pub fn riloc(&mut self) -> _RILOCW {
        _RILOCW { w: self }
    }
}
