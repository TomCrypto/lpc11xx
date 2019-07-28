#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_SSEL1_LOC {
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
#[doc = "Possible values of the field `SSEL1LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEL1LOCR {
    #[doc = "Function SSEL1 is available for pin PIO2_2"]
    PIO2_2,
    #[doc = "Function SSEL1 is available for pin PIO2_4"]
    PIO2_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSEL1LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSEL1LOCR::PIO2_2 => 0,
            SSEL1LOCR::PIO2_4 => 1,
            SSEL1LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSEL1LOCR {
        match value {
            0 => SSEL1LOCR::PIO2_2,
            1 => SSEL1LOCR::PIO2_4,
            i => SSEL1LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_2`"]
    #[inline]
    pub fn is_pio2_2(&self) -> bool {
        *self == SSEL1LOCR::PIO2_2
    }
    #[doc = "Checks if the value of the field is `PIO2_4`"]
    #[inline]
    pub fn is_pio2_4(&self) -> bool {
        *self == SSEL1LOCR::PIO2_4
    }
}
#[doc = "Values that can be written to the field `SSEL1LOC`"]
pub enum SSEL1LOCW {
    #[doc = "Function SSEL1 is available for pin PIO2_2"]
    PIO2_2,
    #[doc = "Function SSEL1 is available for pin PIO2_4"]
    PIO2_4,
}
impl SSEL1LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSEL1LOCW::PIO2_2 => 0,
            SSEL1LOCW::PIO2_4 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSEL1LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SSEL1LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSEL1LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Function SSEL1 is available for pin PIO2_2"]
    #[inline]
    pub fn pio2_2(self) -> &'a mut W {
        self.variant(SSEL1LOCW::PIO2_2)
    }
    #[doc = "Function SSEL1 is available for pin PIO2_4"]
    #[inline]
    pub fn pio2_4(self) -> &'a mut W {
        self.variant(SSEL1LOCW::PIO2_4)
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
    #[doc = "Bits 0:1 - Selects pin location for SSEL1 function."]
    #[inline]
    pub fn ssel1loc(&self) -> SSEL1LOCR {
        SSEL1LOCR::_from({
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
    #[doc = "Bits 0:1 - Selects pin location for SSEL1 function."]
    #[inline]
    pub fn ssel1loc(&mut self) -> _SSEL1LOCW {
        _SSEL1LOCW { w: self }
    }
}
