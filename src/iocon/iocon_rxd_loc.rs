#[doc = "Reader of register IOCON_RXD_LOC"]
pub type R = crate::R<u32, super::IOCON_RXD_LOC>;
#[doc = "Writer for register IOCON_RXD_LOC"]
pub type W = crate::W<u32, super::IOCON_RXD_LOC>;
#[doc = "Register IOCON_RXD_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_RXD_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for the RXD function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDLOC_A {
    #[doc = "0: Function RXD is available for pin PIO1_6"]
    PIO1_6,
    #[doc = "1: Function RXD is available for pin PIO2_7"]
    PIO2_7,
    #[doc = "2: Function RXD is available for pin PIO3_1"]
    PIO3_1,
    #[doc = "3: Function RXD is available for pin PIO3_4"]
    PIO3_4,
}
impl From<RXDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RXDLOC_A) -> Self {
        match variant {
            RXDLOC_A::PIO1_6 => 0,
            RXDLOC_A::PIO2_7 => 1,
            RXDLOC_A::PIO3_1 => 2,
            RXDLOC_A::PIO3_4 => 3,
        }
    }
}
#[doc = "Reader of field `RXDLOC`"]
pub type RXDLOC_R = crate::R<u8, RXDLOC_A>;
impl RXDLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDLOC_A {
        match self.bits {
            0 => RXDLOC_A::PIO1_6,
            1 => RXDLOC_A::PIO2_7,
            2 => RXDLOC_A::PIO3_1,
            3 => RXDLOC_A::PIO3_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIO1_6`"]
    #[inline(always)]
    pub fn is_pio1_6(&self) -> bool {
        *self == RXDLOC_A::PIO1_6
    }
    #[doc = "Checks if the value of the field is `PIO2_7`"]
    #[inline(always)]
    pub fn is_pio2_7(&self) -> bool {
        *self == RXDLOC_A::PIO2_7
    }
    #[doc = "Checks if the value of the field is `PIO3_1`"]
    #[inline(always)]
    pub fn is_pio3_1(&self) -> bool {
        *self == RXDLOC_A::PIO3_1
    }
    #[doc = "Checks if the value of the field is `PIO3_4`"]
    #[inline(always)]
    pub fn is_pio3_4(&self) -> bool {
        *self == RXDLOC_A::PIO3_4
    }
}
#[doc = "Write proxy for field `RXDLOC`"]
pub struct RXDLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDLOC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Function RXD is available for pin PIO1_6"]
    #[inline(always)]
    pub fn pio1_6(self) -> &'a mut W {
        self.variant(RXDLOC_A::PIO1_6)
    }
    #[doc = "Function RXD is available for pin PIO2_7"]
    #[inline(always)]
    pub fn pio2_7(self) -> &'a mut W {
        self.variant(RXDLOC_A::PIO2_7)
    }
    #[doc = "Function RXD is available for pin PIO3_1"]
    #[inline(always)]
    pub fn pio3_1(self) -> &'a mut W {
        self.variant(RXDLOC_A::PIO3_1)
    }
    #[doc = "Function RXD is available for pin PIO3_4"]
    #[inline(always)]
    pub fn pio3_4(self) -> &'a mut W {
        self.variant(RXDLOC_A::PIO3_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for the RXD function."]
    #[inline(always)]
    pub fn rxdloc(&self) -> RXDLOC_R {
        RXDLOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for the RXD function."]
    #[inline(always)]
    pub fn rxdloc(&mut self) -> RXDLOC_W {
        RXDLOC_W { w: self }
    }
}
