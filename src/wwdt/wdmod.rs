#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WDMOD {
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
#[doc = "Possible values of the field `WDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDENR {
    #[doc = "The watchdog timer is stopped"]
    STOPPED,
    #[doc = "The watchdog timer is running"]
    RUN,
}
impl WDENR {
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
            WDENR::STOPPED => false,
            WDENR::RUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDENR {
        match value {
            false => WDENR::STOPPED,
            true => WDENR::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline]
    pub fn is_stopped(&self) -> bool {
        *self == WDENR::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == WDENR::RUN
    }
}
#[doc = "Possible values of the field `WDRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRESETR {
    #[doc = "A watchdog timeout will not cause a chip reset"]
    NORESET,
    #[doc = "A watchdog timeout will cause a chip reset"]
    RESET,
}
impl WDRESETR {
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
            WDRESETR::NORESET => false,
            WDRESETR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDRESETR {
        match value {
            false => WDRESETR::NORESET,
            true => WDRESETR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline]
    pub fn is_noreset(&self) -> bool {
        *self == WDRESETR::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == WDRESETR::RESET
    }
}
#[doc = r" Value of the field"]
pub struct WDTOFR {
    bits: bool,
}
impl WDTOFR {
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
pub struct WDINTR {
    bits: bool,
}
impl WDINTR {
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
#[doc = "Possible values of the field `WDPROTECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDPROTECTR {
    #[doc = "The watchdog reload value (WDTC) can be changed at any time"]
    ANYTIME,
    #[doc = "The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1"]
    LOWCOUNTER,
}
impl WDPROTECTR {
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
            WDPROTECTR::ANYTIME => false,
            WDPROTECTR::LOWCOUNTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDPROTECTR {
        match value {
            false => WDPROTECTR::ANYTIME,
            true => WDPROTECTR::LOWCOUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `ANYTIME`"]
    #[inline]
    pub fn is_anytime(&self) -> bool {
        *self == WDPROTECTR::ANYTIME
    }
    #[doc = "Checks if the value of the field is `LOWCOUNTER`"]
    #[inline]
    pub fn is_lowcounter(&self) -> bool {
        *self == WDPROTECTR::LOWCOUNTER
    }
}
#[doc = "Values that can be written to the field `WDEN`"]
pub enum WDENW {
    #[doc = "The watchdog timer is stopped"]
    STOPPED,
    #[doc = "The watchdog timer is running"]
    RUN,
}
impl WDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDENW::STOPPED => false,
            WDENW::RUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDENW<'a> {
    w: &'a mut W,
}
impl<'a> _WDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The watchdog timer is stopped"]
    #[inline]
    pub fn stopped(self) -> &'a mut W {
        self.variant(WDENW::STOPPED)
    }
    #[doc = "The watchdog timer is running"]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(WDENW::RUN)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDRESET`"]
pub enum WDRESETW {
    #[doc = "A watchdog timeout will not cause a chip reset"]
    NORESET,
    #[doc = "A watchdog timeout will cause a chip reset"]
    RESET,
}
impl WDRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDRESETW::NORESET => false,
            WDRESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A watchdog timeout will not cause a chip reset"]
    #[inline]
    pub fn noreset(self) -> &'a mut W {
        self.variant(WDRESETW::NORESET)
    }
    #[doc = "A watchdog timeout will cause a chip reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDRESETW::RESET)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDTOFW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOFW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDINTW<'a> {
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
#[doc = "Values that can be written to the field `WDPROTECT`"]
pub enum WDPROTECTW {
    #[doc = "The watchdog reload value (WDTC) can be changed at any time"]
    ANYTIME,
    #[doc = "The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1"]
    LOWCOUNTER,
}
impl WDPROTECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDPROTECTW::ANYTIME => false,
            WDPROTECTW::LOWCOUNTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDPROTECTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDPROTECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDPROTECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The watchdog reload value (WDTC) can be changed at any time"]
    #[inline]
    pub fn anytime(self) -> &'a mut W {
        self.variant(WDPROTECTW::ANYTIME)
    }
    #[doc = "The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1"]
    #[inline]
    pub fn lowcounter(self) -> &'a mut W {
        self.variant(WDPROTECTW::LOWCOUNTER)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only. Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to the watchdog oscillator before setting this bit to one."]
    #[inline]
    pub fn wden(&self) -> WDENR {
        WDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only."]
    #[inline]
    pub fn wdreset(&self) -> WDRESETR {
        WDRESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline]
    pub fn wdtof(&self) -> WDTOFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDTOFR { bits }
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline]
    pub fn wdint(&self) -> WDINTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDINTR { bits }
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit is Set Only."]
    #[inline]
    pub fn wdprotect(&self) -> WDPROTECTR {
        WDPROTECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only. Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to the watchdog oscillator before setting this bit to one."]
    #[inline]
    pub fn wden(&mut self) -> _WDENW {
        _WDENW { w: self }
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only."]
    #[inline]
    pub fn wdreset(&mut self) -> _WDRESETW {
        _WDRESETW { w: self }
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline]
    pub fn wdtof(&mut self) -> _WDTOFW {
        _WDTOFW { w: self }
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline]
    pub fn wdint(&mut self) -> _WDINTW {
        _WDINTW { w: self }
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit is Set Only."]
    #[inline]
    pub fn wdprotect(&mut self) -> _WDPROTECTW {
        _WDPROTECTW { w: self }
    }
}
