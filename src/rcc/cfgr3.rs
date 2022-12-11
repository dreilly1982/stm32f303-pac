#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART1SW` reader - USART1 clock source selection"]
pub type USART1SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART1SW` writer - USART1 clock source selection"]
pub type USART1SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2C1SW` reader - I2C1 clock source selection"]
pub type I2C1SW_R = crate::BitReader<bool>;
#[doc = "Field `I2C1SW` writer - I2C1 clock source selection"]
pub type I2C1SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, bool, O>;
#[doc = "Field `I2C2SW` reader - I2C2 clock source selection"]
pub type I2C2SW_R = crate::BitReader<bool>;
#[doc = "Field `I2C2SW` writer - I2C2 clock source selection"]
pub type I2C2SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, bool, O>;
#[doc = "Field `I2C3SW` reader - I2C3 clock source selection"]
pub type I2C3SW_R = crate::BitReader<bool>;
#[doc = "Field `I2C3SW` writer - I2C3 clock source selection"]
pub type I2C3SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, bool, O>;
#[doc = "Field `TIM1SW` reader - Timer1 clock source selection"]
pub type TIM1SW_R = crate::BitReader<bool>;
#[doc = "Field `TIM1SW` writer - Timer1 clock source selection"]
pub type TIM1SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, bool, O>;
#[doc = "Field `TIM8SW` reader - Timer8 clock source selection"]
pub type TIM8SW_R = crate::BitReader<bool>;
#[doc = "Field `TIM8SW` writer - Timer8 clock source selection"]
pub type TIM8SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, bool, O>;
#[doc = "Field `USART2SW` reader - USART2 clock source selection"]
pub type USART2SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART2SW` writer - USART2 clock source selection"]
pub type USART2SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `USART3SW` reader - USART3 clock source selection"]
pub type USART3SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART3SW` writer - USART3 clock source selection"]
pub type USART3SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `UART4SW` reader - UART4 clock source selection"]
pub type UART4SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART4SW` writer - UART4 clock source selection"]
pub type UART4SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `UART5SW` reader - UART5 clock source selection"]
pub type UART5SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART5SW` writer - UART5 clock source selection"]
pub type UART5SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&self) -> USART1SW_R {
        USART1SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2C1SW_R {
        I2C1SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sw(&self) -> I2C2SW_R {
        I2C2SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sw(&self) -> I2C3SW_R {
        I2C3SW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer1 clock source selection"]
    #[inline(always)]
    pub fn tim1sw(&self) -> TIM1SW_R {
        TIM1SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer8 clock source selection"]
    #[inline(always)]
    pub fn tim8sw(&self) -> TIM8SW_R {
        TIM8SW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sw(&self) -> USART3SW_R {
        USART3SW_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sw(&self) -> UART4SW_R {
        UART4SW_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sw(&self) -> UART5SW_R {
        UART5SW_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sw(&mut self) -> USART1SW_W<0> {
        USART1SW_W::new(self)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sw(&mut self) -> I2C1SW_W<4> {
        I2C1SW_W::new(self)
    }
    #[doc = "Bit 5 - I2C2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sw(&mut self) -> I2C2SW_W<5> {
        I2C2SW_W::new(self)
    }
    #[doc = "Bit 6 - I2C3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sw(&mut self) -> I2C3SW_W<6> {
        I2C3SW_W::new(self)
    }
    #[doc = "Bit 8 - Timer1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sw(&mut self) -> TIM1SW_W<8> {
        TIM1SW_W::new(self)
    }
    #[doc = "Bit 9 - Timer8 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim8sw(&mut self) -> TIM8SW_W<9> {
        TIM8SW_W::new(self)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sw(&mut self) -> USART2SW_W<16> {
        USART2SW_W::new(self)
    }
    #[doc = "Bits 18:19 - USART3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sw(&mut self) -> USART3SW_W<18> {
        USART3SW_W::new(self)
    }
    #[doc = "Bits 20:21 - UART4 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart4sw(&mut self) -> UART4SW_W<20> {
        UART4SW_W::new(self)
    }
    #[doc = "Bits 22:23 - UART5 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart5sw(&mut self) -> UART5SW_W<22> {
        UART5SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
