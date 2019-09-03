#[doc = "Reader of register SYSAHBCLKCTRL"]
pub type R = crate::R<u32, super::SYSAHBCLKCTRL>;
#[doc = "Writer for register SYSAHBCLKCTRL"]
pub type W = crate::W<u32, super::SYSAHBCLKCTRL>;
#[doc = "Register SYSAHBCLKCTRL `reset()`'s with value 0x085f"]
impl crate::ResetValue for super::SYSAHBCLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x085f
    }
}
#[doc = "Clock control for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU (this bit is read-only).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_A {
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<SYS_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_A) -> Self {
        match variant {
            SYS_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SYS`"]
pub type SYS_R = crate::R<bool, SYS_A>;
impl SYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_A {
        match self.bits {
            true => SYS_A::ENABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYS_A::ENABLE
    }
}
#[doc = "Write proxy for field `SYS`"]
pub struct SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYS_A::ENABLE)
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
#[doc = "Clock control for ROM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        match variant {
            ROM_A::DISABLE => false,
            ROM_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ROM`"]
pub type ROM_R = crate::R<bool, ROM_A>;
impl ROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::DISABLE,
            true => ROM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ROM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ROM_A::ENABLE
    }
}
#[doc = "Write proxy for field `ROM`"]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROM_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROM_A::ENABLE)
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
#[doc = "Clock control for RAM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<RAM_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        match variant {
            RAM_A::DISABLE => false,
            RAM_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `RAM`"]
