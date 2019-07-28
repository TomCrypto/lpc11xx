#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTCR {
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
#[doc = "Possible values of the field `CTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTMR {
    #[doc = "Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI,
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_RISING,
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_FALLING,
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_BOTH,
}
impl CTMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTMR::TIMER_MODE_EVERY_RI => 0,
            CTMR::COUNTER_MODE_TC_IS_RISING => 1,
            CTMR::COUNTER_MODE_TC_IS_FALLING => 2,
            CTMR::COUNTER_MODE_TC_IS_BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTMR {
        match value {
            0 => CTMR::TIMER_MODE_EVERY_RI,
            1 => CTMR::COUNTER_MODE_TC_IS_RISING,
            2 => CTMR::COUNTER_MODE_TC_IS_FALLING,
            3 => CTMR::COUNTER_MODE_TC_IS_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_EVERY_RI`"]
    #[inline]
    pub fn is_timer_mode_every_ri(&self) -> bool {
        *self == CTMR::TIMER_MODE_EVERY_RI
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_TC_IS_RISING`"]
    #[inline]
    pub fn is_counter_mode_tc_is_rising(&self) -> bool {
        *self == CTMR::COUNTER_MODE_TC_IS_RISING
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_TC_IS_FALLING`"]
    #[inline]
    pub fn is_counter_mode_tc_is_falling(&self) -> bool {
        *self == CTMR::COUNTER_MODE_TC_IS_FALLING
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_TC_IS_BOTH`"]
    #[inline]
    pub fn is_counter_mode_tc_is_both(&self) -> bool {
        *self == CTMR::COUNTER_MODE_TC_IS_BOTH
    }
}
#[doc = "Possible values of the field `CIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CISR {
    #[doc = "CT16Bn_CAP0"]
    CT16BN_CAP0,
    #[doc = "CT16Bn_CAP1"]
    CT16BN_CAP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CISR::CT16BN_CAP0 => 0,
            CISR::CT16BN_CAP1 => 1,
            CISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CISR {
        match value {
            0 => CISR::CT16BN_CAP0,
            1 => CISR::CT16BN_CAP1,
            i => CISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CT16BN_CAP0`"]
    #[inline]
    pub fn is_ct16bn_cap0(&self) -> bool {
        *self == CISR::CT16BN_CAP0
    }
    #[doc = "Checks if the value of the field is `CT16BN_CAP1`"]
    #[inline]
    pub fn is_ct16bn_cap1(&self) -> bool {
        *self == CISR::CT16BN_CAP1
    }
}
#[doc = r" Value of the field"]
pub struct ENCCR {
    bits: bool,
}
impl ENCCR {
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
#[doc = "Possible values of the field `SELCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELCCR {
    #[doc = "Rising Edge of CAP0 clears the timer (if bit 4 is set)"]
    RISING_EDGE_OF_CAP0_,
    #[doc = "Falling Edge of CAP0 clears the timer (if bit 4 is set)"]
    FALLING_EDGE_OF_CAP0,
    #[doc = "Rising Edge of CAP1 clears the timer (if bit 4 is set)"]
    RISING_EDGE_OF_CAP1_,
    #[doc = "Falling Edge of CAP1 clears the timer (if bit 4 is set)"]
    FALLING_EDGE_OF_CAP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELCCR::RISING_EDGE_OF_CAP0_ => 0,
            SELCCR::FALLING_EDGE_OF_CAP0 => 1,
            SELCCR::RISING_EDGE_OF_CAP1_ => 2,
            SELCCR::FALLING_EDGE_OF_CAP1 => 3,
            SELCCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELCCR {
        match value {
            0 => SELCCR::RISING_EDGE_OF_CAP0_,
            1 => SELCCR::FALLING_EDGE_OF_CAP0,
            2 => SELCCR::RISING_EDGE_OF_CAP1_,
            3 => SELCCR::FALLING_EDGE_OF_CAP1,
            i => SELCCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_CAP0_`"]
    #[inline]
    pub fn is_rising_edge_of_cap0_(&self) -> bool {
        *self == SELCCR::RISING_EDGE_OF_CAP0_
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_CAP0`"]
    #[inline]
    pub fn is_falling_edge_of_cap0(&self) -> bool {
        *self == SELCCR::FALLING_EDGE_OF_CAP0
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_CAP1_`"]
    #[inline]
    pub fn is_rising_edge_of_cap1_(&self) -> bool {
        *self == SELCCR::RISING_EDGE_OF_CAP1_
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_CAP1`"]
    #[inline]
    pub fn is_falling_edge_of_cap1(&self) -> bool {
        *self == SELCCR::FALLING_EDGE_OF_CAP1
    }
}
#[doc = "Values that can be written to the field `CTM`"]
pub enum CTMW {
    #[doc = "Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI,
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_RISING,
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_FALLING,
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_BOTH,
}
impl CTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTMW::TIMER_MODE_EVERY_RI => 0,
            CTMW::COUNTER_MODE_TC_IS_RISING => 1,
            CTMW::COUNTER_MODE_TC_IS_FALLING => 2,
            CTMW::COUNTER_MODE_TC_IS_BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTMW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline]
    pub fn timer_mode_every_ri(self) -> &'a mut W {
        self.variant(CTMW::TIMER_MODE_EVERY_RI)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2"]
    #[inline]
    pub fn counter_mode_tc_is_rising(self) -> &'a mut W {
        self.variant(CTMW::COUNTER_MODE_TC_IS_RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2"]
    #[inline]
    pub fn counter_mode_tc_is_falling(self) -> &'a mut W {
        self.variant(CTMW::COUNTER_MODE_TC_IS_FALLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2"]
    #[inline]
    pub fn counter_mode_tc_is_both(self) -> &'a mut W {
        self.variant(CTMW::COUNTER_MODE_TC_IS_BOTH)
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
#[doc = "Values that can be written to the field `CIS`"]
pub enum CISW {
    #[doc = "CT16Bn_CAP0"]
    CT16BN_CAP0,
    #[doc = "CT16Bn_CAP1"]
    CT16BN_CAP1,
}
impl CISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CISW::CT16BN_CAP0 => 0,
            CISW::CT16BN_CAP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CISW<'a> {
    w: &'a mut W,
}
impl<'a> _CISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CT16Bn_CAP0"]
    #[inline]
    pub fn ct16bn_cap0(self) -> &'a mut W {
        self.variant(CISW::CT16BN_CAP0)
    }
    #[doc = "CT16Bn_CAP1"]
    #[inline]
    pub fn ct16bn_cap1(self) -> &'a mut W {
        self.variant(CISW::CT16BN_CAP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENCCW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCCW<'a> {
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
#[doc = "Values that can be written to the field `SELCC`"]
pub enum SELCCW {
    #[doc = "Rising Edge of CAP0 clears the timer (if bit 4 is set)"]
    RISING_EDGE_OF_CAP0_,
    #[doc = "Falling Edge of CAP0 clears the timer (if bit 4 is set)"]
    FALLING_EDGE_OF_CAP0,
    #[doc = "Rising Edge of CAP1 clears the timer (if bit 4 is set)"]
    RISING_EDGE_OF_CAP1_,
    #[doc = "Falling Edge of CAP1 clears the timer (if bit 4 is set)"]
    FALLING_EDGE_OF_CAP1,
}
impl SELCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELCCW::RISING_EDGE_OF_CAP0_ => 0,
            SELCCW::FALLING_EDGE_OF_CAP0 => 1,
            SELCCW::RISING_EDGE_OF_CAP1_ => 2,
            SELCCW::FALLING_EDGE_OF_CAP1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELCCW<'a> {
    w: &'a mut W,
}
impl<'a> _SELCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELCCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Rising Edge of CAP0 clears the timer (if bit 4 is set)"]
    #[inline]
    pub fn rising_edge_of_cap0_(self) -> &'a mut W {
        self.variant(SELCCW::RISING_EDGE_OF_CAP0_)
    }
    #[doc = "Falling Edge of CAP0 clears the timer (if bit 4 is set)"]
    #[inline]
    pub fn falling_edge_of_cap0(self) -> &'a mut W {
        self.variant(SELCCW::FALLING_EDGE_OF_CAP0)
    }
    #[doc = "Rising Edge of CAP1 clears the timer (if bit 4 is set)"]
    #[inline]
    pub fn rising_edge_of_cap1_(self) -> &'a mut W {
        self.variant(SELCCW::RISING_EDGE_OF_CAP1_)
    }
    #[doc = "Falling Edge of CAP1 clears the timer (if bit 4 is set)"]
    #[inline]
    pub fn falling_edge_of_cap1(self) -> &'a mut W {
        self.variant(SELCCW::FALLING_EDGE_OF_CAP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC)."]
    #[inline]
    pub fn ctm(&self) -> CTMR {
        CTMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected in the CTCR register, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline]
    pub fn cis(&self) -> CISR {
        CISR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Setting this bit to one enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline]
    pub fn encc(&self) -> ENCCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENCCR { bits }
    }
    #[doc = "Bits 5:7 - When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero."]
    #[inline]
    pub fn selcc(&self) -> SELCCR {
        SELCCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC)."]
    #[inline]
    pub fn ctm(&mut self) -> _CTMW {
        _CTMW { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected in the CTCR register, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline]
    pub fn cis(&mut self) -> _CISW {
        _CISW { w: self }
    }
    #[doc = "Bit 4 - Setting this bit to one enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline]
    pub fn encc(&mut self) -> _ENCCW {
        _ENCCW { w: self }
    }
    #[doc = "Bits 5:7 - When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero."]
    #[inline]
    pub fn selcc(&mut self) -> _SELCCW {
        _SELCCW { w: self }
    }
}
