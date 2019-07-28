#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDSLEEPCFG {
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
pub struct NOTUSED0R {
    bits: u8,
}
impl NOTUSED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BOD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PDR {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl BOD_PDR {
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
            BOD_PDR::POWERED => false,
            BOD_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOD_PDR {
        match value {
            false => BOD_PDR::POWERED,
            true => BOD_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PDR::POWERED_DOWN
    }
}
#[doc = r" Value of the field"]
pub struct NOTUSED1R {
    bits: u8,
}
impl NOTUSED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WDTOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PDR {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl WDTOSC_PDR {
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
            WDTOSC_PDR::POWERED => false,
            WDTOSC_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTOSC_PDR {
        match value {
            false => WDTOSC_PDR::POWERED,
            true => WDTOSC_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == WDTOSC_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == WDTOSC_PDR::POWERED_DOWN
    }
}
#[doc = r" Value of the field"]
pub struct NOTUSED2R {
    bits: bool,
}
impl NOTUSED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct NOTUSED3R {
    bits: u8,
}
impl NOTUSED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NOTUSED4R {
    bits: u8,
}
impl NOTUSED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _NOTUSED0W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTUSED0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOD_PD`"]
pub enum BOD_PDW {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl BOD_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOD_PDW::POWERED => false,
            BOD_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOD_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOD_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED)
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED_DOWN)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTUSED1W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTUSED1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTOSC_PD`"]
pub enum WDTOSC_PDW {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl WDTOSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTOSC_PDW::POWERED => false,
            WDTOSC_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTOSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOSC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTOSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::POWERED)
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::POWERED_DOWN)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTUSED2W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTUSED2W<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTUSED3W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTUSED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTUSED4W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTUSED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:2 - Reserved. Always write these bits as 111."]
    #[inline]
    pub fn notused0(&self) -> NOTUSED0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NOTUSED0R { bits }
    }
    #[doc = "Bit 3 - BOD power-down control in Deep-sleep mode, see Table 40."]
    #[inline]
    pub fn bod_pd(&self) -> BOD_PDR {
        BOD_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Reserved. Always write these bits as 11."]
    #[inline]
    pub fn notused1(&self) -> NOTUSED1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NOTUSED1R { bits }
    }
    #[doc = "Bit 6 - Watchdog oscillator power control in Deep-sleep mode, see Table 40."]
    #[inline]
    pub fn wdtosc_pd(&self) -> WDTOSC_PDR {
        WDTOSC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Reserved. Always write this bit as 1."]
    #[inline]
    pub fn notused2(&self) -> NOTUSED2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOTUSED2R { bits }
    }
    #[doc = "Bits 8:10 - Reserved. Always write these bits as 000."]
    #[inline]
    pub fn notused3(&self) -> NOTUSED3R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NOTUSED3R { bits }
    }
    #[doc = "Bits 11:12 - Reserved. Always write these bits as 11."]
    #[inline]
    pub fn notused4(&self) -> NOTUSED4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NOTUSED4R { bits }
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
    #[doc = "Bits 0:2 - Reserved. Always write these bits as 111."]
    #[inline]
    pub fn notused0(&mut self) -> _NOTUSED0W {
        _NOTUSED0W { w: self }
    }
    #[doc = "Bit 3 - BOD power-down control in Deep-sleep mode, see Table 40."]
    #[inline]
    pub fn bod_pd(&mut self) -> _BOD_PDW {
        _BOD_PDW { w: self }
    }
    #[doc = "Bits 4:5 - Reserved. Always write these bits as 11."]
    #[inline]
    pub fn notused1(&mut self) -> _NOTUSED1W {
        _NOTUSED1W { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator power control in Deep-sleep mode, see Table 40."]
    #[inline]
    pub fn wdtosc_pd(&mut self) -> _WDTOSC_PDW {
        _WDTOSC_PDW { w: self }
    }
    #[doc = "Bit 7 - Reserved. Always write this bit as 1."]
    #[inline]
    pub fn notused2(&mut self) -> _NOTUSED2W {
        _NOTUSED2W { w: self }
    }
    #[doc = "Bits 8:10 - Reserved. Always write these bits as 000."]
    #[inline]
    pub fn notused3(&mut self) -> _NOTUSED3W {
        _NOTUSED3W { w: self }
    }
    #[doc = "Bits 11:12 - Reserved. Always write these bits as 11."]
    #[inline]
    pub fn notused4(&mut self) -> _NOTUSED4W {
        _NOTUSED4W { w: self }
    }
}
