#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CANIF_MSK1 {
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
#[doc = "Possible values of the field `MSK_15_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSK_15_0R {
    #[doc = "The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    NOINHIBIT,
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl MSK_15_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            MSK_15_0R::NOINHIBIT => 0,
            MSK_15_0R::ACCEPTANCEFILTERING => 1,
            MSK_15_0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> MSK_15_0R {
        match value {
            0 => MSK_15_0R::NOINHIBIT,
            1 => MSK_15_0R::ACCEPTANCEFILTERING,
            i => MSK_15_0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOINHIBIT`"]
    #[inline]
    pub fn is_noinhibit(&self) -> bool {
        *self == MSK_15_0R::NOINHIBIT
    }
    #[doc = "Checks if the value of the field is `ACCEPTANCEFILTERING`"]
    #[inline]
    pub fn is_acceptancefiltering(&self) -> bool {
        *self == MSK_15_0R::ACCEPTANCEFILTERING
    }
}
#[doc = "Values that can be written to the field `MSK_15_0`"]
pub enum MSK_15_0W {
    #[doc = "The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    NOINHIBIT,
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl MSK_15_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            MSK_15_0W::NOINHIBIT => 0,
            MSK_15_0W::ACCEPTANCEFILTERING => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSK_15_0W<'a> {
    w: &'a mut W,
}
impl<'a> _MSK_15_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSK_15_0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    #[inline]
    pub fn noinhibit(self) -> &'a mut W {
        self.variant(MSK_15_0W::NOINHIBIT)
    }
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    #[inline]
    pub fn acceptancefiltering(self) -> &'a mut W {
        self.variant(MSK_15_0W::ACCEPTANCEFILTERING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - Identifier mask."]
    #[inline]
    pub fn msk_15_0(&self) -> MSK_15_0R {
        MSK_15_0R::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Identifier mask."]
    #[inline]
    pub fn msk_15_0(&mut self) -> _MSK_15_0W {
        _MSK_15_0W { w: self }
    }
}