pub type RAM_R = crate::R<bool, RAM_A>;
impl RAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_A {
        match self.bits {
            false => RAM_A::DISABLE,
            true => RAM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAM_A::ENABLE
    }
}
#[doc = "Write proxy for field `RAM`"]
pub struct RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM_A::ENABLE)
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
#[doc = "Clock control for the flash register interface.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHREG_A {
    #[doc = "0: Disabled"]
    DISABLED,
    #[doc = "1: Enabled"]
    ENABLED,
}
impl From<FLASHREG_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHREG_A) -> Self {
        match variant {
            FLASHREG_A::DISABLED => false,
            FLASHREG_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `FLASHREG`"]
pub type FLASHREG_R = crate::R<bool, FLASHREG_A>;
impl FLASHREG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHREG_A {
        match self.bits {
            false => FLASHREG_A::DISABLED,
            true => FLASHREG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHREG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHREG_A::ENABLED
    }
}
#[doc = "Write proxy for field `FLASHREG`"]
pub struct FLASHREG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHREG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHREG_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHREG_A::ENABLED)
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
#[doc = "Clock control for flash array access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHARRAY_A {
    #[doc = "0: Disabled"]
    DISABLED,
    #[doc = "1: Enabled"]
    ENABLED,
}
impl From<FLASHARRAY_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHARRAY_A) -> Self {
        match variant {
            FLASHARRAY_A::DISABLED => false,
            FLASHARRAY_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `FLASHARRAY`"]
pub type FLASHARRAY_R = crate::R<bool, FLASHARRAY_A>;
impl FLASHARRAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHARRAY_A {
        match self.bits {
            false => FLASHARRAY_A::DISABLED,
            true => FLASHARRAY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHARRAY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHARRAY_A::ENABLED
    }
}
#[doc = "Write proxy for field `FLASHARRAY`"]
pub struct FLASHARRAY_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHARRAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHARRAY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHARRAY_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHARRAY_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Clock control for I2C.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<I2C_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_A) -> Self {
        match variant {
            I2C_A::DISABLE => false,
            I2C_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `I2C`"]
pub type I2C_R = crate::R<bool, I2C_A>;
impl I2C_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_A {
        match self.bits {
            false => I2C_A::DISABLE,
            true => I2C_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2C_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2C_A::ENABLE
    }
}
#[doc = "Write proxy for field `I2C`"]
pub struct I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C_A::ENABLE)
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
#[doc = "Clock control for GPIO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<GPIO_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_A) -> Self {
        match variant {
            GPIO_A::DISABLE => false,
            GPIO_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO`"]
pub type GPIO_R = crate::R<bool, GPIO_A>;
impl GPIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_A {
        match self.bits {
            false => GPIO_A::DISABLE,
            true => GPIO_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO`"]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_A::ENABLE)
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
#[doc = "Clock control for CT16B0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<CT16B0_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B0_A) -> Self {
        match variant {
            CT16B0_A::DISABLE => false,
            CT16B0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CT16B0`"]
pub type CT16B0_R = crate::R<bool, CT16B0_A>;
impl CT16B0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B0_A {
        match self.bits {
            false => CT16B0_A::DISABLE,
            true => CT16B0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT16B0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT16B0_A::ENABLE
    }
}
#[doc = "Write proxy for field `CT16B0`"]
pub struct CT16B0_W<'a> {
    w: &'a mut W,
}
impl<'a> CT16B0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT16B0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT16B0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT16B0_A::ENABLE)
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
#[doc = "Clock control for CT16B1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B1_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<CT16B1_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B1_A) -> Self {
        match variant {
            CT16B1_A::DISABLE => false,
            CT16B1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CT16B1`"]
pub type CT16B1_R = crate::R<bool, CT16B1_A>;
impl CT16B1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B1_A {
        match self.bits {
            false => CT16B1_A::DISABLE,
            true => CT16B1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT16B1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT16B1_A::ENABLE
    }
}
#[doc = "Write proxy for field `CT16B1`"]
pub struct CT16B1_W<'a> {
    w: &'a mut W,
}
impl<'a> CT16B1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT16B1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT16B1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT16B1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Clock control for CT32B0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<CT32B0_A> for bool {
    #[inline(always)]
    fn from(variant: CT32B0_A) -> Self {
        match variant {
            CT32B0_A::DISABLE => false,
            CT32B0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CT32B0`"]
pub type CT32B0_R = crate::R<bool, CT32B0_A>;
impl CT32B0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B0_A {
        match self.bits {
            false => CT32B0_A::DISABLE,
            true => CT32B0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32B0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32B0_A::ENABLE
    }
}
#[doc = "Write proxy for field `CT32B0`"]
pub struct CT32B0_W<'a> {
    w: &'a mut W,
}
impl<'a> CT32B0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT32B0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32B0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32B0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Clock control for CT32B1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B1_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<CT32B1_A> for bool {
    #[inline(always)]
    fn from(variant: CT32B1_A) -> Self {
        match variant {
            CT32B1_A::DISABLE => false,
            CT32B1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CT32B1`"]
pub type CT32B1_R = crate::R<bool, CT32B1_A>;
impl CT32B1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B1_A {
        match self.bits {
            false => CT32B1_A::DISABLE,
            true => CT32B1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32B1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32B1_A::ENABLE
    }
}
#[doc = "Write proxy for field `CT32B1`"]
pub struct CT32B1_W<'a> {
    w: &'a mut W,
}
impl<'a> CT32B1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT32B1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32B1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32B1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Clock control for SPI0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<SSP0_A> for bool {
    #[inline(always)]
    fn from(variant: SSP0_A) -> Self {
        match variant {
            SSP0_A::DISABLE => false,
            SSP0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SSP0`"]
pub type SSP0_R = crate::R<bool, SSP0_A>;
impl SSP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP0_A {
        match self.bits {
            false => SSP0_A::DISABLE,
            true => SSP0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SSP0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SSP0_A::ENABLE
    }
}
#[doc = "Write proxy for field `SSP0`"]
pub struct SSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSP0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSP0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Clock control for UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<UART_A> for bool {
    #[inline(always)]
    fn from(variant: UART_A) -> Self {
        match variant {
            UART_A::DISABLE => false,
            UART_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `UART`"]
pub type UART_R = crate::R<bool, UART_A>;
impl UART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_A {
        match self.bits {
            false => UART_A::DISABLE,
            true => UART_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UART_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UART_A::ENABLE
    }
}
#[doc = "Write proxy for field `UART`"]
pub struct UART_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Clock control for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        match variant {
            ADC_A::DISABLE => false,
            ADC_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, ADC_A>;
impl ADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::DISABLE,
            true => ADC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADC`"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Clock control for WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        match variant {
            WDT_A::DISABLE => false,
            WDT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, WDT_A>;
impl WDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::DISABLE,
            true => WDT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WDT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WDT_A::ENABLE
    }
}
#[doc = "Write proxy for field `WDT`"]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Clock control for the I/O configuration block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        match variant {
            IOCON_A::DISABLE => false,
            IOCON_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `IOCON`"]
pub type IOCON_R = crate::R<bool, IOCON_A>;
impl IOCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::DISABLE,
            true => IOCON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IOCON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IOCON_A::ENABLE
    }
}
#[doc = "Write proxy for field `IOCON`"]
pub struct IOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCON_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCON_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Clock control for C_CAN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<CAN_A> for bool {
    #[inline(always)]
    fn from(variant: CAN_A) -> Self {
        match variant {
            CAN_A::DISABLE => false,
            CAN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CAN`"]
pub type CAN_R = crate::R<bool, CAN_A>;
impl CAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN_A {
        match self.bits {
            false => CAN_A::DISABLE,
            true => CAN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CAN`"]
pub struct CAN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Clock control for SPI1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<SSP1_A> for bool {
    #[inline(always)]
    fn from(variant: SSP1_A) -> Self {
        match variant {
            SSP1_A::DISABLE => false,
            SSP1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SSP1`"]
pub type SSP1_R = crate::R<bool, SSP1_A>;
impl SSP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP1_A {
        match self.bits {
            false => SSP1_A::DISABLE,
            true => SSP1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SSP1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SSP1_A::ENABLE
    }
}
#[doc = "Write proxy for field `SSP1`"]
pub struct SSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSP1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSP1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock control for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU (this bit is read-only)."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock control for ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock control for RAM."]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock control for the flash register interface."]
    #[inline(always)]
    pub fn flashreg(&self) -> FLASHREG_R {
        FLASHREG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock control for flash array access."]
    #[inline(always)]
    pub fn flasharray(&self) -> FLASHARRAY_R {
        FLASHARRAY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock control for I2C."]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clock control for GPIO."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock control for CT16B0."]
    #[inline(always)]
    pub fn ct16b0(&self) -> CT16B0_R {
        CT16B0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clock control for CT16B1."]
    #[inline(always)]
    pub fn ct16b1(&self) -> CT16B1_R {
        CT16B1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clock control for CT32B0."]
    #[inline(always)]
    pub fn ct32b0(&self) -> CT32B0_R {
        CT32B0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clock control for CT32B1."]
    #[inline(always)]
    pub fn ct32b1(&self) -> CT32B1_R {
        CT32B1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Clock control for SPI0."]
    #[inline(always)]
    pub fn ssp0(&self) -> SSP0_R {
        SSP0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Clock control for UART."]
    #[inline(always)]
    pub fn uart(&self) -> UART_R {
        UART_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Clock control for ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clock control for WDT."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clock control for the I/O configuration block."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Clock control for C_CAN."]
    #[inline(always)]
    pub fn can(&self) -> CAN_R {
        CAN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clock control for SPI1."]
    #[inline(always)]
    pub fn ssp1(&self) -> SSP1_R {
        SSP1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock control for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU (this bit is read-only)."]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W {
        SYS_W { w: self }
    }
    #[doc = "Bit 1 - Clock control for ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 2 - Clock control for RAM."]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W {
        RAM_W { w: self }
    }
    #[doc = "Bit 3 - Clock control for the flash register interface."]
    #[inline(always)]
    pub fn flashreg(&mut self) -> FLASHREG_W {
        FLASHREG_W { w: self }
    }
    #[doc = "Bit 4 - Clock control for flash array access."]
    #[inline(always)]
    pub fn flasharray(&mut self) -> FLASHARRAY_W {
        FLASHARRAY_W { w: self }
    }
    #[doc = "Bit 5 - Clock control for I2C."]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W {
        I2C_W { w: self }
    }
    #[doc = "Bit 6 - Clock control for GPIO."]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
    #[doc = "Bit 7 - Clock control for CT16B0."]
    #[inline(always)]
    pub fn ct16b0(&mut self) -> CT16B0_W {
        CT16B0_W { w: self }
    }
    #[doc = "Bit 8 - Clock control for CT16B1."]
    #[inline(always)]
    pub fn ct16b1(&mut self) -> CT16B1_W {
        CT16B1_W { w: self }
    }
    #[doc = "Bit 9 - Clock control for CT32B0."]
    #[inline(always)]
    pub fn ct32b0(&mut self) -> CT32B0_W {
        CT32B0_W { w: self }
    }
    #[doc = "Bit 10 - Clock control for CT32B1."]
    #[inline(always)]
    pub fn ct32b1(&mut self) -> CT32B1_W {
        CT32B1_W { w: self }
    }
    #[doc = "Bit 11 - Clock control for SPI0."]
    #[inline(always)]
    pub fn ssp0(&mut self) -> SSP0_W {
        SSP0_W { w: self }
    }
    #[doc = "Bit 12 - Clock control for UART."]
    #[inline(always)]
    pub fn uart(&mut self) -> UART_W {
        UART_W { w: self }
    }
    #[doc = "Bit 13 - Clock control for ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 15 - Clock control for WDT."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 16 - Clock control for the I/O configuration block."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W {
        IOCON_W { w: self }
    }
    #[doc = "Bit 17 - Clock control for C_CAN."]
    #[inline(always)]
    pub fn can(&mut self) -> CAN_W {
        CAN_W { w: self }
    }
    #[doc = "Bit 18 - Clock control for SPI1."]
    #[inline(always)]
    pub fn ssp1(&mut self) -> SSP1_W {
        SSP1_W { w: self }
    }
}
