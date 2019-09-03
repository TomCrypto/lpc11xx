#[doc = "Reader of register PWMC"]
pub type R = crate::R<u32, super::PWMC>;
#[doc = "Writer for register PWMC"]
pub type W = crate::W<u32, super::PWMC>;
#[doc = "Register PWMC `reset()`'s with value 0"]
impl crate::ResetValue for super::PWMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWM channel 0 enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0_A {
    #[doc = "0: CT32Bn_MAT0 is controlled by EM0"]
    CT32BN_MAT0_IS_CONTR,
    #[doc = "1: PWM mode is enabled for CT32Bn_MAT0"]
    PWM_MODE_IS_ENABLED_,
}
impl From<PWMEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN0_A) -> Self {
        match variant {
            PWMEN0_A::CT32BN_MAT0_IS_CONTR => false,
            PWMEN0_A::PWM_MODE_IS_ENABLED_ => true,
        }
    }
}
#[doc = "Reader of field `PWMEN0`"]
pub type PWMEN0_R = crate::R<bool, PWMEN0_A>;
impl PWMEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN0_A {
        match self.bits {
            false => PWMEN0_A::CT32BN_MAT0_IS_CONTR,
            true => PWMEN0_A::PWM_MODE_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BN_MAT0_IS_CONTR`"]
    #[inline(always)]
    pub fn is_ct32bn_mat0_is_contr(&self) -> bool {
        *self == PWMEN0_A::CT32BN_MAT0_IS_CONTR
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN0_A::PWM_MODE_IS_ENABLED_
    }
}
#[doc = "Write proxy for field `PWMEN0`"]
pub struct PWMEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CT32Bn_MAT0 is controlled by EM0"]
    #[inline(always)]
    pub fn ct32bn_mat0_is_contr(self) -> &'a mut W {
        self.variant(PWMEN0_A::CT32BN_MAT0_IS_CONTR)
    }
    #[doc = "PWM mode is enabled for CT32Bn_MAT0"]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN0_A::PWM_MODE_IS_ENABLED_)
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
#[doc = "PWM channel 1 enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1_A {
    #[doc = "0: CT32Bn_MAT1 is controlled by EM1"]
    CT32BN_MAT1_IS_CONTR,
    #[doc = "1: PWM mode is enabled for CT32Bn_MAT1"]
    PWM_MODE_IS_ENABLED_,
}
impl From<PWMEN1_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN1_A) -> Self {
        match variant {
            PWMEN1_A::CT32BN_MAT1_IS_CONTR => false,
            PWMEN1_A::PWM_MODE_IS_ENABLED_ => true,
        }
    }
}
#[doc = "Reader of field `PWMEN1`"]
pub type PWMEN1_R = crate::R<bool, PWMEN1_A>;
impl PWMEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN1_A {
        match self.bits {
            false => PWMEN1_A::CT32BN_MAT1_IS_CONTR,
            true => PWMEN1_A::PWM_MODE_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BN_MAT1_IS_CONTR`"]
    #[inline(always)]
    pub fn is_ct32bn_mat1_is_contr(&self) -> bool {
        *self == PWMEN1_A::CT32BN_MAT1_IS_CONTR
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN1_A::PWM_MODE_IS_ENABLED_
    }
}
#[doc = "Write proxy for field `PWMEN1`"]
pub struct PWMEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CT32Bn_MAT1 is controlled by EM1"]
    #[inline(always)]
    pub fn ct32bn_mat1_is_contr(self) -> &'a mut W {
        self.variant(PWMEN1_A::CT32BN_MAT1_IS_CONTR)
    }
    #[doc = "PWM mode is enabled for CT32Bn_MAT1"]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN1_A::PWM_MODE_IS_ENABLED_)
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
#[doc = "PWM channel 2 enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2_A {
    #[doc = "0: CT32Bn_MAT2 is controlled by EM2"]
    CT32BN_MAT2_IS_CONTR,
    #[doc = "1: PWM mode is enabled for CT32Bn_MAT2"]
    PWM_MODE_IS_ENABLED_,
}
impl From<PWMEN2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN2_A) -> Self {
        match variant {
            PWMEN2_A::CT32BN_MAT2_IS_CONTR => false,
            PWMEN2_A::PWM_MODE_IS_ENABLED_ => true,
        }
    }
}
#[doc = "Reader of field `PWMEN2`"]
pub type PWMEN2_R = crate::R<bool, PWMEN2_A>;
impl PWMEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN2_A {
        match self.bits {
            false => PWMEN2_A::CT32BN_MAT2_IS_CONTR,
            true => PWMEN2_A::PWM_MODE_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BN_MAT2_IS_CONTR`"]
    #[inline(always)]
    pub fn is_ct32bn_mat2_is_contr(&self) -> bool {
        *self == PWMEN2_A::CT32BN_MAT2_IS_CONTR
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN2_A::PWM_MODE_IS_ENABLED_
    }
}
#[doc = "Write proxy for field `PWMEN2`"]
pub struct PWMEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CT32Bn_MAT2 is controlled by EM2"]
    #[inline(always)]
    pub fn ct32bn_mat2_is_contr(self) -> &'a mut W {
        self.variant(PWMEN2_A::CT32BN_MAT2_IS_CONTR)
    }
    #[doc = "PWM mode is enabled for CT32Bn_MAT2"]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN2_A::PWM_MODE_IS_ENABLED_)
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
#[doc = "PWM channel 3 enable Note: It is recommended to use match channel 3 to set the PWM cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3_A {
    #[doc = "0: CT32Bn_MAT3 is controlled by EM3"]
    CT32BN_MAT3_IS_CONTR,
    #[doc = "1: PWM mode is enabled for CT32Bn_MAT3"]
    PWM_MODE_IS_ENABLED_,
}
impl From<PWMEN3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN3_A) -> Self {
        match variant {
            PWMEN3_A::CT32BN_MAT3_IS_CONTR => false,
            PWMEN3_A::PWM_MODE_IS_ENABLED_ => true,
        }
    }
}
#[doc = "Reader of field `PWMEN3`"]
pub type PWMEN3_R = crate::R<bool, PWMEN3_A>;
impl PWMEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN3_A {
        match self.bits {
            false => PWMEN3_A::CT32BN_MAT3_IS_CONTR,
            true => PWMEN3_A::PWM_MODE_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BN_MAT3_IS_CONTR`"]
    #[inline(always)]
    pub fn is_ct32bn_mat3_is_contr(&self) -> bool {
        *self == PWMEN3_A::CT32BN_MAT3_IS_CONTR
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN3_A::PWM_MODE_IS_ENABLED_
    }
}
#[doc = "Write proxy for field `PWMEN3`"]
pub struct PWMEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CT32Bn_MAT3 is controlled by EM3"]
    #[inline(always)]
    pub fn ct32bn_mat3_is_contr(self) -> &'a mut W {
        self.variant(PWMEN3_A::CT32BN_MAT3_IS_CONTR)
    }
    #[doc = "PWM mode is enabled for CT32Bn_MAT3"]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN3_A::PWM_MODE_IS_ENABLED_)
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
impl R {
    #[doc = "Bit 0 - PWM channel 0 enable."]
    #[inline(always)]
    pub fn pwmen0(&self) -> PWMEN0_R {
        PWMEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM channel 1 enable."]
    #[inline(always)]
    pub fn pwmen1(&self) -> PWMEN1_R {
        PWMEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM channel 2 enable."]
    #[inline(always)]
    pub fn pwmen2(&self) -> PWMEN2_R {
        PWMEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM channel 3 enable Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline(always)]
    pub fn pwmen3(&self) -> PWMEN3_R {
        PWMEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM channel 0 enable."]
    #[inline(always)]
    pub fn pwmen0(&mut self) -> PWMEN0_W {
        PWMEN0_W { w: self }
    }
    #[doc = "Bit 1 - PWM channel 1 enable."]
    #[inline(always)]
    pub fn pwmen1(&mut self) -> PWMEN1_W {
        PWMEN1_W { w: self }
    }
    #[doc = "Bit 2 - PWM channel 2 enable."]
    #[inline(always)]
    pub fn pwmen2(&mut self) -> PWMEN2_W {
        PWMEN2_W { w: self }
    }
    #[doc = "Bit 3 - PWM channel 3 enable Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline(always)]
    pub fn pwmen3(&mut self) -> PWMEN3_W {
        PWMEN3_W { w: self }
    }
}
