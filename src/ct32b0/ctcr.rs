#[doc = "Reader of register CTCR"]
pub type R = crate::R<u32, super::CTCR>;
#[doc = "Writer for register CTCR"]
pub type W = crate::W<u32, super::CTCR>;
#[doc = "Register CTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: every rising PCLK edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTM_A {
    #[doc = "0: Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI,
    #[doc = "1: Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_RISING,
    #[doc = "2: Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_FALLING,
    #[doc = "3: Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2"]
    COUNTER_MODE_TC_IS_BOTH,
}
impl From<CTM_A> for u8 {
    #[inline(always)]
    fn from(variant: CTM_A) -> Self {
        match variant {
            CTM_A::TIMER_MODE_EVERY_RI => 0,
            CTM_A::COUNTER_MODE_TC_IS_RISING => 1,
            CTM_A::COUNTER_MODE_TC_IS_FALLING => 2,
            CTM_A::COUNTER_MODE_TC_IS_BOTH => 3,
        }
    }
}
#[doc = "Reader of field `CTM`"]
pub type CTM_R = crate::R<u8, CTM_A>;
impl CTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTM_A {
        match self.bits {
            0 => CTM_A::TIMER_MODE_EVERY_RI,
            1 => CTM_A::COUNTER_MODE_TC_IS_RISING,
            2 => CTM_A::COUNTER_MODE_TC_IS_FALLING,
            3 => CTM_A::COUNTER_MODE_TC_IS_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_EVERY_RI`"]
    #[inline(always)]
    pub fn is_timer_mode_every_ri(&self) -> bool {
        *self == CTM_A::TIMER_MODE_EVERY_RI
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_TC_IS_RISING`"]
    #[inline(always)]
    pub fn is_counter_mode_tc_is_rising(&self) -> bool {
        *self == CTM_A::COUNTER_MODE_TC_IS_RISING
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_TC_IS_FALLING`"]
    #[inline(always)]
    pub fn is_counter_mode_tc_is_falling(&self) -> bool {
        *self == CTM_A::COUNTER_MODE_TC_IS_FALLING
    }
    #[doc = "Checks if the value of the field is `COUNTER_MODE_TC_IS_BOTH`"]
    #[inline(always)]
    pub fn is_counter_mode_tc_is_both(&self) -> bool {
        *self == CTM_A::COUNTER_MODE_TC_IS_BOTH
    }
}
#[doc = "Write proxy for field `CTM`"]
pub struct CTM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn timer_mode_every_ri(self) -> &'a mut W {
        self.variant(CTM_A::TIMER_MODE_EVERY_RI)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2"]
    #[inline(always)]
    pub fn counter_mode_tc_is_rising(self) -> &'a mut W {
        self.variant(CTM_A::COUNTER_MODE_TC_IS_RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2"]
    #[inline(always)]
    pub fn counter_mode_tc_is_falling(self) -> &'a mut W {
        self.variant(CTM_A::COUNTER_MODE_TC_IS_FALLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2"]
    #[inline(always)]
    pub fn counter_mode_tc_is_both(self) -> &'a mut W {
        self.variant(CTM_A::COUNTER_MODE_TC_IS_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Count Input Select. When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIS_A {
    #[doc = "0: CT32Bn_CAP0"]
    CT32BN_CAP0,
    #[doc = "1: CT32Bn_CAP1"]
    CT32BN_CAP1,
    #[doc = "3: Reserved Note: If Counter mode is selected in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000"]
    RESERVED_NOTE_IF_CO,
}
impl From<CIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        match variant {
            CIS_A::CT32BN_CAP0 => 0,
            CIS_A::CT32BN_CAP1 => 1,
            CIS_A::RESERVED_NOTE_IF_CO => 3,
        }
    }
}
#[doc = "Reader of field `CIS`"]
pub type CIS_R = crate::R<u8, CIS_A>;
impl CIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIS_A {
        match self.bits {
            0 => CIS_A::CT32BN_CAP0,
            1 => CIS_A::CT32BN_CAP1,
            3 => CIS_A::RESERVED_NOTE_IF_CO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CT32BN_CAP0`"]
    #[inline(always)]
    pub fn is_ct32bn_cap0(&self) -> bool {
        *self == CIS_A::CT32BN_CAP0
    }
    #[doc = "Checks if the value of the field is `CT32BN_CAP1`"]
    #[inline(always)]
    pub fn is_ct32bn_cap1(&self) -> bool {
        *self == CIS_A::CT32BN_CAP1
    }
    #[doc = "Checks if the value of the field is `RESERVED_NOTE_IF_CO`"]
    #[inline(always)]
    pub fn is_reserved_note_if_co(&self) -> bool {
        *self == CIS_A::RESERVED_NOTE_IF_CO
    }
}
#[doc = "Write proxy for field `CIS`"]
pub struct CIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CT32Bn_CAP0"]
    #[inline(always)]
    pub fn ct32bn_cap0(self) -> &'a mut W {
        self.variant(CIS_A::CT32BN_CAP0)
    }
    #[doc = "CT32Bn_CAP1"]
    #[inline(always)]
    pub fn ct32bn_cap1(self) -> &'a mut W {
        self.variant(CIS_A::CT32BN_CAP1)
    }
    #[doc = "Reserved Note: If Counter mode is selected in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000"]
    #[inline(always)]
    pub fn reserved_note_if_co(self) -> &'a mut W {
        self.variant(CIS_A::RESERVED_NOTE_IF_CO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ENCC`"]
pub type ENCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCC`"]
pub struct ENCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELCC_A {
    #[doc = "0: Rising Edge of CAP0 clears the timer (if bit 4 is set)"]
    RISING_EDGE_OF_CAP0_,
    #[doc = "1: Falling Edge of CAP0 clears the timer (if bit 4 is set)"]
    FALLING_EDGE_OF_CAP0,
    #[doc = "2: Rising Edge of CAP1 clears the timer (if bit 4 is set)"]
    RISING_EDGE_OF_CAP1_,
    #[doc = "3: Falling Edge of CAP1 clears the timer (if bit 4 is set)"]
    FALLING_EDGE_OF_CAP1,
}
impl From<SELCC_A> for u8 {
    #[inline(always)]
    fn from(variant: SELCC_A) -> Self {
        match variant {
            SELCC_A::RISING_EDGE_OF_CAP0_ => 0,
            SELCC_A::FALLING_EDGE_OF_CAP0 => 1,
            SELCC_A::RISING_EDGE_OF_CAP1_ => 2,
            SELCC_A::FALLING_EDGE_OF_CAP1 => 3,
        }
    }
}
#[doc = "Reader of field `SELCC`"]
pub type SELCC_R = crate::R<u8, SELCC_A>;
impl SELCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SELCC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SELCC_A::RISING_EDGE_OF_CAP0_),
            1 => Val(SELCC_A::FALLING_EDGE_OF_CAP0),
            2 => Val(SELCC_A::RISING_EDGE_OF_CAP1_),
            3 => Val(SELCC_A::FALLING_EDGE_OF_CAP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_CAP0_`"]
    #[inline(always)]
    pub fn is_rising_edge_of_cap0_(&self) -> bool {
        *self == SELCC_A::RISING_EDGE_OF_CAP0_
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_CAP0`"]
    #[inline(always)]
    pub fn is_falling_edge_of_cap0(&self) -> bool {
        *self == SELCC_A::FALLING_EDGE_OF_CAP0
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_CAP1_`"]
    #[inline(always)]
    pub fn is_rising_edge_of_cap1_(&self) -> bool {
        *self == SELCC_A::RISING_EDGE_OF_CAP1_
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_CAP1`"]
    #[inline(always)]
    pub fn is_falling_edge_of_cap1(&self) -> bool {
        *self == SELCC_A::FALLING_EDGE_OF_CAP1
    }
}
#[doc = "Write proxy for field `SELCC`"]
pub struct SELCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELCC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Rising Edge of CAP0 clears the timer (if bit 4 is set)"]
    #[inline(always)]
    pub fn rising_edge_of_cap0_(self) -> &'a mut W {
        self.variant(SELCC_A::RISING_EDGE_OF_CAP0_)
    }
    #[doc = "Falling Edge of CAP0 clears the timer (if bit 4 is set)"]
    #[inline(always)]
    pub fn falling_edge_of_cap0(self) -> &'a mut W {
        self.variant(SELCC_A::FALLING_EDGE_OF_CAP0)
    }
    #[doc = "Rising Edge of CAP1 clears the timer (if bit 4 is set)"]
    #[inline(always)]
    pub fn rising_edge_of_cap1_(self) -> &'a mut W {
        self.variant(SELCC_A::RISING_EDGE_OF_CAP1_)
    }
    #[doc = "Falling Edge of CAP1 clears the timer (if bit 4 is set)"]
    #[inline(always)]
    pub fn falling_edge_of_cap1(self) -> &'a mut W {
        self.variant(SELCC_A::FALLING_EDGE_OF_CAP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: every rising PCLK edge."]
    #[inline(always)]
    pub fn ctm(&self) -> CTM_R {
        CTM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking:."]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Setting this bit to one enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&self) -> ENCC_R {
        ENCC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero."]
    #[inline(always)]
    pub fn selcc(&self) -> SELCC_R {
        SELCC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: every rising PCLK edge."]
    #[inline(always)]
    pub fn ctm(&mut self) -> CTM_W {
        CTM_W { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking:."]
    #[inline(always)]
    pub fn cis(&mut self) -> CIS_W {
        CIS_W { w: self }
    }
    #[doc = "Bit 4 - Setting this bit to one enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&mut self) -> ENCC_W {
        ENCC_W { w: self }
    }
    #[doc = "Bits 5:7 - When bit 4 is one, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is zero."]
    #[inline(always)]
    pub fn selcc(&mut self) -> SELCC_W {
        SELCC_W { w: self }
    }
}
