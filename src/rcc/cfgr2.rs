#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREDIV` reader - PREDIV division factor"]
pub type PREDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PREDIV` writer - PREDIV division factor"]
pub type PREDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADC12PRES` reader - ADC1 and ADC2 prescaler"]
pub type ADC12PRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC12PRES` writer - ADC1 and ADC2 prescaler"]
pub type ADC12PRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ADC34PRES` reader - ADC3 and ADC4 prescaler"]
pub type ADC34PRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC34PRES` writer - ADC3 and ADC4 prescaler"]
pub type ADC34PRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - ADC1 and ADC2 prescaler"]
    #[inline(always)]
    pub fn adc12pres(&self) -> ADC12PRES_R {
        ADC12PRES_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - ADC3 and ADC4 prescaler"]
    #[inline(always)]
    pub fn adc34pres(&self) -> ADC34PRES_R {
        ADC34PRES_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    #[must_use]
    pub fn prediv(&mut self) -> PREDIV_W<0> {
        PREDIV_W::new(self)
    }
    #[doc = "Bits 4:8 - ADC1 and ADC2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adc12pres(&mut self) -> ADC12PRES_W<4> {
        ADC12PRES_W::new(self)
    }
    #[doc = "Bits 9:13 - ADC3 and ADC4 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adc34pres(&mut self) -> ADC34PRES_W<9> {
        ADC34PRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
