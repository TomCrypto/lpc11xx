#[doc = "Reader of register CANTXREQ2"]
pub type R = crate::R<u32, super::CANTXREQ2>;
#[doc = "Reader of field `TXRQST_32_17`"]
pub type TXRQST_32_17_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmission request bit of message objects 32 to 17. 0 = This message object is not waiting for transmission. 1 = The transmission of this message object is requested and not yet done."]
    #[inline(always)]
    pub fn txrqst_32_17(&self) -> TXRQST_32_17_R {
        TXRQST_32_17_R::new((self.bits & 0xffff) as u16)
    }
}
