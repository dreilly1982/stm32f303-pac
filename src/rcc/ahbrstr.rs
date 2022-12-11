#[doc = "Register `AHBRSTR` reader"]
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBRSTR` writer"]
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
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
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCRST` reader - FMC reset"]
pub type FMCRST_R = crate::BitReader<bool>;
#[doc = "Field `FMCRST` writer - FMC reset"]
pub type FMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `IOPHRST` reader - I/O port H reset"]
pub type IOPHRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPHRST` writer - I/O port H reset"]
pub type IOPHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub type IOPARST_R = crate::BitReader<bool>;
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub type IOPARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub type IOPBRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub type IOPBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `IOPCRST` reader - I/O port C reset"]
pub type IOPCRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPCRST` writer - I/O port C reset"]
pub type IOPCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub type IOPDRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub type IOPDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `IOPERST` reader - I/O port E reset"]
pub type IOPERST_R = crate::BitReader<bool>;
#[doc = "Field `IOPERST` writer - I/O port E reset"]
pub type IOPERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `IOPFRST` reader - I/O port F reset"]
pub type IOPFRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPFRST` writer - I/O port F reset"]
pub type IOPFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `IOPGRST` reader - Touch sensing controller reset"]
pub type IOPGRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPGRST` writer - Touch sensing controller reset"]
pub type IOPGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `TSCRST` reader - Touch sensing controller reset"]
pub type TSCRST_R = crate::BitReader<bool>;
#[doc = "Field `TSCRST` writer - Touch sensing controller reset"]
pub type TSCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `ADC12RST` reader - ADC1 and ADC2 reset"]
pub type ADC12RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC12RST` writer - ADC1 and ADC2 reset"]
pub type ADC12RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `ADC34RST` reader - ADC3 and ADC4 reset"]
pub type ADC34RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC34RST` writer - ADC3 and ADC4 reset"]
pub type ADC34RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - FMC reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - I/O port H reset"]
    #[inline(always)]
    pub fn iophrst(&self) -> IOPHRST_R {
        IOPHRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I/O port E reset"]
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn iopgrst(&self) -> IOPGRST_R {
        IOPGRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 reset"]
    #[inline(always)]
    pub fn adc34rst(&self) -> ADC34RST_R {
        ADC34RST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - FMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<5> {
        FMCRST_W::new(self)
    }
    #[doc = "Bit 16 - I/O port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn iophrst(&mut self) -> IOPHRST_W<16> {
        IOPHRST_W::new(self)
    }
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IOPARST_W<17> {
        IOPARST_W::new(self)
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IOPBRST_W<18> {
        IOPBRST_W::new(self)
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IOPCRST_W<19> {
        IOPCRST_W::new(self)
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IOPDRST_W<20> {
        IOPDRST_W::new(self)
    }
    #[doc = "Bit 21 - I/O port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioperst(&mut self) -> IOPERST_W<21> {
        IOPERST_W::new(self)
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IOPFRST_W<22> {
        IOPFRST_W::new(self)
    }
    #[doc = "Bit 23 - Touch sensing controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopgrst(&mut self) -> IOPGRST_W<23> {
        IOPGRST_W::new(self)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<24> {
        TSCRST_W::new(self)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<28> {
        ADC12RST_W::new(self)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc34rst(&mut self) -> ADC34RST_W<29> {
        ADC34RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrstr](index.html) module"]
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbrstr::R](R) reader structure"]
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbrstr::W](W) writer structure"]
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
