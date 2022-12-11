#[doc = "Register `ARR` reader"]
pub struct R(crate::R<ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARR` writer"]
pub struct W(crate::W<ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARR_SPEC>;
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
impl From<crate::W<ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARRL` reader - Low Auto-reload value"]
pub type ARRL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARRL` writer - Low Auto-reload value"]
pub type ARRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARR_SPEC, u16, u16, 16, O>;
#[doc = "Field `ARRH` reader - High Auto-reload value"]
pub type ARRH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARRH` writer - High Auto-reload value"]
pub type ARRH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn arrl(&self) -> ARRL_R {
        ARRL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    pub fn arrh(&self) -> ARRH_R {
        ARRH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    #[must_use]
    pub fn arrl(&mut self) -> ARRL_W<0> {
        ARRL_W::new(self)
    }
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    #[must_use]
    pub fn arrh(&mut self) -> ARRH_W<16> {
        ARRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arr](index.html) module"]
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arr::R](R) reader structure"]
impl crate::Readable for ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arr::W](W) writer structure"]
impl crate::Writable for ARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARR to value 0"]
impl crate::Resettable for ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
