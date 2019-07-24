#[doc = r" Register block"]
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
#[doc = "Data register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod data;
#[doc = "Data direction register"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data direction register"]
pub mod dir;
#[doc = "Interrupt sense register"]
pub struct IS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt sense register"]
pub mod is;
#[doc = "Interrupt both-edges register"]
pub struct IBE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt both-edges register"]
pub mod ibe;
#[doc = "Interrupt event register"]
pub struct IEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt event register"]
pub mod iev;
#[doc = "Interrupt mask register"]
pub struct IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt mask register"]
pub mod ie;
#[doc = "Raw interrupt status register"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw interrupt status register"]
pub mod ris;
#[doc = "Masked interrupt status register"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked interrupt status register"]
pub mod mis;
#[doc = "Interrupt clear register"]
pub struct IC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt clear register"]
pub mod ic;
