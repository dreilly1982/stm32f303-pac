#[doc = "Register `EMR2` reader"]
pub struct R(crate::R<EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR2` writer"]
pub struct W(crate::W<EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR2_SPEC>;
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
impl From<crate::W<EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR32` reader - Event mask on external/internal line 32"]
pub type MR32_R = crate::BitReader<bool>;
#[doc = "Field `MR32` writer - Event mask on external/internal line 32"]
pub type MR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `MR33` reader - Event mask on external/internal line 33"]
pub type MR33_R = crate::BitReader<bool>;
#[doc = "Field `MR33` writer - Event mask on external/internal line 33"]
pub type MR33_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `MR34` reader - Event mask on external/internal line 34"]
pub type MR34_R = crate::BitReader<bool>;
#[doc = "Field `MR34` writer - Event mask on external/internal line 34"]
pub type MR34_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `MR35` reader - Event mask on external/internal line 35"]
pub type MR35_R = crate::BitReader<bool>;
#[doc = "Field `MR35` writer - Event mask on external/internal line 35"]
pub type MR35_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    #[must_use]
    pub fn mr32(&mut self) -> MR32_W<0> {
        MR32_W::new(self)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    #[must_use]
    pub fn mr33(&mut self) -> MR33_W<1> {
        MR33_W::new(self)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<2> {
        MR34_W::new(self)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<3> {
        MR35_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr2](index.html) module"]
pub struct EMR2_SPEC;
impl crate::RegisterSpec for EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr2::R](R) reader structure"]
impl crate::Readable for EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr2::W](W) writer structure"]
impl crate::Writable for EMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
