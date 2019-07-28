#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKOUTCLKSEL {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "IRC oscillator"]
    IRC_OSCILLATOR,
    #[doc = "System oscillator"]
    SYSTEM_OSCILLATOR,
    #[doc = "Watchdog oscillator"]
    WATCHDOG_OSCILLATOR,
    #[doc = "Main clock"]
    MAIN_CLOCK,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELR::IRC_OSCILLATOR => 0,
            SELR::SYSTEM_OSCILLATOR => 1,
            SELR::WATCHDOG_OSCILLATOR => 2,
            SELR::MAIN_CLOCK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELR {
        match value {
            0 => SELR::IRC_OSCILLATOR,
            1 => SELR::SYSTEM_OSCILLATOR,
            2 => SELR::WATCHDOG_OSCILLATOR,
            3 => SELR::MAIN_CLOCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SELR::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `SYSTEM_OSCILLATOR`"]
    #[inline]
    pub fn is_system_oscillator(&self) -> bool {
        *self == SELR::SYSTEM_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SELR::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline]
    pub fn is_main_clock(&self) -> bool {
        *self == SELR::MAIN_CLOCK
    }
}
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "IRC oscillator"]
    IRC_OSCILLATOR,
    #[doc = "System oscillator"]
    SYSTEM_OSCILLATOR,
    #[doc = "Watchdog oscillator"]
    WATCHDOG_OSCILLATOR,
    #[doc = "Main clock"]
    MAIN_CLOCK,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::IRC_OSCILLATOR => 0,
            SELW::SYSTEM_OSCILLATOR => 1,
            SELW::WATCHDOG_OSCILLATOR => 2,
            SELW::MAIN_CLOCK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IRC oscillator"]
    #[inline]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SELW::IRC_OSCILLATOR)
    }
    #[doc = "System oscillator"]
    #[inline]
    pub fn system_oscillator(self) -> &'a mut W {
        self.variant(SELW::SYSTEM_OSCILLATOR)
    }
    #[doc = "Watchdog oscillator"]
    #[inline]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SELW::WATCHDOG_OSCILLATOR)
    }
    #[doc = "Main clock"]
    #[inline]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SELW::MAIN_CLOCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - CLKOUT clock source."]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
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
    #[doc = "Bits 0:1 - CLKOUT clock source."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
