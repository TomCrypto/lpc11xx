#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control Set Register."]
    pub conset: CONSET,
    #[doc = "0x04 - I2C Status Register. During I2C operation, this register provides detailed status codes that allow software to determine the next action needed"]
    pub stat: STAT,
    #[doc = "0x08 - I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register"]
    pub dat: DAT,
    #[doc = "0x0c - I2C Slave Address Register 0. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address"]
    pub adr0: ADR0,
    #[doc = "0x10 - SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock"]
    pub sclh: SCLH,
    #[doc = "0x14 - SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. I2nSCLL and I2nSCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode"]
    pub scll: SCLL,
    #[doc = "0x18 - I2C Control Clear Register."]
    pub conclr: CONCLR,
    #[doc = "0x1c - Monitor mode control register"]
    pub mmctrl: MMCTRL,
    #[doc = "0x20 - I2C Slave Address Register 1. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address"]
    pub adr1: ADR,
    #[doc = "0x24 - I2C Slave Address Register 1. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address"]
    pub adr2: ADR,
    #[doc = "0x28 - I2C Slave Address Register 1. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address"]
    pub adr3: ADR,
    #[doc = "0x2c - Data buffer register. The contents of the 8 MSBs of the I2DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus"]
    pub data_buffer: DATA_BUFFER,
    #[doc = "0x30 - I2C Slave address mask register 0. This mask register is associated with I2ADR0 to determine an address match. The mask register has no effect when comparing to the General Call address (0000000)"]
    pub mask: [MASK; 4],
}
#[doc = "I2C Control Set Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conset](conset) module"]
pub type CONSET = crate::Reg<u32, _CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONSET;
#[doc = "`read()` method returns [conset::R](conset::R) reader structure"]
impl crate::Readable for CONSET {}
#[doc = "`write(|w| ..)` method takes [conset::W](conset::W) writer structure"]
impl crate::Writable for CONSET {}
#[doc = "I2C Control Set Register."]
pub mod conset;
#[doc = "I2C Status Register. During I2C operation, this register provides detailed status codes that allow software to determine the next action needed\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "I2C Status Register. During I2C operation, this register provides detailed status codes that allow software to determine the next action needed"]
pub mod stat;
#[doc = "I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dat](dat) module"]
pub type DAT = crate::Reg<u32, _DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAT;
#[doc = "`read()` method returns [dat::R](dat::R) reader structure"]
impl crate::Readable for DAT {}
#[doc = "`write(|w| ..)` method takes [dat::W](dat::W) writer structure"]
impl crate::Writable for DAT {}
#[doc = "I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register"]
pub mod dat;
#[doc = "I2C Slave Address Register 0. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adr0](adr0) module"]
pub type ADR0 = crate::Reg<u32, _ADR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADR0;
#[doc = "`read()` method returns [adr0::R](adr0::R) reader structure"]
impl crate::Readable for ADR0 {}
#[doc = "`write(|w| ..)` method takes [adr0::W](adr0::W) writer structure"]
impl crate::Writable for ADR0 {}
#[doc = "I2C Slave Address Register 0. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address"]
pub mod adr0;
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sclh](sclh) module"]
pub type SCLH = crate::Reg<u32, _SCLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCLH;
#[doc = "`read()` method returns [sclh::R](sclh::R) reader structure"]
impl crate::Readable for SCLH {}
#[doc = "`write(|w| ..)` method takes [sclh::W](sclh::W) writer structure"]
impl crate::Writable for SCLH {}
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock"]
pub mod sclh;
#[doc = "SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. I2nSCLL and I2nSCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scll](scll) module"]
pub type SCLL = crate::Reg<u32, _SCLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCLL;
#[doc = "`read()` method returns [scll::R](scll::R) reader structure"]
impl crate::Readable for SCLL {}
#[doc = "`write(|w| ..)` method takes [scll::W](scll::W) writer structure"]
impl crate::Writable for SCLL {}
#[doc = "SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. I2nSCLL and I2nSCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode"]
pub mod scll;
#[doc = "I2C Control Clear Register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conclr](conclr) module"]
pub type CONCLR = crate::Reg<u32, _CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONCLR;
#[doc = "`write(|w| ..)` method takes [conclr::W](conclr::W) writer structure"]
impl crate::Writable for CONCLR {}
#[doc = "I2C Control Clear Register."]
pub mod conclr;
#[doc = "Monitor mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmctrl](mmctrl) module"]
pub type MMCTRL = crate::Reg<u32, _MMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCTRL;
#[doc = "`read()` method returns [mmctrl::R](mmctrl::R) reader structure"]
impl crate::Readable for MMCTRL {}
#[doc = "`write(|w| ..)` method takes [mmctrl::W](mmctrl::W) writer structure"]
impl crate::Writable for MMCTRL {}
#[doc = "Monitor mode control register"]
pub mod mmctrl;
#[doc = "I2C Slave Address Register 1. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adr](adr) module"]
pub type ADR = crate::Reg<u32, _ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADR;
#[doc = "`read()` method returns [adr::R](adr::R) reader structure"]
impl crate::Readable for ADR {}
#[doc = "`write(|w| ..)` method takes [adr::W](adr::W) writer structure"]
impl crate::Writable for ADR {}
#[doc = "I2C Slave Address Register 1. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address"]
pub mod adr;
#[doc = "Data buffer register. The contents of the 8 MSBs of the I2DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_buffer](data_buffer) module"]
pub type DATA_BUFFER = crate::Reg<u32, _DATA_BUFFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_BUFFER;
#[doc = "`read()` method returns [data_buffer::R](data_buffer::R) reader structure"]
impl crate::Readable for DATA_BUFFER {}
#[doc = "Data buffer register. The contents of the 8 MSBs of the I2DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus"]
pub mod data_buffer;
#[doc = "I2C Slave address mask register 0. This mask register is associated with I2ADR0 to determine an address match. The mask register has no effect when comparing to the General Call address (0000000)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "I2C Slave address mask register 0. This mask register is associated with I2ADR0 to determine an address match. The mask register has no effect when comparing to the General Call address (0000000)"]
pub mod mask;
