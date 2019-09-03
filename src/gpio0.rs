#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16380usize],
    #[doc = "0x3ffc - Data register"]
    pub data: DATA,
    _reserved1: [u8; 16384usize],
    #[doc = "0x8000 - Data direction register"]
    pub dir: DIR,
    #[doc = "0x8004 - Interrupt sense register"]
    pub is: IS,
    #[doc = "0x8008 - Interrupt both-edges register"]
    pub ibe: IBE,
    #[doc = "0x800c - Interrupt event register"]
    pub iev: IEV,
    #[doc = "0x8010 - Interrupt mask register"]
    pub ie: IE,
    #[doc = "0x8014 - Raw interrupt status register"]
    pub ris: RIS,
    #[doc = "0x8018 - Masked interrupt status register"]
    pub mis: MIS,
    #[doc = "0x801c - Interrupt clear register"]
    pub ic: IC,
}
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Data register"]
pub mod data;
#[doc = "Data direction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "Data direction register"]
pub mod dir;
#[doc = "Interrupt sense register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [is](is) module"]
pub type IS = crate::Reg<u32, _IS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IS;
#[doc = "`read()` method returns [is::R](is::R) reader structure"]
impl crate::Readable for IS {}
#[doc = "`write(|w| ..)` method takes [is::W](is::W) writer structure"]
impl crate::Writable for IS {}
#[doc = "Interrupt sense register"]
pub mod is;
#[doc = "Interrupt both-edges register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ibe](ibe) module"]
pub type IBE = crate::Reg<u32, _IBE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IBE;
#[doc = "`read()` method returns [ibe::R](ibe::R) reader structure"]
impl crate::Readable for IBE {}
#[doc = "`write(|w| ..)` method takes [ibe::W](ibe::W) writer structure"]
impl crate::Writable for IBE {}
#[doc = "Interrupt both-edges register"]
pub mod ibe;
#[doc = "Interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iev](iev) module"]
pub type IEV = crate::Reg<u32, _IEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEV;
#[doc = "`read()` method returns [iev::R](iev::R) reader structure"]
impl crate::Readable for IEV {}
#[doc = "`write(|w| ..)` method takes [iev::W](iev::W) writer structure"]
impl crate::Writable for IEV {}
#[doc = "Interrupt event register"]
pub mod iev;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "Interrupt mask register"]
pub mod ie;
#[doc = "Raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Raw interrupt status register"]
pub mod ris;
#[doc = "Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "Masked interrupt status register"]
pub mod mis;
#[doc = "Interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ic](ic) module"]
pub type IC = crate::Reg<u32, _IC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC;
#[doc = "`write(|w| ..)` method takes [ic::W](ic::W) writer structure"]
impl crate::Writable for IC {}
#[doc = "Interrupt clear register"]
pub mod ic;
