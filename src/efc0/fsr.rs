#[doc = "Reader of register FSR"]
pub type R = crate::R<u32, super::FSR>;
#[doc = "Reader of field `FRDY`"]
pub type FRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCMDE`"]
pub type FCMDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLOCKE`"]
pub type FLOCKE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLERR`"]
pub type FLERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Flash Ready Status (cleared when Flash is busy)"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Command Error Status (cleared on read or by writing EEFC_FCR)"]
    #[inline(always)]
    pub fn fcmde(&self) -> FCMDE_R {
        FCMDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash Lock Error Status (cleared on read)"]
    #[inline(always)]
    pub fn flocke(&self) -> FLOCKE_R {
        FLOCKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash Error Status (cleared when a programming operation starts)"]
    #[inline(always)]
    pub fn flerr(&self) -> FLERR_R {
        FLERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
