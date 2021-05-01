#[doc = "Register `VALUE4` reader"]
pub struct R(crate::R<VALUE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALUE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VALUE4_SPEC>> for R {
    fn from(reader: crate::R<VALUE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VALUE4` writer"]
pub struct W(crate::W<VALUE4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VALUE4_SPEC>;
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
impl core::convert::From<crate::W<VALUE4_SPEC>> for W {
    fn from(writer: crate::W<VALUE4_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value replacement 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value4](index.html) module"]
pub struct VALUE4_SPEC;
impl crate::RegisterSpec for VALUE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [value4::R](R) reader structure"]
impl crate::Readable for VALUE4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [value4::W](W) writer structure"]
impl crate::Writable for VALUE4_SPEC {
    type Writer = W;
}
