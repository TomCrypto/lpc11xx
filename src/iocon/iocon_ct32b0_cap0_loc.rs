#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_CT32B0_CAP0_LOC {
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
#[doc = "Possible values of the field `CT32B0_CAP0LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0_CAP0LOCR {
    #[doc = "Function CT32B0_CAP0 is available for pin PIO1_5"]
    PIO1_5,
    #[doc = "Function CT32B0_CAP0 is available for pin PIO2_9"]
    PIO2_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CT32B0_CAP0LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CT32B0_CAP0LOCR::PIO1_5 => 0,
            CT32B0_CAP0LOCR::PIO2_9 => 1,
            CT32B0_CAP0LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CT32B0_CAP0LOCR {
        match value {
            0 => CT32B0_CAP0LOCR::PIO1_5,
            1 => CT32B0_CAP0LOCR::PIO2_9,
            i => CT32B0_CAP0LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO1_5`"]
    #[inline]
    pub fn is_pio1_5(&self) -> bool {
        *self == CT32B0_CAP0LOCR::PIO1_5
    }
    #[doc = "Checks if the value of the field is `PIO2_9`"]
    #[inline]
    pub fn is_pio2_9(&self) -> bool {
        *self == CT32B0_CAP0LOCR::PIO2_9
    }
}
#[doc = "Values that can be written to the field `CT32B0_CAP0LOC`"]
pub enum CT32B0_CAP0LOCW {
    #[doc = "Function CT32B0_CAP0 is available for pin PIO1_5"]
    PIO1_5,
    #[doc = "Function CT32B0_CAP0 is available for pin PIO2_9"]
    PIO2_9,
}
impl CT32B0_CAP0LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CT32B0_CAP0LOCW::PIO1_5 => 0,
            CT32B0_CAP0LOCW::PIO2_9 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT32B0_CAP0LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CT32B0_CAP0LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT32B0_CAP0LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Function CT32B0_CAP0 is available for pin PIO1_5"]
    #[inline]
    pub fn pio1_5(self) -> &'a mut W {
        self.variant(CT32B0_CAP0LOCW::PIO1_5)
    }
    #[doc = "Function CT32B0_CAP0 is available for pin PIO2_9"]
    #[inline]
    pub fn pio2_9(self) -> &'a mut W {
        self.variant(CT32B0_CAP0LOCW::PIO2_9)
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
    #[doc = "Bits 0:1 - Selects pin location for the CT32B0_CAP0 function."]
    #[inline]
    pub fn ct32b0_cap0loc(&self) -> CT32B0_CAP0LOCR {
        CT32B0_CAP0LOCR::_from({
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
    #[doc = "Bits 0:1 - Selects pin location for the CT32B0_CAP0 function."]
    #[inline]
    pub fn ct32b0_cap0loc(&mut self) -> _CT32B0_CAP0LOCW {
        _CT32B0_CAP0LOCW { w: self }
    }
}
