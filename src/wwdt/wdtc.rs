#[doc = "Reader of register WDTC"]
pub type R = crate::R<u32, super::WDTC>;
#[doc = "Writer for register WDTC"]
pub type W = crate::W<u32, super::WDTC>;
#[doc = "Register WDTC `reset()`'s with value 0xff"]
impl crate::ResetValue for super::WDTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `Count`"]
pub type COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Count`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Watchdog time-out interval."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Watchdog time-out interval."]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
