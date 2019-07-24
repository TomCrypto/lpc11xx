#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CANIF_MSK2 {
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
#[doc = "Possible values of the field `MSK_28_16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSK_28_16R {
    #[doc = "The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    NOINHIBIT,
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl MSK_28_16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            MSK_28_16R::NOINHIBIT => 0,
            MSK_28_16R::ACCEPTANCEFILTERING => 1,
            MSK_28_16R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> MSK_28_16R {
        match value {
            0 => MSK_28_16R::NOINHIBIT,
            1 => MSK_28_16R::ACCEPTANCEFILTERING,
            i => MSK_28_16R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOINHIBIT`"]
    #[inline]
    pub fn is_noinhibit(&self) -> bool {
        *self == MSK_28_16R::NOINHIBIT
    }
    #[doc = "Checks if the value of the field is `ACCEPTANCEFILTERING`"]
    #[inline]
    pub fn is_acceptancefiltering(&self) -> bool {
        *self == MSK_28_16R::ACCEPTANCEFILTERING
    }
}
#[doc = "Possible values of the field `MDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIRR {
    #[doc = "The message direction bit (DIR) has no effect on acceptance filtering"]
    NOEFFECT,
    #[doc = "The message direction bit (DIR) is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl MDIRR {
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
            MDIRR::NOEFFECT => false,
            MDIRR::ACCEPTANCEFILTERING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDIRR {
        match value {
            false => MDIRR::NOEFFECT,
            true => MDIRR::ACCEPTANCEFILTERING,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline]
    pub fn is_noeffect(&self) -> bool {
        *self == MDIRR::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `ACCEPTANCEFILTERING`"]
    #[inline]
    pub fn is_acceptancefiltering(&self) -> bool {
        *self == MDIRR::ACCEPTANCEFILTERING
    }
}
#[doc = "Possible values of the field `MXTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MXTDR {
    #[doc = "The extended identifier bit (XTD) has no effect on acceptance filtering"]
    NOEFFECT,
    #[doc = "The extended identifier bit (XTD) is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl MXTDR {
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
            MXTDR::NOEFFECT => false,
            MXTDR::ACCEPTANCEFILTERING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MXTDR {
        match value {
            false => MXTDR::NOEFFECT,
            true => MXTDR::ACCEPTANCEFILTERING,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline]
    pub fn is_noeffect(&self) -> bool {
        *self == MXTDR::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `ACCEPTANCEFILTERING`"]
    #[inline]
    pub fn is_acceptancefiltering(&self) -> bool {
        *self == MXTDR::ACCEPTANCEFILTERING
    }
}
#[doc = "Values that can be written to the field `MSK_28_16`"]
pub enum MSK_28_16W {
    #[doc = "The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    NOINHIBIT,
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl MSK_28_16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            MSK_28_16W::NOINHIBIT => 0,
            MSK_28_16W::ACCEPTANCEFILTERING => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSK_28_16W<'a> {
    w: &'a mut W,
}
impl<'a> _MSK_28_16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSK_28_16W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    #[inline]
    pub fn noinhibit(self) -> &'a mut W {
        self.variant(MSK_28_16W::NOINHIBIT)
    }
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    #[inline]
    pub fn acceptancefiltering(self) -> &'a mut W {
        self.variant(MSK_28_16W::ACCEPTANCEFILTERING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDIR`"]
pub enum MDIRW {
    #[doc = "The message direction bit (DIR) has no effect on acceptance filtering"]
    NOEFFECT,
    #[doc = "The message direction bit (DIR) is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl MDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDIRW::NOEFFECT => false,
            MDIRW::ACCEPTANCEFILTERING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The message direction bit (DIR) has no effect on acceptance filtering"]
    #[inline]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MDIRW::NOEFFECT)
    }
    #[doc = "The message direction bit (DIR) is used for acceptance filtering"]
    #[inline]
    pub fn acceptancefiltering(self) -> &'a mut W {
        self.variant(MDIRW::ACCEPTANCEFILTERING)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MXTD`"]
pub enum MXTDW {
    #[doc = "The extended identifier bit (XTD) has no effect on acceptance filtering"]
    NOEFFECT,
    #[doc = "The extended identifier bit (XTD) is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl MXTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MXTDW::NOEFFECT => false,
            MXTDW::ACCEPTANCEFILTERING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MXTDW<'a> {
    w: &'a mut W,
}
impl<'a> _MXTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MXTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The extended identifier bit (XTD) has no effect on acceptance filtering"]
    #[inline]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MXTDW::NOEFFECT)
    }
    #[doc = "The extended identifier bit (XTD) is used for acceptance filtering"]
    #[inline]
    pub fn acceptancefiltering(self) -> &'a mut W {
        self.variant(MXTDW::ACCEPTANCEFILTERING)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:12 - Identifier mask"]
    #[inline]
    pub fn msk_28_16(&self) -> MSK_28_16R {
        MSK_28_16R::_from({
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 14 - Mask message direction"]
    #[inline]
    pub fn mdir(&self) -> MDIRR {
        MDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Mask extend identifier"]
    #[inline]
    pub fn mxtd(&self) -> MXTDR {
        MXTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:12 - Identifier mask"]
    #[inline]
    pub fn msk_28_16(&mut self) -> _MSK_28_16W {
        _MSK_28_16W { w: self }
    }
    #[doc = "Bit 14 - Mask message direction"]
    #[inline]
    pub fn mdir(&mut self) -> _MDIRW {
        _MDIRW { w: self }
    }
    #[doc = "Bit 15 - Mask extend identifier"]
    #[inline]
    pub fn mxtd(&mut self) -> _MXTDW {
        _MXTDW { w: self }
    }
}
