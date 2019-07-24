#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCON {
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
#[doc = "Possible values of the field `DPDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDENR {
    #[doc = "ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M0 core turned off)"]
    SLEEPMODE,
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)"]
    DEEPPOWERDOWN,
}
impl DPDENR {
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
            DPDENR::SLEEPMODE => false,
            DPDENR::DEEPPOWERDOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPDENR {
        match value {
            false => DPDENR::SLEEPMODE,
            true => DPDENR::DEEPPOWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEPMODE`"]
    #[inline]
    pub fn is_sleepmode(&self) -> bool {
        *self == DPDENR::SLEEPMODE
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == DPDENR::DEEPPOWERDOWN
    }
}
#[doc = "Possible values of the field `SLEEPFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAGR {
    #[doc = "Read: No power-down mode entered. LPC111x/LPC11C1x is in Active mode. Write: No effect"]
    NOPOWERDOWN,
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0"]
    POWERDOWN,
}
impl SLEEPFLAGR {
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
            SLEEPFLAGR::NOPOWERDOWN => false,
            SLEEPFLAGR::POWERDOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPFLAGR {
        match value {
            false => SLEEPFLAGR::NOPOWERDOWN,
            true => SLEEPFLAGR::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NOPOWERDOWN`"]
    #[inline]
    pub fn is_nopowerdown(&self) -> bool {
        *self == SLEEPFLAGR::NOPOWERDOWN
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline]
    pub fn is_powerdown(&self) -> bool {
        *self == SLEEPFLAGR::POWERDOWN
    }
}
#[doc = "Possible values of the field `DPDFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAGR {
    #[doc = "Read: Deep power-down mode not entered. Write: No effect"]
    NODEEPPOWERDOWN,
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag"]
    DEEPPOWERDOWN,
}
impl DPDFLAGR {
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
            DPDFLAGR::NODEEPPOWERDOWN => false,
            DPDFLAGR::DEEPPOWERDOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPDFLAGR {
        match value {
            false => DPDFLAGR::NODEEPPOWERDOWN,
            true => DPDFLAGR::DEEPPOWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NODEEPPOWERDOWN`"]
    #[inline]
    pub fn is_nodeeppowerdown(&self) -> bool {
        *self == DPDFLAGR::NODEEPPOWERDOWN
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == DPDFLAGR::DEEPPOWERDOWN
    }
}
#[doc = "Values that can be written to the field `DPDEN`"]
pub enum DPDENW {
    #[doc = "ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M0 core turned off)"]
    SLEEPMODE,
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)"]
    DEEPPOWERDOWN,
}
impl DPDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDENW::SLEEPMODE => false,
            DPDENW::DEEPPOWERDOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPDENW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M0 core turned off)"]
    #[inline]
    pub fn sleepmode(self) -> &'a mut W {
        self.variant(DPDENW::SLEEPMODE)
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)"]
    #[inline]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(DPDENW::DEEPPOWERDOWN)
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
#[doc = "Values that can be written to the field `SLEEPFLAG`"]
pub enum SLEEPFLAGW {
    #[doc = "Read: No power-down mode entered. LPC111x/LPC11C1x is in Active mode. Write: No effect"]
    NOPOWERDOWN,
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0"]
    POWERDOWN,
}
impl SLEEPFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPFLAGW::NOPOWERDOWN => false,
            SLEEPFLAGW::POWERDOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: No power-down mode entered. LPC111x/LPC11C1x is in Active mode. Write: No effect"]
    #[inline]
    pub fn nopowerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::NOPOWERDOWN)
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0"]
    #[inline]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::POWERDOWN)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DPDFLAG`"]
pub enum DPDFLAGW {
    #[doc = "Read: Deep power-down mode not entered. Write: No effect"]
    NODEEPPOWERDOWN,
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag"]
    DEEPPOWERDOWN,
}
impl DPDFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDFLAGW::NODEEPPOWERDOWN => false,
            DPDFLAGW::DEEPPOWERDOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPDFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPDFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: Deep power-down mode not entered. Write: No effect"]
    #[inline]
    pub fn nodeeppowerdown(self) -> &'a mut W {
        self.variant(DPDFLAGW::NODEEPPOWERDOWN)
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag"]
    #[inline]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(DPDFLAGW::DEEPPOWERDOWN)
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
    #[doc = "Bit 1 - Deep power-down mode enable"]
    #[inline]
    pub fn dpden(&self) -> DPDENR {
        DPDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline]
    pub fn sleepflag(&self) -> SLEEPFLAGR {
        SLEEPFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline]
    pub fn dpdflag(&self) -> DPDFLAGR {
        DPDFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 1 - Deep power-down mode enable"]
    #[inline]
    pub fn dpden(&mut self) -> _DPDENW {
        _DPDENW { w: self }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline]
    pub fn sleepflag(&mut self) -> _SLEEPFLAGW {
        _SLEEPFLAGW { w: self }
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline]
    pub fn dpdflag(&mut self) -> _DPDFLAGW {
        _DPDFLAGW { w: self }
    }
}
