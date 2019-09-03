#[doc = "Reader of register CANINT"]
pub type R = crate::R<u32, super::CANINT>;
#[doc = "Reader of field `INTID`"]
pub type INTID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - 0x0000 = No interrupt is pending. 0x0001 - 0x0020 = Number of message object which caused the interrupt. 0x0021 - 0x7FFF = Unused 0x8000 = Status interrupt 0x8001 - 0xFFFF = Unused."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0xffff) as u16)
    }
}
