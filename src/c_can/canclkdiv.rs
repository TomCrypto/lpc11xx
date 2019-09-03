#[doc = "Reader of register CANCLKDIV"]
pub type R = crate::R<u32, super::CANCLKDIV>;
#[doc = "Writer for register CANCLKDIV"]
pub type W = crate::W<u32, super::CANCLKDIV>;
#[doc = "Register CANCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CANCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKDIVVAL`"]
pub type CLKDIVVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIVVAL`"]
pub struct CLKDIVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIVVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock divider value. CAN_CLK = PCLK/(CLKDIVVAL +1) 0000: CAN_CLK = PCLK divided by 1. 0001: CAN_CLK = PCLK divided by 2. 0010: CAN_CLK = PCLK divided by 3 0010: CAN_CLK = PCLK divided by 4. ... 1111: CAN_CLK = PCLK divided by 16."]
    #[inline(always)]
    pub fn clkdivval(&self) -> CLKDIVVAL_R {
        CLKDIVVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock divider value. CAN_CLK = PCLK/(CLKDIVVAL +1) 0000: CAN_CLK = PCLK divided by 1. 0001: CAN_CLK = PCLK divided by 2. 0010: CAN_CLK = PCLK divided by 3 0010: CAN_CLK = PCLK divided by 4. ... 1111: CAN_CLK = PCLK divided by 16."]
    #[inline(always)]
    pub fn clkdivval(&mut self) -> CLKDIVVAL_W {
        CLKDIVVAL_W { w: self }
    }
}
