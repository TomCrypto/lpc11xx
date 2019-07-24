#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_CT16B0_CAP0_LOC {
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
#[doc = "Possible values of the field `CT16B0_CAP0LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0_CAP0LOCR {
    #[doc = "Function CT16B0_CAP0 is available for pin PIO0_2"]
    PIO0_2,
    #[doc = "Function CT16B0_CAP0 is available for pin PIO3_3"]
    PIO3_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CT16B0_CAP0LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CT16B0_CAP0LOCR::PIO0_2 => 0,
            CT16B0_CAP0LOCR::PIO3_3 => 1,
            CT16B0_CAP0LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CT16B0_CAP0LOCR {
        match value {
            0 => CT16B0_CAP0LOCR::PIO0_2,
            1 => CT16B0_CAP0LOCR::PIO3_3,
            i => CT16B0_CAP0LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_2`"]
    #[inline]
    pub fn is_pio0_2(&self) -> bool {
        *self == CT16B0_CAP0LOCR::PIO0_2
    }
    #[doc = "Checks if the value of the field is `PIO3_3`"]
    #[inline]
    pub fn is_pio3_3(&self) -> bool {
        *self == CT16B0_CAP0LOCR::PIO3_3
    }
}
#[doc = "Values that can be written to the field `CT16B0_CAP0LOC`"]
pub enum CT16B0_CAP0LOCW {
    #[doc = "Function CT16B0_CAP0 is available for pin PIO0_2"]
    PIO0_2,
    #[doc = "Function CT16B0_CAP0 is available for pin PIO3_3"]
    PIO3_3,
}
impl CT16B0_CAP0LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CT16B0_CAP0LOCW::PIO0_2 => 0,
            CT16B0_CAP0LOCW::PIO3_3 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT16B0_CAP0LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CT16B0_CAP0LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT16B0_CAP0LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Function CT16B0_CAP0 is available for pin PIO0_2"]
    #[inline]
    pub fn pio0_2(self) -> &'a mut W {
        self.variant(CT16B0_CAP0LOCW::PIO0_2)
    }
    #[doc = "Function CT16B0_CAP0 is available for pin PIO3_3"]
    #[inline]
    pub fn pio3_3(self) -> &'a mut W {
        self.variant(CT16B0_CAP0LOCW::PIO3_3)
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
    #[doc = "Bits 0:1 - Selects pin location for CT16B0_CAP0 function"]
    #[inline]
    pub fn ct16b0_cap0loc(&self) -> CT16B0_CAP0LOCR {
        CT16B0_CAP0LOCR::_from({
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
    #[doc = "Bits 0:1 - Selects pin location for CT16B0_CAP0 function"]
    #[inline]
    pub fn ct16b0_cap0loc(&mut self) -> _CT16B0_CAP0LOCW {
        _CT16B0_CAP0LOCW { w: self }
    }
}
