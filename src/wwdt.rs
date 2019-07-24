#[doc = r" Register block"]
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
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer"]
pub struct WDMOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer"]
pub mod wdmod;
#[doc = "Watchdog timer constant register. This register determines the time-out value"]
pub struct WDTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog timer constant register. This register determines the time-out value"]
pub mod wdtc;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC"]
pub struct WDFEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC"]
pub mod wdfeed;
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer"]
pub struct WDTV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer"]
pub mod wdtv;
#[doc = "Watchdog Warning Interrupt compare value"]
pub struct WDWARNINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Warning Interrupt compare value"]
pub mod wdwarnint;
#[doc = "Watchdog Window compare value"]
pub struct WDWINDOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Window compare value"]
pub mod wdwindow;
