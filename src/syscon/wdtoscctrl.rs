#[doc = "Reader of register WDTOSCCTRL"]
pub type R = crate::R<u32, super::WDTOSCCTRL>;
#[doc = "Writer for register WDTOSCCTRL"]
pub type W = crate::W<u32, super::WDTOSCCTRL>;
#[doc = "Register WDTOSCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTOSCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVSEL`"]
pub type DIVSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVSEL`"]
pub struct DIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Select watchdog oscillator analog output frequency (Fclkana).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQSEL_A {
    #[doc = "1: 0.5 MHz"]
    _0_5_MHZ,
    #[doc = "2: 0.8 MHz"]
    _0_8_MHZ,
    #[doc = "3: 1.1 MHz"]
    _1_1_MHZ,
    #[doc = "4: 1.4 MHz"]
    _1_4_MHZ,
    #[doc = "5: 1.6 MHz"]
    _1_6_MHZ,
    #[doc = "6: 1.8 MHz"]
    _1_8_MHZ,
    #[doc = "7: 2.0 MHz"]
    _2_0_MHZ,
    #[doc = "8: 2.2 MHz"]
    _2_2_MHZ,
    #[doc = "9: 2.4 MHz"]
    _2_4_MHZ,
    #[doc = "10: 2.6 MHz"]
    _2_6_MHZ,
    #[doc = "11: 2.7 MHz"]
    _2_7_MHZ,
    #[doc = "12: 2.9 MHz"]
    _2_9_MHZ,
    #[doc = "13: 3.1 MHz"]
    _3_1_MHZ,
    #[doc = "14: 3.2 MHz"]
    _3_2_MHZ,
    #[doc = "15: 3.4 MHz"]
    _3_4_MHZ,
}
impl From<FREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQSEL_A) -> Self {
        match variant {
            FREQSEL_A::_0_5_MHZ => 1,
            FREQSEL_A::_0_8_MHZ => 2,
            FREQSEL_A::_1_1_MHZ => 3,
            FREQSEL_A::_1_4_MHZ => 4,
            FREQSEL_A::_1_6_MHZ => 5,
            FREQSEL_A::_1_8_MHZ => 6,
            FREQSEL_A::_2_0_MHZ => 7,
            FREQSEL_A::_2_2_MHZ => 8,
            FREQSEL_A::_2_4_MHZ => 9,
            FREQSEL_A::_2_6_MHZ => 10,
            FREQSEL_A::_2_7_MHZ => 11,
            FREQSEL_A::_2_9_MHZ => 12,
            FREQSEL_A::_3_1_MHZ => 13,
            FREQSEL_A::_3_2_MHZ => 14,
            FREQSEL_A::_3_4_MHZ => 15,
        }
    }
}
#[doc = "Reader of field `FREQSEL`"]
pub type FREQSEL_R = crate::R<u8, FREQSEL_A>;
impl FREQSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FREQSEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FREQSEL_A::_0_5_MHZ),
            2 => Val(FREQSEL_A::_0_8_MHZ),
            3 => Val(FREQSEL_A::_1_1_MHZ),
            4 => Val(FREQSEL_A::_1_4_MHZ),
            5 => Val(FREQSEL_A::_1_6_MHZ),
            6 => Val(FREQSEL_A::_1_8_MHZ),
            7 => Val(FREQSEL_A::_2_0_MHZ),
            8 => Val(FREQSEL_A::_2_2_MHZ),
            9 => Val(FREQSEL_A::_2_4_MHZ),
            10 => Val(FREQSEL_A::_2_6_MHZ),
            11 => Val(FREQSEL_A::_2_7_MHZ),
            12 => Val(FREQSEL_A::_2_9_MHZ),
            13 => Val(FREQSEL_A::_3_1_MHZ),
            14 => Val(FREQSEL_A::_3_2_MHZ),
            15 => Val(FREQSEL_A::_3_4_MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_5_MHZ`"]
    #[inline(always)]
    pub fn is_0_5_mhz(&self) -> bool {
        *self == FREQSEL_A::_0_5_MHZ
    }
    #[doc = "Checks if the value of the field is `_0_8_MHZ`"]
    #[inline(always)]
    pub fn is_0_8_mhz(&self) -> bool {
        *self == FREQSEL_A::_0_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_1_MHZ`"]
    #[inline(always)]
    pub fn is_1_1_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_1_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_4_MHZ`"]
    #[inline(always)]
    pub fn is_1_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_6_MHZ`"]
    #[inline(always)]
    pub fn is_1_6_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_6_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_8_MHZ`"]
    #[inline(always)]
    pub fn is_1_8_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_0_MHZ`"]
    #[inline(always)]
    pub fn is_2_0_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_0_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_2_MHZ`"]
    #[inline(always)]
    pub fn is_2_2_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_2_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_4_MHZ`"]
    #[inline(always)]
    pub fn is_2_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_6_MHZ`"]
    #[inline(always)]
    pub fn is_2_6_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_6_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_7_MHZ`"]
    #[inline(always)]
    pub fn is_2_7_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_7_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_9_MHZ`"]
    #[inline(always)]
    pub fn is_2_9_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_9_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_1_MHZ`"]
    #[inline(always)]
    pub fn is_3_1_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_1_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_2_MHZ`"]
    #[inline(always)]
    pub fn is_3_2_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_2_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_4_MHZ`"]
    #[inline(always)]
    pub fn is_3_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_4_MHZ
    }
}
#[doc = "Write proxy for field `FREQSEL`"]
pub struct FREQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.5 MHz"]
    #[inline(always)]
    pub fn _0_5_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_0_5_MHZ)
    }
    #[doc = "0.8 MHz"]
    #[inline(always)]
    pub fn _0_8_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_0_8_MHZ)
    }
    #[doc = "1.1 MHz"]
    #[inline(always)]
    pub fn _1_1_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_1_MHZ)
    }
    #[doc = "1.4 MHz"]
    #[inline(always)]
    pub fn _1_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_4_MHZ)
    }
    #[doc = "1.6 MHz"]
    #[inline(always)]
    pub fn _1_6_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_6_MHZ)
    }
    #[doc = "1.8 MHz"]
    #[inline(always)]
    pub fn _1_8_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_8_MHZ)
    }
    #[doc = "2.0 MHz"]
    #[inline(always)]
    pub fn _2_0_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_0_MHZ)
    }
    #[doc = "2.2 MHz"]
    #[inline(always)]
    pub fn _2_2_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_2_MHZ)
    }
    #[doc = "2.4 MHz"]
    #[inline(always)]
    pub fn _2_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_4_MHZ)
    }
    #[doc = "2.6 MHz"]
    #[inline(always)]
    pub fn _2_6_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_6_MHZ)
    }
    #[doc = "2.7 MHz"]
    #[inline(always)]
    pub fn _2_7_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_7_MHZ)
    }
    #[doc = "2.9 MHz"]
    #[inline(always)]
    pub fn _2_9_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_9_MHZ)
    }
    #[doc = "3.1 MHz"]
    #[inline(always)]
    pub fn _3_1_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_1_MHZ)
    }
    #[doc = "3.2 MHz"]
    #[inline(always)]
    pub fn _3_2_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_2_MHZ)
    }
    #[doc = "3.4 MHz"]
    #[inline(always)]
    pub fn _3_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_4_MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64."]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64."]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W {
        DIVSEL_W { w: self }
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline(always)]
    pub fn freqsel(&mut self) -> FREQSEL_W {
        FREQSEL_W { w: self }
    }
}
