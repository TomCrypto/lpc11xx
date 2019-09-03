#[doc = "Reader of register CANMSGV2"]
pub type R = crate::R<u32, super::CANMSGV2>;
#[doc = "Reader of field `MSGVAL_32_17`"]
pub type MSGVAL_32_17_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Message valid bits of message objects 32 to 17. 0 = This message object is ignored by the message handler. 1 = This message object is configured and should be considered by the message handler."]
    #[inline(always)]
    pub fn msgval_32_17(&self) -> MSGVAL_32_17_R {
        MSGVAL_32_17_R::new((self.bits & 0xffff) as u16)
    }
}
