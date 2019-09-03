#[doc = "Reader of register CANND2"]
pub type R = crate::R<u32, super::CANND2>;
#[doc = "Reader of field `NEWDAT_32_17`"]
pub type NEWDAT_32_17_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - New data bits of message objects 32 to 17. 0 = No new data has been written into the data portion of this Message Object by the Message Handler since last time this flag was cleared by the CPU. 1 = The Message Handler or the CPU has written new data into the data portion of this Message Object."]
    #[inline(always)]
    pub fn newdat_32_17(&self) -> NEWDAT_32_17_R {
        NEWDAT_32_17_R::new((self.bits & 0xffff) as u16)
    }
}
