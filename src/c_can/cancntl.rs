#[doc = "Reader of register CANCNTL"]
pub type R = crate::R<u32, super::CANCNTL>;
#[doc = "Writer for register CANCNTL"]
pub type W = crate::W<u32, super::CANCNTL>;
#[doc = "Register CANCNTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CANCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Initialization.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "1: Initialization is started. On reset, software needs to initialize the CAN controller"]
    INITIALIZATION,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        match variant {
            INIT_A::NORMAL_OPERATION_ => false,
            INIT_A::INITIALIZATION => true,
        }
    }
}
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<bool, INIT_A>;
impl INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::NORMAL_OPERATION_,
            true => INIT_A::INITIALIZATION,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline(always)]
    pub fn is_normal_operation_(&self) -> bool {
        *self == INIT_A::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `INITIALIZATION`"]
    #[inline(always)]
    pub fn is_initialization(&self) -> bool {
        *self == INIT_A::INITIALIZATION
    }
}
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(INIT_A::NORMAL_OPERATION_)
    }
    #[doc = "Initialization is started. On reset, software needs to initialize the CAN controller"]
    #[inline(always)]
    pub fn initialization(self) -> &'a mut W {
        self.variant(INIT_A::INITIALIZATION)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Module interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: Disable CAN interrupts. The interrupt line is always HIGH"]
    DISABLE_CAN_INTERRUP,
    #[doc = "1: Enable CAN interrupts. The interrupt line is set to LOW and remains LOW until all pending interrupts are cleared"]
    ENABLE_CAN_INTERRUPT,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        match variant {
            IE_A::DISABLE_CAN_INTERRUP => false,
            IE_A::ENABLE_CAN_INTERRUPT => true,
        }
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, IE_A>;
impl IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::DISABLE_CAN_INTERRUP,
            true => IE_A::ENABLE_CAN_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_CAN_INTERRUP`"]
    #[inline(always)]
    pub fn is_disable_can_interrup(&self) -> bool {
        *self == IE_A::DISABLE_CAN_INTERRUP
    }
    #[doc = "Checks if the value of the field is `ENABLE_CAN_INTERRUPT`"]
    #[inline(always)]
    pub fn is_enable_can_interrupt(&self) -> bool {
        *self == IE_A::ENABLE_CAN_INTERRUPT
    }
}
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CAN interrupts. The interrupt line is always HIGH"]
    #[inline(always)]
    pub fn disable_can_interrup(self) -> &'a mut W {
        self.variant(IE_A::DISABLE_CAN_INTERRUP)
    }
    #[doc = "Enable CAN interrupts. The interrupt line is set to LOW and remains LOW until all pending interrupts are cleared"]
    #[inline(always)]
    pub fn enable_can_interrupt(self) -> &'a mut W {
        self.variant(IE_A::ENABLE_CAN_INTERRUPT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Status change interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIE_A {
    #[doc = "0: Disable status change interrupts. No status change interrupt will be generated"]
    DISABLE_STATUS_CHANG,
    #[doc = "1: Enable status change interrupts. A status change interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected"]
    ENABLE_STATUS_CHANGE,
}
impl From<SIE_A> for bool {
    #[inline(always)]
    fn from(variant: SIE_A) -> Self {
        match variant {
            SIE_A::DISABLE_STATUS_CHANG => false,
            SIE_A::ENABLE_STATUS_CHANGE => true,
        }
    }
}
#[doc = "Reader of field `SIE`"]
pub type SIE_R = crate::R<bool, SIE_A>;
impl SIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIE_A {
        match self.bits {
            false => SIE_A::DISABLE_STATUS_CHANG,
            true => SIE_A::ENABLE_STATUS_CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_STATUS_CHANG`"]
    #[inline(always)]
    pub fn is_disable_status_chang(&self) -> bool {
        *self == SIE_A::DISABLE_STATUS_CHANG
    }
    #[doc = "Checks if the value of the field is `ENABLE_STATUS_CHANGE`"]
    #[inline(always)]
    pub fn is_enable_status_change(&self) -> bool {
        *self == SIE_A::ENABLE_STATUS_CHANGE
    }
}
#[doc = "Write proxy for field `SIE`"]
pub struct SIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable status change interrupts. No status change interrupt will be generated"]
    #[inline(always)]
    pub fn disable_status_chang(self) -> &'a mut W {
        self.variant(SIE_A::DISABLE_STATUS_CHANG)
    }
    #[doc = "Enable status change interrupts. A status change interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected"]
    #[inline(always)]
    pub fn enable_status_change(self) -> &'a mut W {
        self.variant(SIE_A::ENABLE_STATUS_CHANGE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Error interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIE_A {
    #[doc = "0: Disable error interrupt. No error status interrupt will be generated"]
    DISABLE_ERROR_INTERR,
    #[doc = "1: Enable error interrupt. A change in the bits BOFF or EWARN in the CANSTAT registers will generate an interrupt"]
    ENABLE_ERROR_INTERRU,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        match variant {
            EIE_A::DISABLE_ERROR_INTERR => false,
            EIE_A::ENABLE_ERROR_INTERRU => true,
        }
    }
}
#[doc = "Reader of field `EIE`"]
pub type EIE_R = crate::R<bool, EIE_A>;
impl EIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::DISABLE_ERROR_INTERR,
            true => EIE_A::ENABLE_ERROR_INTERRU,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_ERROR_INTERR`"]
    #[inline(always)]
    pub fn is_disable_error_interr(&self) -> bool {
        *self == EIE_A::DISABLE_ERROR_INTERR
    }
    #[doc = "Checks if the value of the field is `ENABLE_ERROR_INTERRU`"]
    #[inline(always)]
    pub fn is_enable_error_interru(&self) -> bool {
        *self == EIE_A::ENABLE_ERROR_INTERRU
    }
}
#[doc = "Write proxy for field `EIE`"]
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable error interrupt. No error status interrupt will be generated"]
    #[inline(always)]
    pub fn disable_error_interr(self) -> &'a mut W {
        self.variant(EIE_A::DISABLE_ERROR_INTERR)
    }
    #[doc = "Enable error interrupt. A change in the bits BOFF or EWARN in the CANSTAT registers will generate an interrupt"]
    #[inline(always)]
    pub fn enable_error_interru(self) -> &'a mut W {
        self.variant(EIE_A::ENABLE_ERROR_INTERRU)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Disable automatic retransmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAR_A {
    #[doc = "0: Automatic retransmission of disturbed messages enabled"]
    ENABLED,
    #[doc = "1: Automatic retransmission disabled"]
    DISABLED,
}
impl From<DAR_A> for bool {
    #[inline(always)]
    fn from(variant: DAR_A) -> Self {
        match variant {
            DAR_A::ENABLED => false,
            DAR_A::DISABLED => true,
        }
    }
}
#[doc = "Reader of field `DAR`"]
pub type DAR_R = crate::R<bool, DAR_A>;
impl DAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAR_A {
        match self.bits {
            false => DAR_A::ENABLED,
            true => DAR_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DAR_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DAR_A::DISABLED
    }
}
#[doc = "Write proxy for field `DAR`"]
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic retransmission of disturbed messages enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAR_A::ENABLED)
    }
    #[doc = "Automatic retransmission disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAR_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Configuration change enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    #[doc = "0: The CPU has no write access to the bit timing register"]
    NOACCESS,
    #[doc = "1: The CPU has write access to the CANBT register while the INIT bit is one"]
    ACCESS,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        match variant {
            CCE_A::NOACCESS => false,
            CCE_A::ACCESS => true,
        }
    }
}
#[doc = "Reader of field `CCE`"]
pub type CCE_R = crate::R<bool, CCE_A>;
impl CCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::NOACCESS,
            true => CCE_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_noaccess(&self) -> bool {
        *self == CCE_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == CCE_A::ACCESS
    }
}
#[doc = "Write proxy for field `CCE`"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CPU has no write access to the bit timing register"]
    #[inline(always)]
    pub fn noaccess(self) -> &'a mut W {
        self.variant(CCE_A::NOACCESS)
    }
    #[doc = "The CPU has write access to the CANBT register while the INIT bit is one"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(CCE_A::ACCESS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Test mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "1: Test mode"]
    TEST_MODE_,
}
impl From<TEST_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_A) -> Self {
        match variant {
            TEST_A::NORMAL_OPERATION_ => false,
            TEST_A::TEST_MODE_ => true,
        }
    }
}
#[doc = "Reader of field `TEST`"]
pub type TEST_R = crate::R<bool, TEST_A>;
impl TEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_A {
        match self.bits {
            false => TEST_A::NORMAL_OPERATION_,
            true => TEST_A::TEST_MODE_,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline(always)]
    pub fn is_normal_operation_(&self) -> bool {
        *self == TEST_A::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `TEST_MODE_`"]
    #[inline(always)]
    pub fn is_test_mode_(&self) -> bool {
        *self == TEST_A::TEST_MODE_
    }
}
#[doc = "Write proxy for field `TEST`"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(TEST_A::NORMAL_OPERATION_)
    }
    #[doc = "Test mode"]
    #[inline(always)]
    pub fn test_mode_(self) -> &'a mut W {
        self.variant(TEST_A::TEST_MODE_)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Initialization."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module interrupt enable."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status change interrupt enable."]
    #[inline(always)]
    pub fn sie(&self) -> SIE_R {
        SIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable."]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable automatic retransmission."]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configuration change enable."]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test mode enable."]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization."]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - Module interrupt enable."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 2 - Status change interrupt enable."]
    #[inline(always)]
    pub fn sie(&mut self) -> SIE_W {
        SIE_W { w: self }
    }
    #[doc = "Bit 3 - Error interrupt enable."]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    #[doc = "Bit 5 - Disable automatic retransmission."]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
    #[doc = "Bit 6 - Configuration change enable."]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 7 - Test mode enable."]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
}
