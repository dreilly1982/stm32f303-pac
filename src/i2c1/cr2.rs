#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADD0` reader - Slave address bit 0 (master mode)"]
pub type SADD0_R = crate::BitReader<bool>;
#[doc = "Field `SADD0` writer - Slave address bit 0 (master mode)"]
pub type SADD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SADD1` reader - Slave address bit 7:1 (master mode)"]
pub type SADD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SADD1` writer - Slave address bit 7:1 (master mode)"]
pub type SADD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 7, O>;
#[doc = "Field `SADD8` reader - Slave address bit 9:8 (master mode)"]
pub type SADD8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SADD8` writer - Slave address bit 9:8 (master mode)"]
pub type SADD8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode)"]
pub type RD_WRN_R = crate::BitReader<bool>;
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode)"]
pub type RD_WRN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode)"]
pub type ADD10_R = crate::BitReader<bool>;
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode)"]
pub type ADD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode)"]
pub type HEAD10R_R = crate::BitReader<bool>;
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode)"]
pub type HEAD10R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Stop generation (master mode)"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Stop generation (master mode)"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `NACK` reader - NACK generation (slave mode)"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `NACK` writer - NACK generation (slave mode)"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `NBYTES` reader - Number of bytes"]
pub type NBYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBYTES` writer - Number of bytes"]
pub type NBYTES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `RELOAD` reader - NBYTES reload mode"]
pub type RELOAD_R = crate::BitReader<bool>;
#[doc = "Field `RELOAD` writer - NBYTES reload mode"]
pub type RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode)"]
pub type AUTOEND_R = crate::BitReader<bool>;
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode)"]
pub type AUTOEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PECBYTE` reader - Packet error checking byte"]
pub type PECBYTE_R = crate::BitReader<bool>;
#[doc = "Field `PECBYTE` writer - Packet error checking byte"]
pub type PECBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Slave address bit 0 (master mode)"]
    #[inline(always)]
    pub fn sadd0(&self) -> SADD0_R {
        SADD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave address bit 7:1 (master mode)"]
    #[inline(always)]
    pub fn sadd1(&self) -> SADD1_R {
        SADD1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Slave address bit 9:8 (master mode)"]
    #[inline(always)]
    pub fn sadd8(&self) -> SADD8_R {
        SADD8_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave address bit 0 (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sadd0(&mut self) -> SADD0_W<0> {
        SADD0_W::new(self)
    }
    #[doc = "Bits 1:7 - Slave address bit 7:1 (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sadd1(&mut self) -> SADD1_W<1> {
        SADD1_W::new(self)
    }
    #[doc = "Bits 8:9 - Slave address bit 9:8 (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sadd8(&mut self) -> SADD8_W<8> {
        SADD8_W::new(self)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<10> {
        RD_WRN_W::new(self)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<11> {
        ADD10_W::new(self)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<12> {
        HEAD10R_W::new(self)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<13> {
        START_W::new(self)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<14> {
        STOP_W::new(self)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<15> {
        NACK_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<16> {
        NBYTES_W::new(self)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<24> {
        RELOAD_W::new(self)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<25> {
        AUTOEND_W::new(self)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    #[must_use]
    pub fn pecbyte(&mut self) -> PECBYTE_W<26> {
        PECBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
