#[doc = r"Register block"]
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
    _reserved_8_canif1_cmdmsk: [u8; 4usize],
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
    _reserved_19_canif2_cmdmsk: [u8; 4usize],
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
impl RegisterBlock {
    #[doc = "0x24 - Message interface command mask - read direction"]
    #[inline(always)]
    pub fn canif1_cmdmsk_r(&self) -> &CANIF_CMDMSK_R {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const CANIF_CMDMSK_R) }
    }
    #[doc = "0x24 - Message interface command mask - read direction"]
    #[inline(always)]
    pub fn canif1_cmdmsk_r_mut(&self) -> &mut CANIF_CMDMSK_R {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut CANIF_CMDMSK_R) }
    }
    #[doc = "0x24 - Message interface command mask - write direction"]
    #[inline(always)]
    pub fn canif1_cmdmsk_w(&self) -> &CANIF_CMDMSK_W {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const CANIF_CMDMSK_W) }
    }
    #[doc = "0x24 - Message interface command mask - write direction"]
    #[inline(always)]
    pub fn canif1_cmdmsk_w_mut(&self) -> &mut CANIF_CMDMSK_W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut CANIF_CMDMSK_W) }
    }
    #[doc = "0x84 - Message interface command mask - read direction"]
    #[inline(always)]
    pub fn canif2_cmdmsk_r(&self) -> &CANIF_CMDMSK_R {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const CANIF_CMDMSK_R) }
    }
    #[doc = "0x84 - Message interface command mask - read direction"]
    #[inline(always)]
    pub fn canif2_cmdmsk_r_mut(&self) -> &mut CANIF_CMDMSK_R {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut CANIF_CMDMSK_R) }
    }
    #[doc = "0x84 - Message interface command mask - write direction"]
    #[inline(always)]
    pub fn canif2_cmdmsk_w(&self) -> &CANIF_CMDMSK_W {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const CANIF_CMDMSK_W) }
    }
    #[doc = "0x84 - Message interface command mask - write direction"]
    #[inline(always)]
    pub fn canif2_cmdmsk_w_mut(&self) -> &mut CANIF_CMDMSK_W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(132usize) as *mut CANIF_CMDMSK_W) }
    }
}
#[doc = "CAN control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cancntl](cancntl) module"]
pub type CANCNTL = crate::Reg<u32, _CANCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANCNTL;
#[doc = "`read()` method returns [cancntl::R](cancntl::R) reader structure"]
impl crate::Readable for CANCNTL {}
#[doc = "`write(|w| ..)` method takes [cancntl::W](cancntl::W) writer structure"]
impl crate::Writable for CANCNTL {}
#[doc = "CAN control"]
pub mod cancntl;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canstat](canstat) module"]
pub type CANSTAT = crate::Reg<u32, _CANSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANSTAT;
#[doc = "`read()` method returns [canstat::R](canstat::R) reader structure"]
impl crate::Readable for CANSTAT {}
#[doc = "`write(|w| ..)` method takes [canstat::W](canstat::W) writer structure"]
impl crate::Writable for CANSTAT {}
#[doc = "Status register"]
pub mod canstat;
#[doc = "Error counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canec](canec) module"]
pub type CANEC = crate::Reg<u32, _CANEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANEC;
#[doc = "`read()` method returns [canec::R](canec::R) reader structure"]
impl crate::Readable for CANEC {}
#[doc = "Error counter"]
pub mod canec;
#[doc = "Bit timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canbt](canbt) module"]
pub type CANBT = crate::Reg<u32, _CANBT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANBT;
#[doc = "`read()` method returns [canbt::R](canbt::R) reader structure"]
impl crate::Readable for CANBT {}
#[doc = "`write(|w| ..)` method takes [canbt::W](canbt::W) writer structure"]
impl crate::Writable for CANBT {}
#[doc = "Bit timing register"]
pub mod canbt;
#[doc = "Interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canint](canint) module"]
pub type CANINT = crate::Reg<u32, _CANINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANINT;
#[doc = "`read()` method returns [canint::R](canint::R) reader structure"]
impl crate::Readable for CANINT {}
#[doc = "Interrupt register"]
pub mod canint;
#[doc = "Test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cantest](cantest) module"]
pub type CANTEST = crate::Reg<u32, _CANTEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTEST;
#[doc = "`read()` method returns [cantest::R](cantest::R) reader structure"]
impl crate::Readable for CANTEST {}
#[doc = "`write(|w| ..)` method takes [cantest::W](cantest::W) writer structure"]
impl crate::Writable for CANTEST {}
#[doc = "Test register"]
pub mod cantest;
#[doc = "Baud rate prescaler extension register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canbrpe](canbrpe) module"]
pub type CANBRPE = crate::Reg<u32, _CANBRPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANBRPE;
#[doc = "`read()` method returns [canbrpe::R](canbrpe::R) reader structure"]
impl crate::Readable for CANBRPE {}
#[doc = "`write(|w| ..)` method takes [canbrpe::W](canbrpe::W) writer structure"]
impl crate::Writable for CANBRPE {}
#[doc = "Baud rate prescaler extension register"]
pub mod canbrpe;
#[doc = "Message interface command request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_cmdreq](canif_cmdreq) module"]
pub type CANIF_CMDREQ = crate::Reg<u32, _CANIF_CMDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_CMDREQ;
#[doc = "`read()` method returns [canif_cmdreq::R](canif_cmdreq::R) reader structure"]
impl crate::Readable for CANIF_CMDREQ {}
#[doc = "`write(|w| ..)` method takes [canif_cmdreq::W](canif_cmdreq::W) writer structure"]
impl crate::Writable for CANIF_CMDREQ {}
#[doc = "Message interface command request"]
pub mod canif_cmdreq;
#[doc = "Message interface command mask - write direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_cmdmsk_w](canif_cmdmsk_w) module"]
pub type CANIF_CMDMSK_W = crate::Reg<u32, _CANIF_CMDMSK_W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_CMDMSK_W;
#[doc = "`read()` method returns [canif_cmdmsk_w::R](canif_cmdmsk_w::R) reader structure"]
impl crate::Readable for CANIF_CMDMSK_W {}
#[doc = "`write(|w| ..)` method takes [canif_cmdmsk_w::W](canif_cmdmsk_w::W) writer structure"]
impl crate::Writable for CANIF_CMDMSK_W {}
#[doc = "Message interface command mask - write direction"]
pub mod canif_cmdmsk_w;
#[doc = "Message interface command mask - read direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_cmdmsk_r](canif_cmdmsk_r) module"]
pub type CANIF_CMDMSK_R = crate::Reg<u32, _CANIF_CMDMSK_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_CMDMSK_R;
#[doc = "`read()` method returns [canif_cmdmsk_r::R](canif_cmdmsk_r::R) reader structure"]
impl crate::Readable for CANIF_CMDMSK_R {}
#[doc = "`write(|w| ..)` method takes [canif_cmdmsk_r::W](canif_cmdmsk_r::W) writer structure"]
impl crate::Writable for CANIF_CMDMSK_R {}
#[doc = "Message interface command mask - read direction"]
pub mod canif_cmdmsk_r;
#[doc = "Message interface 1 mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_msk1](canif_msk1) module"]
pub type CANIF_MSK1 = crate::Reg<u32, _CANIF_MSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_MSK1;
#[doc = "`read()` method returns [canif_msk1::R](canif_msk1::R) reader structure"]
impl crate::Readable for CANIF_MSK1 {}
#[doc = "`write(|w| ..)` method takes [canif_msk1::W](canif_msk1::W) writer structure"]
impl crate::Writable for CANIF_MSK1 {}
#[doc = "Message interface 1 mask 1"]
pub mod canif_msk1;
#[doc = "Message interface 1 mask 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_msk2](canif_msk2) module"]
pub type CANIF_MSK2 = crate::Reg<u32, _CANIF_MSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_MSK2;
#[doc = "`read()` method returns [canif_msk2::R](canif_msk2::R) reader structure"]
impl crate::Readable for CANIF_MSK2 {}
#[doc = "`write(|w| ..)` method takes [canif_msk2::W](canif_msk2::W) writer structure"]
impl crate::Writable for CANIF_MSK2 {}
#[doc = "Message interface 1 mask 2"]
pub mod canif_msk2;
#[doc = "Message interface 1 arbitration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_arb1](canif_arb1) module"]
pub type CANIF_ARB1 = crate::Reg<u32, _CANIF_ARB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_ARB1;
#[doc = "`read()` method returns [canif_arb1::R](canif_arb1::R) reader structure"]
impl crate::Readable for CANIF_ARB1 {}
#[doc = "`write(|w| ..)` method takes [canif_arb1::W](canif_arb1::W) writer structure"]
impl crate::Writable for CANIF_ARB1 {}
#[doc = "Message interface 1 arbitration 1"]
pub mod canif_arb1;
#[doc = "Message interface 1 arbitration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_arb2](canif_arb2) module"]
pub type CANIF_ARB2 = crate::Reg<u32, _CANIF_ARB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_ARB2;
#[doc = "`read()` method returns [canif_arb2::R](canif_arb2::R) reader structure"]
impl crate::Readable for CANIF_ARB2 {}
#[doc = "`write(|w| ..)` method takes [canif_arb2::W](canif_arb2::W) writer structure"]
impl crate::Writable for CANIF_ARB2 {}
#[doc = "Message interface 1 arbitration 2"]
pub mod canif_arb2;
#[doc = "Message interface 1 message control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_mctrl](canif_mctrl) module"]
pub type CANIF_MCTRL = crate::Reg<u32, _CANIF_MCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_MCTRL;
#[doc = "`read()` method returns [canif_mctrl::R](canif_mctrl::R) reader structure"]
impl crate::Readable for CANIF_MCTRL {}
#[doc = "`write(|w| ..)` method takes [canif_mctrl::W](canif_mctrl::W) writer structure"]
impl crate::Writable for CANIF_MCTRL {}
#[doc = "Message interface 1 message control"]
pub mod canif_mctrl;
#[doc = "Message interface 1 data A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_da1](canif_da1) module"]
pub type CANIF_DA1 = crate::Reg<u32, _CANIF_DA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_DA1;
#[doc = "`read()` method returns [canif_da1::R](canif_da1::R) reader structure"]
impl crate::Readable for CANIF_DA1 {}
#[doc = "`write(|w| ..)` method takes [canif_da1::W](canif_da1::W) writer structure"]
impl crate::Writable for CANIF_DA1 {}
#[doc = "Message interface 1 data A1"]
pub mod canif_da1;
#[doc = "Message interface 1 data A2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_da2](canif_da2) module"]
pub type CANIF_DA2 = crate::Reg<u32, _CANIF_DA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_DA2;
#[doc = "`read()` method returns [canif_da2::R](canif_da2::R) reader structure"]
impl crate::Readable for CANIF_DA2 {}
#[doc = "`write(|w| ..)` method takes [canif_da2::W](canif_da2::W) writer structure"]
impl crate::Writable for CANIF_DA2 {}
#[doc = "Message interface 1 data A2"]
pub mod canif_da2;
#[doc = "Message interface 1 data B1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_db1](canif_db1) module"]
pub type CANIF_DB1 = crate::Reg<u32, _CANIF_DB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_DB1;
#[doc = "`read()` method returns [canif_db1::R](canif_db1::R) reader structure"]
impl crate::Readable for CANIF_DB1 {}
#[doc = "`write(|w| ..)` method takes [canif_db1::W](canif_db1::W) writer structure"]
impl crate::Writable for CANIF_DB1 {}
#[doc = "Message interface 1 data B1"]
pub mod canif_db1;
#[doc = "Message interface 1 data B2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canif_db2](canif_db2) module"]
pub type CANIF_DB2 = crate::Reg<u32, _CANIF_DB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIF_DB2;
#[doc = "`read()` method returns [canif_db2::R](canif_db2::R) reader structure"]
impl crate::Readable for CANIF_DB2 {}
#[doc = "`write(|w| ..)` method takes [canif_db2::W](canif_db2::W) writer structure"]
impl crate::Writable for CANIF_DB2 {}
#[doc = "Message interface 1 data B2"]
pub mod canif_db2;
#[doc = "Transmission request 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cantxreq1](cantxreq1) module"]
pub type CANTXREQ1 = crate::Reg<u32, _CANTXREQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTXREQ1;
#[doc = "`read()` method returns [cantxreq1::R](cantxreq1::R) reader structure"]
impl crate::Readable for CANTXREQ1 {}
#[doc = "Transmission request 1"]
pub mod cantxreq1;
#[doc = "Transmission request 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cantxreq2](cantxreq2) module"]
pub type CANTXREQ2 = crate::Reg<u32, _CANTXREQ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTXREQ2;
#[doc = "`read()` method returns [cantxreq2::R](cantxreq2::R) reader structure"]
impl crate::Readable for CANTXREQ2 {}
#[doc = "Transmission request 2"]
pub mod cantxreq2;
#[doc = "New data 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cannd1](cannd1) module"]
pub type CANND1 = crate::Reg<u32, _CANND1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANND1;
#[doc = "`read()` method returns [cannd1::R](cannd1::R) reader structure"]
impl crate::Readable for CANND1 {}
#[doc = "New data 1"]
pub mod cannd1;
#[doc = "New data 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cannd2](cannd2) module"]
pub type CANND2 = crate::Reg<u32, _CANND2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANND2;
#[doc = "`read()` method returns [cannd2::R](cannd2::R) reader structure"]
impl crate::Readable for CANND2 {}
#[doc = "New data 2"]
pub mod cannd2;
#[doc = "Interrupt pending 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canir1](canir1) module"]
pub type CANIR1 = crate::Reg<u32, _CANIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIR1;
#[doc = "`read()` method returns [canir1::R](canir1::R) reader structure"]
impl crate::Readable for CANIR1 {}
#[doc = "Interrupt pending 1"]
pub mod canir1;
#[doc = "Interrupt pending 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canir2](canir2) module"]
pub type CANIR2 = crate::Reg<u32, _CANIR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIR2;
#[doc = "`read()` method returns [canir2::R](canir2::R) reader structure"]
impl crate::Readable for CANIR2 {}
#[doc = "Interrupt pending 2"]
pub mod canir2;
#[doc = "Message valid 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canmsgv1](canmsgv1) module"]
pub type CANMSGV1 = crate::Reg<u32, _CANMSGV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANMSGV1;
#[doc = "`read()` method returns [canmsgv1::R](canmsgv1::R) reader structure"]
impl crate::Readable for CANMSGV1 {}
#[doc = "Message valid 1"]
pub mod canmsgv1;
#[doc = "Message valid 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canmsgv2](canmsgv2) module"]
pub type CANMSGV2 = crate::Reg<u32, _CANMSGV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANMSGV2;
#[doc = "`read()` method returns [canmsgv2::R](canmsgv2::R) reader structure"]
impl crate::Readable for CANMSGV2 {}
#[doc = "Message valid 2"]
pub mod canmsgv2;
#[doc = "Can clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canclkdiv](canclkdiv) module"]
pub type CANCLKDIV = crate::Reg<u32, _CANCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANCLKDIV;
#[doc = "`read()` method returns [canclkdiv::R](canclkdiv::R) reader structure"]
impl crate::Readable for CANCLKDIV {}
#[doc = "`write(|w| ..)` method takes [canclkdiv::W](canclkdiv::W) writer structure"]
impl crate::Writable for CANCLKDIV {}
#[doc = "Can clock divider register"]
pub mod canclkdiv;
