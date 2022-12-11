#[doc = "Register `OBR` reader"]
pub struct R(crate::R<OBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OPTERR` reader - Option byte error"]
pub type OPTERR_R = crate::BitReader<bool>;
#[doc = "Field `LEVEL1_PROT` reader - Level 1 protection status"]
pub type LEVEL1_PROT_R = crate::BitReader<bool>;
#[doc = "Field `LEVEL2_PROT` reader - Level 2 protection status"]
pub type LEVEL2_PROT_R = crate::BitReader<bool>;
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub type WDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `BOOT1` reader - BOOT1"]
pub type BOOT1_R = crate::BitReader<bool>;
#[doc = "Field `VDDA_MONITOR` reader - VDDA_MONITOR"]
pub type VDDA_MONITOR_R = crate::BitReader<bool>;
#[doc = "Field `SRAM_PARITY_CHECK` reader - SRAM_PARITY_CHECK"]
pub type SRAM_PARITY_CHECK_R = crate::BitReader<bool>;
#[doc = "Field `Data0` reader - Data0"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Data1` reader - Data1"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 1 protection status"]
    #[inline(always)]
    pub fn level1_prot(&self) -> LEVEL1_PROT_R {
        LEVEL1_PROT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level 2 protection status"]
    #[inline(always)]
    pub fn level2_prot(&self) -> LEVEL2_PROT_R {
        LEVEL2_PROT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - BOOT1"]
    #[inline(always)]
    pub fn boot1(&self) -> BOOT1_R {
        BOOT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VDDA_MONITOR"]
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM_PARITY_CHECK"]
    #[inline(always)]
    pub fn sram_parity_check(&self) -> SRAM_PARITY_CHECK_R {
        SRAM_PARITY_CHECK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obr](index.html) module"]
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obr::R](R) reader structure"]
impl crate::Readable for OBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBR to value 0xffff_ff02"]
impl crate::Resettable for OBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff02;
}
