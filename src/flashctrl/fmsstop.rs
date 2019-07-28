#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FMSSTOP {
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
pub struct STOPR {
    bits: u32,
}
impl STOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `SIG_START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIG_STARTR {
    #[doc = "Signature generation is stopped"]
    SIGNATURE_GENERATION,
    #[doc = "Initiate signature generation"]
    INITIATE_SIGNATURE_G,
}
impl SIG_STARTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SIG_STARTR::SIGNATURE_GENERATION => false,
            SIG_STARTR::INITIATE_SIGNATURE_G => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIG_STARTR {
        match value {
            false => SIG_STARTR::SIGNATURE_GENERATION,
            true => SIG_STARTR::INITIATE_SIGNATURE_G,
        }
    }
    #[doc = "Checks if the value of the field is `SIGNATURE_GENERATION`"]
    #[inline]
    pub fn is_signature_generation(&self) -> bool {
        *self == SIG_STARTR::SIGNATURE_GENERATION
    }
    #[doc = "Checks if the value of the field is `INITIATE_SIGNATURE_G`"]
    #[inline]
    pub fn is_initiate_signature_g(&self) -> bool {
        *self == SIG_STARTR::INITIATE_SIGNATURE_G
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 131071;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIG_START`"]
pub enum SIG_STARTW {
    #[doc = "Signature generation is stopped"]
    SIGNATURE_GENERATION,
    #[doc = "Initiate signature generation"]
    INITIATE_SIGNATURE_G,
}
impl SIG_STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIG_STARTW::SIGNATURE_GENERATION => false,
            SIG_STARTW::INITIATE_SIGNATURE_G => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIG_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SIG_STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIG_STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Signature generation is stopped"]
    #[inline]
    pub fn signature_generation(self) -> &'a mut W {
        self.variant(SIG_STARTW::SIGNATURE_GENERATION)
    }
    #[doc = "Initiate signature generation"]
    #[inline]
    pub fn initiate_signature_g(self) -> &'a mut W {
        self.variant(SIG_STARTW::INITIATE_SIGNATURE_G)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline]
    pub fn stop(&self) -> STOPR {
        let bits = {
            const MASK: u32 = 131071;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        STOPR { bits }
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline]
    pub fn sig_start(&self) -> SIG_STARTR {
        SIG_STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline]
    pub fn sig_start(&mut self) -> _SIG_STARTW {
        _SIG_STARTW { w: self }
    }
}
