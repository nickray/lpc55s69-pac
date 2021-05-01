#[doc = "Register `SDIO_CFG` reader"]
pub struct R(crate::R<SDIO_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SDIO_CFG_SPEC>> for R {
    fn from(reader: crate::R<SDIO_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_CFG` writer"]
pub struct W(crate::W<SDIO_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_CFG_SPEC>;
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
impl core::convert::From<crate::W<SDIO_CFG_SPEC>> for W {
    fn from(writer: crate::W<SDIO_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELD` reader - ."]
pub struct FIELD_R(crate::FieldReader<u32, u32>);
impl FIELD_R {
    pub(crate) fn new(bits: u32) -> Self {
        FIELD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIELD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIELD` writer - ."]
pub struct FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W {
        FIELD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_cfg](index.html) module"]
pub struct SDIO_CFG_SPEC;
impl crate::RegisterSpec for SDIO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_cfg::R](R) reader structure"]
impl crate::Readable for SDIO_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_cfg::W](W) writer structure"]
impl crate::Writable for SDIO_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_CFG to value 0"]
impl crate::Resettable for SDIO_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
