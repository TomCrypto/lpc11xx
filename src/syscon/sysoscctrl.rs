#[doc = "Reader of register SYSOSCCTRL"]
pub type R = crate::R<u32, super::SYSOSCCTRL>;
#[doc = "Writer for register SYSOSCCTRL"]
pub type W = crate::W<u32, super::SYSOSCCTRL>;
#[doc = "Register SYSOSCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSOSCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bypass system oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Oscillator is not bypassed"]
    NOBYPASS,
    #[doc = "1: Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins"]
    BYPASS_ENABLED_PLL_,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        match variant {
            BYPASS_A::NOBYPASS => false,
            BYPASS_A::BYPASS_ENABLED_PLL_ => true,
        }
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, BYPASS_A>;
impl BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::NOBYPASS,
            true => BYPASS_A::BYPASS_ENABLED_PLL_,
        }
    }
    #[doc = "Checks if the value of the field is `NOBYPASS`"]
    #[inline(always)]
    pub fn is_nobypass(&self) -> bool {
        *self == BYPASS_A::NOBYPASS
    }
    #[doc = "Checks if the value of the field is `BYPASS_ENABLED_PLL_`"]
    #[inline(always)]
    pub fn is_bypass_enabled_pll_(&self) -> bool {
        *self == BYPASS_A::BYPASS_ENABLED_PLL_
    }
}
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Oscillator is not bypassed"]
    #[inline(always)]
    pub fn nobypass(self) -> &'a mut W {
        self.variant(BYPASS_A::NOBYPASS)
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins"]
    #[inline(always)]
    pub fn bypass_enabled_pll_(self) -> &'a mut W {
        self.variant(BYPASS_A::BYPASS_ENABLED_PLL_)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Determines frequency range for Low-power oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQRANGE_A {
    #[doc = "0: 1 - 20 MHz frequency range"]
    LOW,
    #[doc = "1: 15 - 25 MHz frequency range"]
    HIGH,
}
impl From<FREQRANGE_A> for bool {
    #[inline(always)]
    fn from(variant: FREQRANGE_A) -> Self {
        match variant {
            FREQRANGE_A::LOW => false,
            FREQRANGE_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `FREQRANGE`"]
pub type FREQRANGE_R = crate::R<bool, FREQRANGE_A>;
impl FREQRANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQRANGE_A {
        match self.bits {
            false => FREQRANGE_A::LOW,
            true => FREQRANGE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FREQRANGE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FREQRANGE_A::HIGH
    }
}
#[doc = "Write proxy for field `FREQRANGE`"]
pub struct FREQRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQRANGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1 - 20 MHz frequency range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FREQRANGE_A::LOW)
    }
    #[doc = "15 - 25 MHz frequency range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FREQRANGE_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bypass system oscillator."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass system oscillator."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FREQRANGE_W {
        FREQRANGE_W { w: self }
    }
}
