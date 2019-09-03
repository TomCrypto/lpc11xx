#[doc = "Reader of register MAINCLKSEL"]
pub type R = crate::R<u32, super::MAINCLKSEL>;
#[doc = "Writer for register MAINCLKSEL"]
pub type W = crate::W<u32, super::MAINCLKSEL>;
#[doc = "Register MAINCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::MAINCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source for main clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: IRC oscillator"]
    IRC_OSCILLATOR,
    #[doc = "1: Input clock to system PLL"]
    INPUT_CLOCK_TO_SYSTE,
    #[doc = "2: WDT oscillator"]
    WDT_OSCILLATOR,
    #[doc = "3: System PLL clock out"]
    SYSTEM_PLL_CLOCK_OUT,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        match variant {
            SEL_A::IRC_OSCILLATOR => 0,
            SEL_A::INPUT_CLOCK_TO_SYSTE => 1,
            SEL_A::WDT_OSCILLATOR => 2,
            SEL_A::SYSTEM_PLL_CLOCK_OUT => 3,
        }
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::IRC_OSCILLATOR,
            1 => SEL_A::INPUT_CLOCK_TO_SYSTE,
            2 => SEL_A::WDT_OSCILLATOR,
            3 => SEL_A::SYSTEM_PLL_CLOCK_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SEL_A::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `INPUT_CLOCK_TO_SYSTE`"]
    #[inline(always)]
    pub fn is_input_clock_to_syste(&self) -> bool {
        *self == SEL_A::INPUT_CLOCK_TO_SYSTE
    }
    #[doc = "Checks if the value of the field is `WDT_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_wdt_oscillator(&self) -> bool {
        *self == SEL_A::WDT_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `SYSTEM_PLL_CLOCK_OUT`"]
    #[inline(always)]
    pub fn is_system_pll_clock_out(&self) -> bool {
        *self == SEL_A::SYSTEM_PLL_CLOCK_OUT
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSCILLATOR)
    }
    #[doc = "Input clock to system PLL"]
    #[inline(always)]
    pub fn input_clock_to_syste(self) -> &'a mut W {
        self.variant(SEL_A::INPUT_CLOCK_TO_SYSTE)
    }
    #[doc = "WDT oscillator"]
    #[inline(always)]
    pub fn wdt_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WDT_OSCILLATOR)
    }
    #[doc = "System PLL clock out"]
    #[inline(always)]
    pub fn system_pll_clock_out(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_PLL_CLOCK_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for main clock."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for main clock."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
