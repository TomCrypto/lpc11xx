#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I/O configuration for pin PIO2_6/ CT32B0_MAT1"]
    pub iocon_pio2_6: IOCON_PIO2_6,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - I/O configuration for pin PIO2_0/DTR/SSEL1"]
    pub iocon_pio2_0: IOCON_PIO2_0,
    #[doc = "0x0c - I/O configuration for pin RESET/PIO0_0"]
    pub iocon_reset_pio0_0: IOCON_RESET_PIO0_0,
    #[doc = "0x10 - I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2"]
    pub iocon_pio0_1: IOCON_PIO0_1,
    #[doc = "0x14 - I/O configuration for pin PIO1_8/CT16B1_CAP0"]
    pub iocon_pio1_8: IOCON_PIO1_8,
    #[doc = "0x18 - SSEL1 pin location select register"]
    pub iocon_ssel1_loc: IOCON_SSEL1_LOC,
    #[doc = "0x1c - I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
    pub iocon_pio0_2: IOCON_PIO0_2,
    #[doc = "0x20 - I/O configuration for pin PIO2_7/ CT32B0_MAT2/RXD"]
    pub iocon_pio2_7: IOCON_PIO2_7,
    #[doc = "0x24 - I/O configuration for pin PIO2_8/ CT32B0_MAT3/TXD"]
    pub iocon_pio2_8: IOCON_PIO2_8,
    #[doc = "0x28 - I/O configuration for pin PIO2_1/DSR/SCK1"]
    pub iocon_pio2_1: IOCON_PIO2_1,
    #[doc = "0x2c - I/O configuration for pin PIO0_3"]
    pub iocon_pio0_3: IOCON_PIO0_3,
    #[doc = "0x30 - I/O configuration for pin PIO0_4/SCL"]
    pub iocon_pio0_4: IOCON_PIO0_4,
    #[doc = "0x34 - I/O configuration for pin PIO0_5/SDA"]
    pub iocon_pio0_5: IOCON_PIO0_5,
    #[doc = "0x38 - I/O configuration for pin PIO1_9/CT16B1_MAT0/ MOSI1"]
    pub iocon_pio1_9: IOCON_PIO1_9,
    #[doc = "0x3c - I/O configuration for pin PIO3_4/ CT16B0_CAP1/RXD"]
    pub iocon_pio3_4: IOCON_PIO3_4,
    #[doc = "0x40 - I/O configuration for pin PIO2_4/ CT16B1_MAT1/ SSEL1"]
    pub iocon_pio2_4: IOCON_PIO2_4,
    #[doc = "0x44 - I/O configuration for pin PIO2_5/ CT32B0_MAT0"]
    pub iocon_pio2_5: IOCON_PIO2_5,
    #[doc = "0x48 - I/O configuration for pin PIO3_5/ CT16B1_CAP1/TXD"]
    pub iocon_pio3_5: IOCON_PIO3_5,
    #[doc = "0x4c - I/O configuration for pin PIO0_6/SCK0"]
    pub iocon_pio0_6: IOCON_PIO0_6,
    #[doc = "0x50 - I/O configuration for pin PIO0_7/CTS"]
    pub iocon_pio0_7: IOCON_PIO0_7,
    #[doc = "0x54 - I/O configuration for pin PIO2_9/ CT32B0_CAP0"]
    pub iocon_pio2_9: IOCON_PIO2_9,
    #[doc = "0x58 - I/O configuration for pin PIO2_10"]
    pub iocon_pio2_10: IOCON_PIO2_10,
    #[doc = "0x5c - I/O configuration for pin PIO2_2/DCD/MISO1"]
    pub iocon_pio2_2: IOCON_PIO2_2,
    #[doc = "0x60 - I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
    pub iocon_pio0_8: IOCON_PIO0_8,
    #[doc = "0x64 - I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
    pub iocon_pio0_9: IOCON_PIO0_9,
    #[doc = "0x68 - I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
    pub iocon_swclk_pio0_10: IOCON_SWCLK_PIO0_10,
    #[doc = "0x6c - I/O configuration for pin PIO1_10/AD6/CT16B1_MAT1/ MISO1"]
    pub iocon_pio1_10: IOCON_PIO1_10,
    #[doc = "0x70 - I/O configuration for pin PIO2_11/SCK0/ CT32B0_CAP1"]
    pub iocon_pio2_11: IOCON_PIO2_11,
    #[doc = "0x74 - I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
    pub iocon_r_pio0_11: IOCON_R_PIO0_11,
    #[doc = "0x78 - I/O configuration for pin R/PIO1_0/AD1/CT32B1_CAP0"]
    pub iocon_r_pio1_0: IOCON_R_PIO1_0,
    #[doc = "0x7c - I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
    pub iocon_r_pio1_1: IOCON_R_PIO1_1,
    #[doc = "0x80 - I/O configuration for pin R/PIO1_2/AD3/CT32B1_MAT1"]
    pub iocon_r_pio1_2: IOCON_R_PIO1_2,
    #[doc = "0x84 - I/O configuration for pin PIO3_0/DTR/CT16B0_MAT0/TXD"]
    pub iocon_pio3_0: IOCON_PIO3_0,
    #[doc = "0x88 - I/O configuration for pin PIO3_1/DSR/CT16B0_MAT1/RXD"]
    pub iocon_pio3_1: IOCON_PIO3_1,
    #[doc = "0x8c - I/O configuration for pin PIO2_3/RI/MOSI1"]
    pub iocon_pio2_3: IOCON_PIO2_3,
    #[doc = "0x90 - I/O configuration for pin SWDIO/PIO1_3/AD4/CT32B1_MAT2"]
    pub iocon_swdio_pio1_3: IOCON_SWDIO_PIO1_3,
    #[doc = "0x94 - I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
    pub iocon_pio1_4: IOCON_PIO1_4,
    #[doc = "0x98 - I/O configuration for pin PIO1_11/AD7/CT32B1_CAP1"]
    pub iocon_pio1_11: IOCON_PIO1_11,
    #[doc = "0x9c - I/O configuration for pin PIO3_2/DCD/ CT16B0_MAT2/SCK1"]
    pub iocon_pio3_2: IOCON_PIO3_2,
    #[doc = "0xa0 - I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
    pub iocon_pio1_5: IOCON_PIO1_5,
    #[doc = "0xa4 - I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
    pub iocon_pio1_6: IOCON_PIO1_6,
    #[doc = "0xa8 - I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
    pub iocon_pio1_7: IOCON_PIO1_7,
    #[doc = "0xac - I/O configuration for pin PIO3_3/RI/ CT16B0_CAP0"]
    pub iocon_pio3_3: IOCON_PIO3_3,
    #[doc = "0xb0 - SCK0 pin location select register"]
    pub iocon_sck0_loc: IOCON_SCK0_LOC,
    #[doc = "0xb4 - DSR pin location select register"]
    pub iocon_dsr_loc: IOCON_DSR_LOC,
    #[doc = "0xb8 - DCD pin location select register"]
    pub iocon_dcd_loc: IOCON_DCD_LOC,
    #[doc = "0xbc - RI pin location select register"]
    pub iocon_ri_loc: IOCON_RI_LOC,
    #[doc = "0xc0 - CT16B0_CAP0 pin location select register"]
    pub iocon_ct16b0_cap0_loc: IOCON_CT16B0_CAP0_LOC,
    #[doc = "0xc4 - SCK1 pin location select register"]
    pub iocon_sck1_loc: IOCON_SCK1_LOC,
    #[doc = "0xc8 - MISO1 pin location select register"]
    pub iocon_miso1_loc: IOCON_MISO1_LOC,
    #[doc = "0xcc - MOSI1 pin location select register"]
    pub iocon_mosi1_loc: IOCON_MOSI1_LOC,
    #[doc = "0xd0 - CT32B0_CAP0 pin location select register"]
    pub iocon_ct32b0_cap0_loc: IOCON_CT32B0_CAP0_LOC,
    #[doc = "0xd4 - RXD pin location select register"]
    pub iocon_rxd_loc: IOCON_RXD_LOC,
}
#[doc = "I/O configuration for pin PIO2_6/ CT32B0_MAT1"]
pub struct IOCON_PIO2_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_6/ CT32B0_MAT1"]
pub mod iocon_pio2_6;
#[doc = "I/O configuration for pin PIO2_0/DTR/SSEL1"]
pub struct IOCON_PIO2_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_0/DTR/SSEL1"]
pub mod iocon_pio2_0;
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub struct IOCON_RESET_PIO0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub mod iocon_reset_pio0_0;
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2"]
pub struct IOCON_PIO0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2"]
pub mod iocon_pio0_1;
#[doc = "I/O configuration for pin PIO1_8/CT16B1_CAP0"]
pub struct IOCON_PIO1_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_8/CT16B1_CAP0"]
pub mod iocon_pio1_8;
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
pub struct IOCON_PIO0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
pub mod iocon_pio0_2;
#[doc = "I/O configuration for pin PIO2_7/ CT32B0_MAT2/RXD"]
pub struct IOCON_PIO2_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_7/ CT32B0_MAT2/RXD"]
pub mod iocon_pio2_7;
#[doc = "I/O configuration for pin PIO2_8/ CT32B0_MAT3/TXD"]
pub struct IOCON_PIO2_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_8/ CT32B0_MAT3/TXD"]
pub mod iocon_pio2_8;
#[doc = "I/O configuration for pin PIO2_1/DSR/SCK1"]
pub struct IOCON_PIO2_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_1/DSR/SCK1"]
pub mod iocon_pio2_1;
#[doc = "I/O configuration for pin PIO0_3"]
pub struct IOCON_PIO0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_3"]
pub mod iocon_pio0_3;
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub struct IOCON_PIO0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub mod iocon_pio0_4;
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub struct IOCON_PIO0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub mod iocon_pio0_5;
#[doc = "I/O configuration for pin PIO1_9/CT16B1_MAT0/ MOSI1"]
pub struct IOCON_PIO1_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_9/CT16B1_MAT0/ MOSI1"]
pub mod iocon_pio1_9;
#[doc = "I/O configuration for pin PIO3_4/ CT16B0_CAP1/RXD"]
pub struct IOCON_PIO3_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO3_4/ CT16B0_CAP1/RXD"]
pub mod iocon_pio3_4;
#[doc = "I/O configuration for pin PIO2_4/ CT16B1_MAT1/ SSEL1"]
pub struct IOCON_PIO2_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_4/ CT16B1_MAT1/ SSEL1"]
pub mod iocon_pio2_4;
#[doc = "I/O configuration for pin PIO2_5/ CT32B0_MAT0"]
pub struct IOCON_PIO2_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_5/ CT32B0_MAT0"]
pub mod iocon_pio2_5;
#[doc = "I/O configuration for pin PIO3_5/ CT16B1_CAP1/TXD"]
pub struct IOCON_PIO3_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO3_5/ CT16B1_CAP1/TXD"]
pub mod iocon_pio3_5;
#[doc = "I/O configuration for pin PIO0_6/SCK0"]
pub struct IOCON_PIO0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_6/SCK0"]
pub mod iocon_pio0_6;
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub struct IOCON_PIO0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub mod iocon_pio0_7;
#[doc = "I/O configuration for pin PIO2_9/ CT32B0_CAP0"]
pub struct IOCON_PIO2_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_9/ CT32B0_CAP0"]
pub mod iocon_pio2_9;
#[doc = "I/O configuration for pin PIO2_10"]
pub struct IOCON_PIO2_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_10"]
pub mod iocon_pio2_10;
#[doc = "I/O configuration for pin PIO2_2/DCD/MISO1"]
pub struct IOCON_PIO2_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_2/DCD/MISO1"]
pub mod iocon_pio2_2;
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub struct IOCON_PIO0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub mod iocon_pio0_8;
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
pub struct IOCON_PIO0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
pub mod iocon_pio0_9;
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
pub struct IOCON_SWCLK_PIO0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
pub mod iocon_swclk_pio0_10;
#[doc = "I/O configuration for pin PIO1_10/AD6/CT16B1_MAT1/ MISO1"]
pub struct IOCON_PIO1_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_10/AD6/CT16B1_MAT1/ MISO1"]
pub mod iocon_pio1_10;
#[doc = "I/O configuration for pin PIO2_11/SCK0/ CT32B0_CAP1"]
pub struct IOCON_PIO2_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_11/SCK0/ CT32B0_CAP1"]
pub mod iocon_pio2_11;
#[doc = "I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
pub struct IOCON_R_PIO0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
pub mod iocon_r_pio0_11;
#[doc = "I/O configuration for pin R/PIO1_0/AD1/CT32B1_CAP0"]
pub struct IOCON_R_PIO1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin R/PIO1_0/AD1/CT32B1_CAP0"]
pub mod iocon_r_pio1_0;
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
pub struct IOCON_R_PIO1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
pub mod iocon_r_pio1_1;
#[doc = "I/O configuration for pin R/PIO1_2/AD3/CT32B1_MAT1"]
pub struct IOCON_R_PIO1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin R/PIO1_2/AD3/CT32B1_MAT1"]
pub mod iocon_r_pio1_2;
#[doc = "I/O configuration for pin PIO3_0/DTR/CT16B0_MAT0/TXD"]
pub struct IOCON_PIO3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO3_0/DTR/CT16B0_MAT0/TXD"]
pub mod iocon_pio3_0;
#[doc = "I/O configuration for pin PIO3_1/DSR/CT16B0_MAT1/RXD"]
pub struct IOCON_PIO3_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO3_1/DSR/CT16B0_MAT1/RXD"]
pub mod iocon_pio3_1;
#[doc = "I/O configuration for pin PIO2_3/RI/MOSI1"]
pub struct IOCON_PIO2_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO2_3/RI/MOSI1"]
pub mod iocon_pio2_3;
#[doc = "I/O configuration for pin SWDIO/PIO1_3/AD4/CT32B1_MAT2"]
pub struct IOCON_SWDIO_PIO1_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin SWDIO/PIO1_3/AD4/CT32B1_MAT2"]
pub mod iocon_swdio_pio1_3;
#[doc = "I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
pub struct IOCON_PIO1_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
pub mod iocon_pio1_4;
#[doc = "I/O configuration for pin PIO1_11/AD7/CT32B1_CAP1"]
pub struct IOCON_PIO1_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_11/AD7/CT32B1_CAP1"]
pub mod iocon_pio1_11;
#[doc = "I/O configuration for pin PIO3_2/DCD/ CT16B0_MAT2/SCK1"]
pub struct IOCON_PIO3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO3_2/DCD/ CT16B0_MAT2/SCK1"]
pub mod iocon_pio3_2;
#[doc = "I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
pub struct IOCON_PIO1_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
pub mod iocon_pio1_5;
#[doc = "I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
pub struct IOCON_PIO1_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
pub mod iocon_pio1_6;
#[doc = "I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
pub struct IOCON_PIO1_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
pub mod iocon_pio1_7;
#[doc = "I/O configuration for pin PIO3_3/RI/ CT16B0_CAP0"]
pub struct IOCON_PIO3_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO3_3/RI/ CT16B0_CAP0"]
pub mod iocon_pio3_3;
#[doc = "SCK0 pin location select register"]
pub struct IOCON_SCK0_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCK0 pin location select register"]
pub mod iocon_sck0_loc;
#[doc = "DSR pin location select register"]
pub struct IOCON_DSR_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSR pin location select register"]
pub mod iocon_dsr_loc;
#[doc = "DCD pin location select register"]
pub struct IOCON_DCD_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCD pin location select register"]
pub mod iocon_dcd_loc;
#[doc = "RI pin location select register"]
pub struct IOCON_RI_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RI pin location select register"]
pub mod iocon_ri_loc;
#[doc = "SSEL1 pin location select register"]
pub struct IOCON_SSEL1_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSEL1 pin location select register"]
pub mod iocon_ssel1_loc;
#[doc = "CT16B0_CAP0 pin location select register"]
pub struct IOCON_CT16B0_CAP0_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CT16B0_CAP0 pin location select register"]
pub mod iocon_ct16b0_cap0_loc;
#[doc = "SCK1 pin location select register"]
pub struct IOCON_SCK1_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCK1 pin location select register"]
pub mod iocon_sck1_loc;
#[doc = "MISO1 pin location select register"]
pub struct IOCON_MISO1_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MISO1 pin location select register"]
pub mod iocon_miso1_loc;
#[doc = "MOSI1 pin location select register"]
pub struct IOCON_MOSI1_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MOSI1 pin location select register"]
pub mod iocon_mosi1_loc;
#[doc = "CT32B0_CAP0 pin location select register"]
pub struct IOCON_CT32B0_CAP0_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CT32B0_CAP0 pin location select register"]
pub mod iocon_ct32b0_cap0_loc;
#[doc = "RXD pin location select register"]
pub struct IOCON_RXD_LOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RXD pin location select register"]
pub mod iocon_rxd_loc;
