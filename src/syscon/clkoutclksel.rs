#[doc = "Reader of register CLKOUTCLKSEL"]
pub type R = crate::R<u32, super::CLKOUTCLKSEL>;
#[doc = "Writer for register CLKOUTCLKSEL"]
pub type W = crate::W<u32, super::CLKOUTCLKSEL>;
#[doc = "Register CLKOUTCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKOUTCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CLKOUT clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: IRC oscillator"]
    IRC_OSCILLATOR,
    #[doc = "1: System oscillator"]
    SYSTEM_OSCILLATOR,
    #[doc = "2: Watchdog oscillator"]
    WATCHDOG_OSCILLATOR,
    #[doc = "3: Main clock"]
    MAIN_CLOCK,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        match variant {
            SEL_A::IRC_OSCILLATOR => 0,
            SEL_A::SYSTEM_OSCILLATOR => 1,
            SEL_A::WATCHDOG_OSCILLATOR => 2,
            SEL_A::MAIN_CLOCK => 3,
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
            1 => SEL_A::SYSTEM_OSCILLATOR,
            2 => SEL_A::WATCHDOG_OSCILLATOR,
            3 => SEL_A::MAIN_CLOCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SEL_A::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `SYSTEM_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_system_oscillator(&self) -> bool {
        *self == SEL_A::SYSTEM_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SEL_A::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK
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
    #[doc = "System oscillator"]
    #[inline(always)]
    pub fn system_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_OSCILLATOR)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WATCHDOG_OSCILLATOR)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CLKOUT clock source."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKOUT clock source."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
