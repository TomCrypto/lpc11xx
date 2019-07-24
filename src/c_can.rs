#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN control"]
    pub cancntl: CANCNTL,
    #[doc = "0x04 - Status register"]
    pub canstat: CANSTAT,
    #[doc = "0x08 - Error counter"]
    pub canec: CANEC,
    #[doc = "0x0c - Bit timing register"]
    pub canbt: CANBT,
    #[doc = "0x10 - Interrupt register"]
    pub canint: CANINT,
    #[doc = "0x14 - Test register"]
    pub cantest: CANTEST,
    #[doc = "0x18 - Baud rate prescaler extension register"]
    pub canbrpe: CANBRPE,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Message interface command request"]
    pub canif1_cmdreq: CANIF_CMDREQ,
    #[doc = "Message interface command mask - read direction Message interface command mask - write direction"]
    pub canif1_cmdmsk: CANIF1_CMDMSK_UNION,
    #[doc = "0x28 - Message interface 1 mask 1"]
    pub canif1_msk1: CANIF_MSK1,
    #[doc = "0x2c - Message interface 1 mask 2"]
    pub canif1_msk2: CANIF_MSK2,
    #[doc = "0x30 - Message interface 1 arbitration 1"]
    pub canif1_arb1: CANIF_ARB1,
    #[doc = "0x34 - Message interface 1 arbitration 2"]
    pub canif1_arb2: CANIF_ARB2,
    #[doc = "0x38 - Message interface 1 message control"]
    pub canif1_mctrl: CANIF_MCTRL,
    #[doc = "0x3c - Message interface 1 data A1"]
    pub canif1_da1: CANIF_DA1,
    #[doc = "0x40 - Message interface 1 data A2"]
    pub canif1_da2: CANIF_DA2,
    #[doc = "0x44 - Message interface 1 data B1"]
    pub canif1_db1: CANIF_DB1,
    #[doc = "0x48 - Message interface 1 data B2"]
    pub canif1_db2: CANIF_DB2,
    _reserved18: [u8; 52usize],
    #[doc = "0x80 - Message interface command request"]
    pub canif2_cmdreq: CANIF_CMDREQ,
    #[doc = "Message interface command mask - read direction Message interface command mask - write direction"]
    pub canif2_cmdmsk: CANIF2_CMDMSK_UNION,
    #[doc = "0x88 - Message interface 1 mask 1"]
    pub canif2_msk1: CANIF_MSK1,
    #[doc = "0x8c - Message interface 1 mask 2"]
    pub canif2_msk2: CANIF_MSK2,
    #[doc = "0x90 - Message interface 1 arbitration 1"]
    pub canif2_arb1: CANIF_ARB1,
    #[doc = "0x94 - Message interface 1 arbitration 2"]
    pub canif2_arb2: CANIF_ARB2,
    #[doc = "0x98 - Message interface 1 message control"]
    pub canif2_mctrl: CANIF_MCTRL,
    #[doc = "0x9c - Message interface 1 data A1"]
    pub canif2_da1: CANIF_DA1,
    #[doc = "0xa0 - Message interface 1 data A2"]
    pub canif2_da2: CANIF_DA2,
    #[doc = "0xa4 - Message interface 1 data B1"]
    pub canif2_db1: CANIF_DB1,
    #[doc = "0xa8 - Message interface 1 data B2"]
    pub canif2_db2: CANIF_DB2,
    _reserved29: [u8; 84usize],
    #[doc = "0x100 - Transmission request 1"]
    pub cantxreq1: CANTXREQ1,
    #[doc = "0x104 - Transmission request 2"]
    pub cantxreq2: CANTXREQ2,
    _reserved31: [u8; 24usize],
    #[doc = "0x120 - New data 1"]
    pub cannd1: CANND1,
    #[doc = "0x124 - New data 2"]
    pub cannd2: CANND2,
    _reserved33: [u8; 24usize],
    #[doc = "0x140 - Interrupt pending 1"]
    pub canir1: CANIR1,
    #[doc = "0x144 - Interrupt pending 2"]
    pub canir2: CANIR2,
    _reserved35: [u8; 24usize],
    #[doc = "0x160 - Message valid 1"]
    pub canmsgv1: CANMSGV1,
    #[doc = "0x164 - Message valid 2"]
    pub canmsgv2: CANMSGV2,
    _reserved37: [u8; 24usize],
    #[doc = "0x180 - Can clock divider register"]
    pub canclkdiv: CANCLKDIV,
}
#[doc = "Message interface command mask - read direction Message interface command mask - write direction"]
#[repr(C)]
pub union CANIF1_CMDMSK_UNION {
    #[doc = "0x24 - Message interface command mask - read direction"]
    pub canif1_cmdmsk_r: CANIF_CMDMSK_R,
    #[doc = "0x24 - Message interface command mask - write direction"]
    pub canif1_cmdmsk_w: CANIF_CMDMSK_W,
}
#[doc = "Message interface command mask - read direction Message interface command mask - write direction"]
#[repr(C)]
pub union CANIF2_CMDMSK_UNION {
    #[doc = "0x84 - Message interface command mask - read direction"]
    pub canif2_cmdmsk_r: CANIF_CMDMSK_R,
    #[doc = "0x84 - Message interface command mask - write direction"]
    pub canif2_cmdmsk_w: CANIF_CMDMSK_W,
}
#[doc = "CAN control"]
pub struct CANCNTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN control"]
pub mod cancntl;
#[doc = "Status register"]
pub struct CANSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod canstat;
#[doc = "Error counter"]
pub struct CANEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error counter"]
pub mod canec;
#[doc = "Bit timing register"]
pub struct CANBT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit timing register"]
pub mod canbt;
#[doc = "Interrupt register"]
pub struct CANINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt register"]
pub mod canint;
#[doc = "Test register"]
pub struct CANTEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test register"]
pub mod cantest;
#[doc = "Baud rate prescaler extension register"]
pub struct CANBRPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud rate prescaler extension register"]
pub mod canbrpe;
#[doc = "Message interface command request"]
pub struct CANIF_CMDREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface command request"]
pub mod canif_cmdreq;
#[doc = "Message interface command mask - write direction"]
pub struct CANIF_CMDMSK_W {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface command mask - write direction"]
pub mod canif_cmdmsk_w;
#[doc = "Message interface command mask - read direction"]
pub struct CANIF_CMDMSK_R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface command mask - read direction"]
pub mod canif_cmdmsk_r;
#[doc = "Message interface 1 mask 1"]
pub struct CANIF_MSK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 mask 1"]
pub mod canif_msk1;
#[doc = "Message interface 1 mask 2"]
pub struct CANIF_MSK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 mask 2"]
pub mod canif_msk2;
#[doc = "Message interface 1 arbitration 1"]
pub struct CANIF_ARB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 arbitration 1"]
pub mod canif_arb1;
#[doc = "Message interface 1 arbitration 2"]
pub struct CANIF_ARB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 arbitration 2"]
pub mod canif_arb2;
#[doc = "Message interface 1 message control"]
pub struct CANIF_MCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 message control"]
pub mod canif_mctrl;
#[doc = "Message interface 1 data A1"]
pub struct CANIF_DA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 data A1"]
pub mod canif_da1;
#[doc = "Message interface 1 data A2"]
pub struct CANIF_DA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 data A2"]
pub mod canif_da2;
#[doc = "Message interface 1 data B1"]
pub struct CANIF_DB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 data B1"]
pub mod canif_db1;
#[doc = "Message interface 1 data B2"]
pub struct CANIF_DB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 data B2"]
pub mod canif_db2;
#[doc = "Transmission request 1"]
pub struct CANTXREQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission request 1"]
pub mod cantxreq1;
#[doc = "Transmission request 2"]
pub struct CANTXREQ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission request 2"]
pub mod cantxreq2;
#[doc = "New data 1"]
pub struct CANND1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New data 1"]
pub mod cannd1;
#[doc = "New data 2"]
pub struct CANND2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New data 2"]
pub mod cannd2;
#[doc = "Interrupt pending 1"]
pub struct CANIR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt pending 1"]
pub mod canir1;
#[doc = "Interrupt pending 2"]
pub struct CANIR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt pending 2"]
pub mod canir2;
#[doc = "Message valid 1"]
pub struct CANMSGV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message valid 1"]
pub mod canmsgv1;
#[doc = "Message valid 2"]
pub struct CANMSGV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message valid 2"]
pub mod canmsgv2;
#[doc = "Can clock divider register"]
pub struct CANCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Can clock divider register"]
pub mod canclkdiv;
