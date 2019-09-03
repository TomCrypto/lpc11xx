#[doc = r"Register block"]
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
#[doc = "I/O configuration for pin PIO2_6/ CT32B0_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_6](iocon_pio2_6) module"]
pub type IOCON_PIO2_6 = crate::Reg<u32, _IOCON_PIO2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_6;
#[doc = "`read()` method returns [iocon_pio2_6::R](iocon_pio2_6::R) reader structure"]
impl crate::Readable for IOCON_PIO2_6 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_6::W](iocon_pio2_6::W) writer structure"]
impl crate::Writable for IOCON_PIO2_6 {}
#[doc = "I/O configuration for pin PIO2_6/ CT32B0_MAT1"]
pub mod iocon_pio2_6;
#[doc = "I/O configuration for pin PIO2_0/DTR/SSEL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_0](iocon_pio2_0) module"]
pub type IOCON_PIO2_0 = crate::Reg<u32, _IOCON_PIO2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_0;
#[doc = "`read()` method returns [iocon_pio2_0::R](iocon_pio2_0::R) reader structure"]
impl crate::Readable for IOCON_PIO2_0 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_0::W](iocon_pio2_0::W) writer structure"]
impl crate::Writable for IOCON_PIO2_0 {}
#[doc = "I/O configuration for pin PIO2_0/DTR/SSEL1"]
pub mod iocon_pio2_0;
#[doc = "I/O configuration for pin RESET/PIO0_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_reset_pio0_0](iocon_reset_pio0_0) module"]
pub type IOCON_RESET_PIO0_0 = crate::Reg<u32, _IOCON_RESET_PIO0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_RESET_PIO0_0;
#[doc = "`read()` method returns [iocon_reset_pio0_0::R](iocon_reset_pio0_0::R) reader structure"]
impl crate::Readable for IOCON_RESET_PIO0_0 {}
#[doc = "`write(|w| ..)` method takes [iocon_reset_pio0_0::W](iocon_reset_pio0_0::W) writer structure"]
impl crate::Writable for IOCON_RESET_PIO0_0 {}
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub mod iocon_reset_pio0_0;
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_1](iocon_pio0_1) module"]
pub type IOCON_PIO0_1 = crate::Reg<u32, _IOCON_PIO0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_1;
#[doc = "`read()` method returns [iocon_pio0_1::R](iocon_pio0_1::R) reader structure"]
impl crate::Readable for IOCON_PIO0_1 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_1::W](iocon_pio0_1::W) writer structure"]
impl crate::Writable for IOCON_PIO0_1 {}
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2"]
pub mod iocon_pio0_1;
#[doc = "I/O configuration for pin PIO1_8/CT16B1_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio1_8](iocon_pio1_8) module"]
pub type IOCON_PIO1_8 = crate::Reg<u32, _IOCON_PIO1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO1_8;
#[doc = "`read()` method returns [iocon_pio1_8::R](iocon_pio1_8::R) reader structure"]
impl crate::Readable for IOCON_PIO1_8 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio1_8::W](iocon_pio1_8::W) writer structure"]
impl crate::Writable for IOCON_PIO1_8 {}
#[doc = "I/O configuration for pin PIO1_8/CT16B1_CAP0"]
pub mod iocon_pio1_8;
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_2](iocon_pio0_2) module"]
pub type IOCON_PIO0_2 = crate::Reg<u32, _IOCON_PIO0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_2;
#[doc = "`read()` method returns [iocon_pio0_2::R](iocon_pio0_2::R) reader structure"]
impl crate::Readable for IOCON_PIO0_2 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_2::W](iocon_pio0_2::W) writer structure"]
impl crate::Writable for IOCON_PIO0_2 {}
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
pub mod iocon_pio0_2;
#[doc = "I/O configuration for pin PIO2_7/ CT32B0_MAT2/RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_7](iocon_pio2_7) module"]
pub type IOCON_PIO2_7 = crate::Reg<u32, _IOCON_PIO2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_7;
#[doc = "`read()` method returns [iocon_pio2_7::R](iocon_pio2_7::R) reader structure"]
impl crate::Readable for IOCON_PIO2_7 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_7::W](iocon_pio2_7::W) writer structure"]
impl crate::Writable for IOCON_PIO2_7 {}
#[doc = "I/O configuration for pin PIO2_7/ CT32B0_MAT2/RXD"]
pub mod iocon_pio2_7;
#[doc = "I/O configuration for pin PIO2_8/ CT32B0_MAT3/TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_8](iocon_pio2_8) module"]
pub type IOCON_PIO2_8 = crate::Reg<u32, _IOCON_PIO2_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_8;
#[doc = "`read()` method returns [iocon_pio2_8::R](iocon_pio2_8::R) reader structure"]
impl crate::Readable for IOCON_PIO2_8 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_8::W](iocon_pio2_8::W) writer structure"]
impl crate::Writable for IOCON_PIO2_8 {}
#[doc = "I/O configuration for pin PIO2_8/ CT32B0_MAT3/TXD"]
pub mod iocon_pio2_8;
#[doc = "I/O configuration for pin PIO2_1/DSR/SCK1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_1](iocon_pio2_1) module"]
pub type IOCON_PIO2_1 = crate::Reg<u32, _IOCON_PIO2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_1;
#[doc = "`read()` method returns [iocon_pio2_1::R](iocon_pio2_1::R) reader structure"]
impl crate::Readable for IOCON_PIO2_1 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_1::W](iocon_pio2_1::W) writer structure"]
impl crate::Writable for IOCON_PIO2_1 {}
#[doc = "I/O configuration for pin PIO2_1/DSR/SCK1"]
pub mod iocon_pio2_1;
#[doc = "I/O configuration for pin PIO0_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_3](iocon_pio0_3) module"]
pub type IOCON_PIO0_3 = crate::Reg<u32, _IOCON_PIO0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_3;
#[doc = "`read()` method returns [iocon_pio0_3::R](iocon_pio0_3::R) reader structure"]
impl crate::Readable for IOCON_PIO0_3 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_3::W](iocon_pio0_3::W) writer structure"]
impl crate::Writable for IOCON_PIO0_3 {}
#[doc = "I/O configuration for pin PIO0_3"]
pub mod iocon_pio0_3;
#[doc = "I/O configuration for pin PIO0_4/SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_4](iocon_pio0_4) module"]
pub type IOCON_PIO0_4 = crate::Reg<u32, _IOCON_PIO0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_4;
#[doc = "`read()` method returns [iocon_pio0_4::R](iocon_pio0_4::R) reader structure"]
impl crate::Readable for IOCON_PIO0_4 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_4::W](iocon_pio0_4::W) writer structure"]
impl crate::Writable for IOCON_PIO0_4 {}
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub mod iocon_pio0_4;
#[doc = "I/O configuration for pin PIO0_5/SDA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_5](iocon_pio0_5) module"]
pub type IOCON_PIO0_5 = crate::Reg<u32, _IOCON_PIO0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_5;
#[doc = "`read()` method returns [iocon_pio0_5::R](iocon_pio0_5::R) reader structure"]
impl crate::Readable for IOCON_PIO0_5 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_5::W](iocon_pio0_5::W) writer structure"]
impl crate::Writable for IOCON_PIO0_5 {}
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub mod iocon_pio0_5;
#[doc = "I/O configuration for pin PIO1_9/CT16B1_MAT0/ MOSI1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio1_9](iocon_pio1_9) module"]
pub type IOCON_PIO1_9 = crate::Reg<u32, _IOCON_PIO1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO1_9;
#[doc = "`read()` method returns [iocon_pio1_9::R](iocon_pio1_9::R) reader structure"]
impl crate::Readable for IOCON_PIO1_9 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio1_9::W](iocon_pio1_9::W) writer structure"]
impl crate::Writable for IOCON_PIO1_9 {}
#[doc = "I/O configuration for pin PIO1_9/CT16B1_MAT0/ MOSI1"]
pub mod iocon_pio1_9;
#[doc = "I/O configuration for pin PIO3_4/ CT16B0_CAP1/RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio3_4](iocon_pio3_4) module"]
pub type IOCON_PIO3_4 = crate::Reg<u32, _IOCON_PIO3_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO3_4;
#[doc = "`read()` method returns [iocon_pio3_4::R](iocon_pio3_4::R) reader structure"]
impl crate::Readable for IOCON_PIO3_4 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio3_4::W](iocon_pio3_4::W) writer structure"]
impl crate::Writable for IOCON_PIO3_4 {}
#[doc = "I/O configuration for pin PIO3_4/ CT16B0_CAP1/RXD"]
pub mod iocon_pio3_4;
#[doc = "I/O configuration for pin PIO2_4/ CT16B1_MAT1/ SSEL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_4](iocon_pio2_4) module"]
pub type IOCON_PIO2_4 = crate::Reg<u32, _IOCON_PIO2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_4;
#[doc = "`read()` method returns [iocon_pio2_4::R](iocon_pio2_4::R) reader structure"]
impl crate::Readable for IOCON_PIO2_4 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_4::W](iocon_pio2_4::W) writer structure"]
impl crate::Writable for IOCON_PIO2_4 {}
#[doc = "I/O configuration for pin PIO2_4/ CT16B1_MAT1/ SSEL1"]
pub mod iocon_pio2_4;
#[doc = "I/O configuration for pin PIO2_5/ CT32B0_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_5](iocon_pio2_5) module"]
pub type IOCON_PIO2_5 = crate::Reg<u32, _IOCON_PIO2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_5;
#[doc = "`read()` method returns [iocon_pio2_5::R](iocon_pio2_5::R) reader structure"]
impl crate::Readable for IOCON_PIO2_5 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_5::W](iocon_pio2_5::W) writer structure"]
impl crate::Writable for IOCON_PIO2_5 {}
#[doc = "I/O configuration for pin PIO2_5/ CT32B0_MAT0"]
pub mod iocon_pio2_5;
#[doc = "I/O configuration for pin PIO3_5/ CT16B1_CAP1/TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio3_5](iocon_pio3_5) module"]
pub type IOCON_PIO3_5 = crate::Reg<u32, _IOCON_PIO3_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO3_5;
#[doc = "`read()` method returns [iocon_pio3_5::R](iocon_pio3_5::R) reader structure"]
impl crate::Readable for IOCON_PIO3_5 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio3_5::W](iocon_pio3_5::W) writer structure"]
impl crate::Writable for IOCON_PIO3_5 {}
#[doc = "I/O configuration for pin PIO3_5/ CT16B1_CAP1/TXD"]
pub mod iocon_pio3_5;
#[doc = "I/O configuration for pin PIO0_6/SCK0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_6](iocon_pio0_6) module"]
pub type IOCON_PIO0_6 = crate::Reg<u32, _IOCON_PIO0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_6;
#[doc = "`read()` method returns [iocon_pio0_6::R](iocon_pio0_6::R) reader structure"]
impl crate::Readable for IOCON_PIO0_6 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_6::W](iocon_pio0_6::W) writer structure"]
impl crate::Writable for IOCON_PIO0_6 {}
#[doc = "I/O configuration for pin PIO0_6/SCK0"]
pub mod iocon_pio0_6;
#[doc = "I/O configuration for pin PIO0_7/CTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_7](iocon_pio0_7) module"]
pub type IOCON_PIO0_7 = crate::Reg<u32, _IOCON_PIO0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_7;
#[doc = "`read()` method returns [iocon_pio0_7::R](iocon_pio0_7::R) reader structure"]
impl crate::Readable for IOCON_PIO0_7 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_7::W](iocon_pio0_7::W) writer structure"]
impl crate::Writable for IOCON_PIO0_7 {}
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub mod iocon_pio0_7;
#[doc = "I/O configuration for pin PIO2_9/ CT32B0_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_9](iocon_pio2_9) module"]
pub type IOCON_PIO2_9 = crate::Reg<u32, _IOCON_PIO2_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_9;
#[doc = "`read()` method returns [iocon_pio2_9::R](iocon_pio2_9::R) reader structure"]
impl crate::Readable for IOCON_PIO2_9 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_9::W](iocon_pio2_9::W) writer structure"]
impl crate::Writable for IOCON_PIO2_9 {}
#[doc = "I/O configuration for pin PIO2_9/ CT32B0_CAP0"]
pub mod iocon_pio2_9;
#[doc = "I/O configuration for pin PIO2_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_10](iocon_pio2_10) module"]
pub type IOCON_PIO2_10 = crate::Reg<u32, _IOCON_PIO2_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_10;
#[doc = "`read()` method returns [iocon_pio2_10::R](iocon_pio2_10::R) reader structure"]
impl crate::Readable for IOCON_PIO2_10 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_10::W](iocon_pio2_10::W) writer structure"]
impl crate::Writable for IOCON_PIO2_10 {}
#[doc = "I/O configuration for pin PIO2_10"]
pub mod iocon_pio2_10;
#[doc = "I/O configuration for pin PIO2_2/DCD/MISO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_2](iocon_pio2_2) module"]
pub type IOCON_PIO2_2 = crate::Reg<u32, _IOCON_PIO2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_2;
#[doc = "`read()` method returns [iocon_pio2_2::R](iocon_pio2_2::R) reader structure"]
impl crate::Readable for IOCON_PIO2_2 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_2::W](iocon_pio2_2::W) writer structure"]
impl crate::Writable for IOCON_PIO2_2 {}
#[doc = "I/O configuration for pin PIO2_2/DCD/MISO1"]
pub mod iocon_pio2_2;
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_8](iocon_pio0_8) module"]
pub type IOCON_PIO0_8 = crate::Reg<u32, _IOCON_PIO0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_8;
#[doc = "`read()` method returns [iocon_pio0_8::R](iocon_pio0_8::R) reader structure"]
impl crate::Readable for IOCON_PIO0_8 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_8::W](iocon_pio0_8::W) writer structure"]
impl crate::Writable for IOCON_PIO0_8 {}
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub mod iocon_pio0_8;
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio0_9](iocon_pio0_9) module"]
pub type IOCON_PIO0_9 = crate::Reg<u32, _IOCON_PIO0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO0_9;
#[doc = "`read()` method returns [iocon_pio0_9::R](iocon_pio0_9::R) reader structure"]
impl crate::Readable for IOCON_PIO0_9 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio0_9::W](iocon_pio0_9::W) writer structure"]
impl crate::Writable for IOCON_PIO0_9 {}
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
pub mod iocon_pio0_9;
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_swclk_pio0_10](iocon_swclk_pio0_10) module"]
pub type IOCON_SWCLK_PIO0_10 = crate::Reg<u32, _IOCON_SWCLK_PIO0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_SWCLK_PIO0_10;
#[doc = "`read()` method returns [iocon_swclk_pio0_10::R](iocon_swclk_pio0_10::R) reader structure"]
impl crate::Readable for IOCON_SWCLK_PIO0_10 {}
#[doc = "`write(|w| ..)` method takes [iocon_swclk_pio0_10::W](iocon_swclk_pio0_10::W) writer structure"]
impl crate::Writable for IOCON_SWCLK_PIO0_10 {}
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
pub mod iocon_swclk_pio0_10;
#[doc = "I/O configuration for pin PIO1_10/AD6/CT16B1_MAT1/ MISO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio1_10](iocon_pio1_10) module"]
pub type IOCON_PIO1_10 = crate::Reg<u32, _IOCON_PIO1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO1_10;
#[doc = "`read()` method returns [iocon_pio1_10::R](iocon_pio1_10::R) reader structure"]
impl crate::Readable for IOCON_PIO1_10 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio1_10::W](iocon_pio1_10::W) writer structure"]
impl crate::Writable for IOCON_PIO1_10 {}
#[doc = "I/O configuration for pin PIO1_10/AD6/CT16B1_MAT1/ MISO1"]
pub mod iocon_pio1_10;
#[doc = "I/O configuration for pin PIO2_11/SCK0/ CT32B0_CAP1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_11](iocon_pio2_11) module"]
pub type IOCON_PIO2_11 = crate::Reg<u32, _IOCON_PIO2_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_11;
#[doc = "`read()` method returns [iocon_pio2_11::R](iocon_pio2_11::R) reader structure"]
impl crate::Readable for IOCON_PIO2_11 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_11::W](iocon_pio2_11::W) writer structure"]
impl crate::Writable for IOCON_PIO2_11 {}
#[doc = "I/O configuration for pin PIO2_11/SCK0/ CT32B0_CAP1"]
pub mod iocon_pio2_11;
#[doc = "I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_r_pio0_11](iocon_r_pio0_11) module"]
pub type IOCON_R_PIO0_11 = crate::Reg<u32, _IOCON_R_PIO0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_R_PIO0_11;
#[doc = "`read()` method returns [iocon_r_pio0_11::R](iocon_r_pio0_11::R) reader structure"]
impl crate::Readable for IOCON_R_PIO0_11 {}
#[doc = "`write(|w| ..)` method takes [iocon_r_pio0_11::W](iocon_r_pio0_11::W) writer structure"]
impl crate::Writable for IOCON_R_PIO0_11 {}
#[doc = "I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
pub mod iocon_r_pio0_11;
#[doc = "I/O configuration for pin R/PIO1_0/AD1/CT32B1_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_r_pio1_0](iocon_r_pio1_0) module"]
pub type IOCON_R_PIO1_0 = crate::Reg<u32, _IOCON_R_PIO1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_R_PIO1_0;
#[doc = "`read()` method returns [iocon_r_pio1_0::R](iocon_r_pio1_0::R) reader structure"]
impl crate::Readable for IOCON_R_PIO1_0 {}
#[doc = "`write(|w| ..)` method takes [iocon_r_pio1_0::W](iocon_r_pio1_0::W) writer structure"]
impl crate::Writable for IOCON_R_PIO1_0 {}
#[doc = "I/O configuration for pin R/PIO1_0/AD1/CT32B1_CAP0"]
pub mod iocon_r_pio1_0;
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_r_pio1_1](iocon_r_pio1_1) module"]
pub type IOCON_R_PIO1_1 = crate::Reg<u32, _IOCON_R_PIO1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_R_PIO1_1;
#[doc = "`read()` method returns [iocon_r_pio1_1::R](iocon_r_pio1_1::R) reader structure"]
impl crate::Readable for IOCON_R_PIO1_1 {}
#[doc = "`write(|w| ..)` method takes [iocon_r_pio1_1::W](iocon_r_pio1_1::W) writer structure"]
impl crate::Writable for IOCON_R_PIO1_1 {}
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
pub mod iocon_r_pio1_1;
#[doc = "I/O configuration for pin R/PIO1_2/AD3/CT32B1_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_r_pio1_2](iocon_r_pio1_2) module"]
pub type IOCON_R_PIO1_2 = crate::Reg<u32, _IOCON_R_PIO1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_R_PIO1_2;
#[doc = "`read()` method returns [iocon_r_pio1_2::R](iocon_r_pio1_2::R) reader structure"]
impl crate::Readable for IOCON_R_PIO1_2 {}
#[doc = "`write(|w| ..)` method takes [iocon_r_pio1_2::W](iocon_r_pio1_2::W) writer structure"]
impl crate::Writable for IOCON_R_PIO1_2 {}
#[doc = "I/O configuration for pin R/PIO1_2/AD3/CT32B1_MAT1"]
pub mod iocon_r_pio1_2;
#[doc = "I/O configuration for pin PIO3_0/DTR/CT16B0_MAT0/TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio3_0](iocon_pio3_0) module"]
pub type IOCON_PIO3_0 = crate::Reg<u32, _IOCON_PIO3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO3_0;
#[doc = "`read()` method returns [iocon_pio3_0::R](iocon_pio3_0::R) reader structure"]
impl crate::Readable for IOCON_PIO3_0 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio3_0::W](iocon_pio3_0::W) writer structure"]
impl crate::Writable for IOCON_PIO3_0 {}
#[doc = "I/O configuration for pin PIO3_0/DTR/CT16B0_MAT0/TXD"]
pub mod iocon_pio3_0;
#[doc = "I/O configuration for pin PIO3_1/DSR/CT16B0_MAT1/RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio3_1](iocon_pio3_1) module"]
pub type IOCON_PIO3_1 = crate::Reg<u32, _IOCON_PIO3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO3_1;
#[doc = "`read()` method returns [iocon_pio3_1::R](iocon_pio3_1::R) reader structure"]
impl crate::Readable for IOCON_PIO3_1 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio3_1::W](iocon_pio3_1::W) writer structure"]
impl crate::Writable for IOCON_PIO3_1 {}
#[doc = "I/O configuration for pin PIO3_1/DSR/CT16B0_MAT1/RXD"]
pub mod iocon_pio3_1;
#[doc = "I/O configuration for pin PIO2_3/RI/MOSI1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio2_3](iocon_pio2_3) module"]
pub type IOCON_PIO2_3 = crate::Reg<u32, _IOCON_PIO2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO2_3;
#[doc = "`read()` method returns [iocon_pio2_3::R](iocon_pio2_3::R) reader structure"]
impl crate::Readable for IOCON_PIO2_3 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio2_3::W](iocon_pio2_3::W) writer structure"]
impl crate::Writable for IOCON_PIO2_3 {}
#[doc = "I/O configuration for pin PIO2_3/RI/MOSI1"]
pub mod iocon_pio2_3;
#[doc = "I/O configuration for pin SWDIO/PIO1_3/AD4/CT32B1_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_swdio_pio1_3](iocon_swdio_pio1_3) module"]
pub type IOCON_SWDIO_PIO1_3 = crate::Reg<u32, _IOCON_SWDIO_PIO1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_SWDIO_PIO1_3;
#[doc = "`read()` method returns [iocon_swdio_pio1_3::R](iocon_swdio_pio1_3::R) reader structure"]
impl crate::Readable for IOCON_SWDIO_PIO1_3 {}
#[doc = "`write(|w| ..)` method takes [iocon_swdio_pio1_3::W](iocon_swdio_pio1_3::W) writer structure"]
impl crate::Writable for IOCON_SWDIO_PIO1_3 {}
#[doc = "I/O configuration for pin SWDIO/PIO1_3/AD4/CT32B1_MAT2"]
pub mod iocon_swdio_pio1_3;
#[doc = "I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio1_4](iocon_pio1_4) module"]
pub type IOCON_PIO1_4 = crate::Reg<u32, _IOCON_PIO1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO1_4;
#[doc = "`read()` method returns [iocon_pio1_4::R](iocon_pio1_4::R) reader structure"]
impl crate::Readable for IOCON_PIO1_4 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio1_4::W](iocon_pio1_4::W) writer structure"]
impl crate::Writable for IOCON_PIO1_4 {}
#[doc = "I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
pub mod iocon_pio1_4;
#[doc = "I/O configuration for pin PIO1_11/AD7/CT32B1_CAP1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio1_11](iocon_pio1_11) module"]
pub type IOCON_PIO1_11 = crate::Reg<u32, _IOCON_PIO1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO1_11;
#[doc = "`read()` method returns [iocon_pio1_11::R](iocon_pio1_11::R) reader structure"]
impl crate::Readable for IOCON_PIO1_11 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio1_11::W](iocon_pio1_11::W) writer structure"]
impl crate::Writable for IOCON_PIO1_11 {}
#[doc = "I/O configuration for pin PIO1_11/AD7/CT32B1_CAP1"]
pub mod iocon_pio1_11;
#[doc = "I/O configuration for pin PIO3_2/DCD/ CT16B0_MAT2/SCK1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio3_2](iocon_pio3_2) module"]
pub type IOCON_PIO3_2 = crate::Reg<u32, _IOCON_PIO3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO3_2;
#[doc = "`read()` method returns [iocon_pio3_2::R](iocon_pio3_2::R) reader structure"]
impl crate::Readable for IOCON_PIO3_2 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio3_2::W](iocon_pio3_2::W) writer structure"]
impl crate::Writable for IOCON_PIO3_2 {}
#[doc = "I/O configuration for pin PIO3_2/DCD/ CT16B0_MAT2/SCK1"]
pub mod iocon_pio3_2;
#[doc = "I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio1_5](iocon_pio1_5) module"]
pub type IOCON_PIO1_5 = crate::Reg<u32, _IOCON_PIO1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO1_5;
#[doc = "`read()` method returns [iocon_pio1_5::R](iocon_pio1_5::R) reader structure"]
impl crate::Readable for IOCON_PIO1_5 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio1_5::W](iocon_pio1_5::W) writer structure"]
impl crate::Writable for IOCON_PIO1_5 {}
#[doc = "I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
pub mod iocon_pio1_5;
#[doc = "I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio1_6](iocon_pio1_6) module"]
pub type IOCON_PIO1_6 = crate::Reg<u32, _IOCON_PIO1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO1_6;
#[doc = "`read()` method returns [iocon_pio1_6::R](iocon_pio1_6::R) reader structure"]
impl crate::Readable for IOCON_PIO1_6 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio1_6::W](iocon_pio1_6::W) writer structure"]
impl crate::Writable for IOCON_PIO1_6 {}
#[doc = "I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
pub mod iocon_pio1_6;
#[doc = "I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio1_7](iocon_pio1_7) module"]
pub type IOCON_PIO1_7 = crate::Reg<u32, _IOCON_PIO1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO1_7;
#[doc = "`read()` method returns [iocon_pio1_7::R](iocon_pio1_7::R) reader structure"]
impl crate::Readable for IOCON_PIO1_7 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio1_7::W](iocon_pio1_7::W) writer structure"]
impl crate::Writable for IOCON_PIO1_7 {}
#[doc = "I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
pub mod iocon_pio1_7;
#[doc = "I/O configuration for pin PIO3_3/RI/ CT16B0_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_pio3_3](iocon_pio3_3) module"]
pub type IOCON_PIO3_3 = crate::Reg<u32, _IOCON_PIO3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_PIO3_3;
#[doc = "`read()` method returns [iocon_pio3_3::R](iocon_pio3_3::R) reader structure"]
impl crate::Readable for IOCON_PIO3_3 {}
#[doc = "`write(|w| ..)` method takes [iocon_pio3_3::W](iocon_pio3_3::W) writer structure"]
impl crate::Writable for IOCON_PIO3_3 {}
#[doc = "I/O configuration for pin PIO3_3/RI/ CT16B0_CAP0"]
pub mod iocon_pio3_3;
#[doc = "SCK0 pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_sck0_loc](iocon_sck0_loc) module"]
pub type IOCON_SCK0_LOC = crate::Reg<u32, _IOCON_SCK0_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_SCK0_LOC;
#[doc = "`read()` method returns [iocon_sck0_loc::R](iocon_sck0_loc::R) reader structure"]
impl crate::Readable for IOCON_SCK0_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_sck0_loc::W](iocon_sck0_loc::W) writer structure"]
impl crate::Writable for IOCON_SCK0_LOC {}
#[doc = "SCK0 pin location select register"]
pub mod iocon_sck0_loc;
#[doc = "DSR pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_dsr_loc](iocon_dsr_loc) module"]
pub type IOCON_DSR_LOC = crate::Reg<u32, _IOCON_DSR_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_DSR_LOC;
#[doc = "`read()` method returns [iocon_dsr_loc::R](iocon_dsr_loc::R) reader structure"]
impl crate::Readable for IOCON_DSR_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_dsr_loc::W](iocon_dsr_loc::W) writer structure"]
impl crate::Writable for IOCON_DSR_LOC {}
#[doc = "DSR pin location select register"]
pub mod iocon_dsr_loc;
#[doc = "DCD pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_dcd_loc](iocon_dcd_loc) module"]
pub type IOCON_DCD_LOC = crate::Reg<u32, _IOCON_DCD_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_DCD_LOC;
#[doc = "`read()` method returns [iocon_dcd_loc::R](iocon_dcd_loc::R) reader structure"]
impl crate::Readable for IOCON_DCD_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_dcd_loc::W](iocon_dcd_loc::W) writer structure"]
impl crate::Writable for IOCON_DCD_LOC {}
#[doc = "DCD pin location select register"]
pub mod iocon_dcd_loc;
#[doc = "RI pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_ri_loc](iocon_ri_loc) module"]
pub type IOCON_RI_LOC = crate::Reg<u32, _IOCON_RI_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_RI_LOC;
#[doc = "`read()` method returns [iocon_ri_loc::R](iocon_ri_loc::R) reader structure"]
impl crate::Readable for IOCON_RI_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_ri_loc::W](iocon_ri_loc::W) writer structure"]
impl crate::Writable for IOCON_RI_LOC {}
#[doc = "RI pin location select register"]
pub mod iocon_ri_loc;
#[doc = "SSEL1 pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_ssel1_loc](iocon_ssel1_loc) module"]
pub type IOCON_SSEL1_LOC = crate::Reg<u32, _IOCON_SSEL1_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_SSEL1_LOC;
#[doc = "`read()` method returns [iocon_ssel1_loc::R](iocon_ssel1_loc::R) reader structure"]
impl crate::Readable for IOCON_SSEL1_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_ssel1_loc::W](iocon_ssel1_loc::W) writer structure"]
impl crate::Writable for IOCON_SSEL1_LOC {}
#[doc = "SSEL1 pin location select register"]
pub mod iocon_ssel1_loc;
#[doc = "CT16B0_CAP0 pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_ct16b0_cap0_loc](iocon_ct16b0_cap0_loc) module"]
pub type IOCON_CT16B0_CAP0_LOC = crate::Reg<u32, _IOCON_CT16B0_CAP0_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_CT16B0_CAP0_LOC;
#[doc = "`read()` method returns [iocon_ct16b0_cap0_loc::R](iocon_ct16b0_cap0_loc::R) reader structure"]
impl crate::Readable for IOCON_CT16B0_CAP0_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_ct16b0_cap0_loc::W](iocon_ct16b0_cap0_loc::W) writer structure"]
impl crate::Writable for IOCON_CT16B0_CAP0_LOC {}
#[doc = "CT16B0_CAP0 pin location select register"]
pub mod iocon_ct16b0_cap0_loc;
#[doc = "SCK1 pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_sck1_loc](iocon_sck1_loc) module"]
pub type IOCON_SCK1_LOC = crate::Reg<u32, _IOCON_SCK1_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_SCK1_LOC;
#[doc = "`read()` method returns [iocon_sck1_loc::R](iocon_sck1_loc::R) reader structure"]
impl crate::Readable for IOCON_SCK1_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_sck1_loc::W](iocon_sck1_loc::W) writer structure"]
impl crate::Writable for IOCON_SCK1_LOC {}
#[doc = "SCK1 pin location select register"]
pub mod iocon_sck1_loc;
#[doc = "MISO1 pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_miso1_loc](iocon_miso1_loc) module"]
pub type IOCON_MISO1_LOC = crate::Reg<u32, _IOCON_MISO1_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_MISO1_LOC;
#[doc = "`read()` method returns [iocon_miso1_loc::R](iocon_miso1_loc::R) reader structure"]
impl crate::Readable for IOCON_MISO1_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_miso1_loc::W](iocon_miso1_loc::W) writer structure"]
impl crate::Writable for IOCON_MISO1_LOC {}
#[doc = "MISO1 pin location select register"]
pub mod iocon_miso1_loc;
#[doc = "MOSI1 pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_mosi1_loc](iocon_mosi1_loc) module"]
pub type IOCON_MOSI1_LOC = crate::Reg<u32, _IOCON_MOSI1_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_MOSI1_LOC;
#[doc = "`read()` method returns [iocon_mosi1_loc::R](iocon_mosi1_loc::R) reader structure"]
impl crate::Readable for IOCON_MOSI1_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_mosi1_loc::W](iocon_mosi1_loc::W) writer structure"]
impl crate::Writable for IOCON_MOSI1_LOC {}
#[doc = "MOSI1 pin location select register"]
pub mod iocon_mosi1_loc;
#[doc = "CT32B0_CAP0 pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_ct32b0_cap0_loc](iocon_ct32b0_cap0_loc) module"]
pub type IOCON_CT32B0_CAP0_LOC = crate::Reg<u32, _IOCON_CT32B0_CAP0_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_CT32B0_CAP0_LOC;
#[doc = "`read()` method returns [iocon_ct32b0_cap0_loc::R](iocon_ct32b0_cap0_loc::R) reader structure"]
impl crate::Readable for IOCON_CT32B0_CAP0_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_ct32b0_cap0_loc::W](iocon_ct32b0_cap0_loc::W) writer structure"]
impl crate::Writable for IOCON_CT32B0_CAP0_LOC {}
#[doc = "CT32B0_CAP0 pin location select register"]
pub mod iocon_ct32b0_cap0_loc;
#[doc = "RXD pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iocon_rxd_loc](iocon_rxd_loc) module"]
pub type IOCON_RXD_LOC = crate::Reg<u32, _IOCON_RXD_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCON_RXD_LOC;
#[doc = "`read()` method returns [iocon_rxd_loc::R](iocon_rxd_loc::R) reader structure"]
impl crate::Readable for IOCON_RXD_LOC {}
#[doc = "`write(|w| ..)` method takes [iocon_rxd_loc::W](iocon_rxd_loc::W) writer structure"]
impl crate::Writable for IOCON_RXD_LOC {}
#[doc = "RXD pin location select register"]
pub mod iocon_rxd_loc;
