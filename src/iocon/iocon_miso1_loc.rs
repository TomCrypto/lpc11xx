#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_MISO1_LOC {
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
#[doc = "Possible values of the field `MISO1LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISO1LOCR {
    #[doc = "Function MISO1 is available for pin PIO2_2"]
    PIO2_2,
    #[doc = "Function MISO1 is available for pin PIO1_10"]
    PIO1_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MISO1LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MISO1LOCR::PIO2_2 => 0,
            MISO1LOCR::PIO1_10 => 1,
            MISO1LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MISO1LOCR {
        match value {
            0 => MISO1LOCR::PIO2_2,
            1 => MISO1LOCR::PIO1_10,
            i => MISO1LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_2`"]
    #[inline]
    pub fn is_pio2_2(&self) -> bool {
        *self == MISO1LOCR::PIO2_2
    }
    #[doc = "Checks if the value of the field is `PIO1_10`"]
    #[inline]
    pub fn is_pio1_10(&self) -> bool {
        *self == MISO1LOCR::PIO1_10
    }
}
#[doc = "Values that can be written to the field `MISO1LOC`"]
pub enum MISO1LOCW {
    #[doc = "Function MISO1 is available for pin PIO2_2"]
    PIO2_2,
    #[doc = "Function MISO1 is available for pin PIO1_10"]
    PIO1_10,
}
impl MISO1LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MISO1LOCW::PIO2_2 => 0,
            MISO1LOCW::PIO1_10 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MISO1LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _MISO1LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MISO1LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Function MISO1 is available for pin PIO2_2"]
    #[inline]
    pub fn pio2_2(self) -> &'a mut W {
        self.variant(MISO1LOCW::PIO2_2)
    }
    #[doc = "Function MISO1 is available for pin PIO1_10"]
    #[inline]
    pub fn pio1_10(self) -> &'a mut W {
        self.variant(MISO1LOCW::PIO1_10)
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
    #[doc = "Bits 0:1 - Selects pin location for the MISO1 function"]
    #[inline]
    pub fn miso1loc(&self) -> MISO1LOCR {
        MISO1LOCR::_from({
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
    #[doc = "Bits 0:1 - Selects pin location for the MISO1 function"]
    #[inline]
    pub fn miso1loc(&mut self) -> _MISO1LOCW {
        _MISO1LOCW { w: self }
    }
}
