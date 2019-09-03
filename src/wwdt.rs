#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer"]
    pub wdmod: WDMOD,
    #[doc = "0x04 - Watchdog timer constant register. This register determines the time-out value"]
    pub wdtc: WDTC,
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC"]
    pub wdfeed: WDFEED,
    #[doc = "0x0c - Watchdog timer value register. This register reads out the current value of the Watchdog timer"]
    pub wdtv: WDTV,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - Watchdog Warning Interrupt compare value"]
    pub wdwarnint: WDWARNINT,
    #[doc = "0x18 - Watchdog Window compare value"]
    pub wdwindow: WDWINDOW,
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdmod](wdmod) module"]
pub type WDMOD = crate::Reg<u32, _WDMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDMOD;
#[doc = "`read()` method returns [wdmod::R](wdmod::R) reader structure"]
impl crate::Readable for WDMOD {}
#[doc = "`write(|w| ..)` method takes [wdmod::W](wdmod::W) writer structure"]
impl crate::Writable for WDMOD {}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer"]
pub mod wdmod;
#[doc = "Watchdog timer constant register. This register determines the time-out value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtc](wdtc) module"]
pub type WDTC = crate::Reg<u32, _WDTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTC;
#[doc = "`read()` method returns [wdtc::R](wdtc::R) reader structure"]
impl crate::Readable for WDTC {}
#[doc = "`write(|w| ..)` method takes [wdtc::W](wdtc::W) writer structure"]
impl crate::Writable for WDTC {}
#[doc = "Watchdog timer constant register. This register determines the time-out value"]
pub mod wdtc;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdfeed](wdfeed) module"]
pub type WDFEED = crate::Reg<u32, _WDFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDFEED;
#[doc = "`write(|w| ..)` method takes [wdfeed::W](wdfeed::W) writer structure"]
impl crate::Writable for WDFEED {}
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC"]
pub mod wdfeed;
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtv](wdtv) module"]
pub type WDTV = crate::Reg<u32, _WDTV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTV;
#[doc = "`read()` method returns [wdtv::R](wdtv::R) reader structure"]
impl crate::Readable for WDTV {}
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer"]
pub mod wdtv;
#[doc = "Watchdog Warning Interrupt compare value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdwarnint](wdwarnint) module"]
pub type WDWARNINT = crate::Reg<u32, _WDWARNINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDWARNINT;
#[doc = "`read()` method returns [wdwarnint::R](wdwarnint::R) reader structure"]
impl crate::Readable for WDWARNINT {}
#[doc = "`write(|w| ..)` method takes [wdwarnint::W](wdwarnint::W) writer structure"]
impl crate::Writable for WDWARNINT {}
#[doc = "Watchdog Warning Interrupt compare value"]
pub mod wdwarnint;
#[doc = "Watchdog Window compare value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdwindow](wdwindow) module"]
pub type WDWINDOW = crate::Reg<u32, _WDWINDOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDWINDOW;
#[doc = "`read()` method returns [wdwindow::R](wdwindow::R) reader structure"]
impl crate::Readable for WDWINDOW {}
#[doc = "`write(|w| ..)` method takes [wdwindow::W](wdwindow::W) writer structure"]
impl crate::Writable for WDWINDOW {}
#[doc = "Watchdog Window compare value"]
pub mod wdwindow;
