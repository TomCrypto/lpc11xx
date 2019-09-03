#[doc = "Reader of register CANTXREQ1"]
pub type R = crate::R<u32, super::CANTXREQ1>;
#[doc = "Reader of field `TXRQST_16_1`"]
pub type TXRQST_16_1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmission request bit of message objects 16 to 1. 0 = This message object is not waiting for transmission. 1 = The transmission of this message object is requested and not yet done."]
    #[inline(always)]
    pub fn txrqst_16_1(&self) -> TXRQST_16_1_R {
        TXRQST_16_1_R::new((self.bits & 0xffff) as u16)
    }
}
