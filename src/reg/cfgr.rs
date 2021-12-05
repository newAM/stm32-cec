/// Signal Free time
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Sft {
    /// Determined by transmission history.
    ///
    /// * 2.5 data-bit periods if CEC is the last bus initiator with
    ///   unsuccessful transmission
    ///   (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1)
    /// * 4 data-bit periods if CEC is the new bus initiator
    /// * 6 data-bit periods if CEC is the last bus initiator with successful
    ///   transmission (TXEOM = 1)
    History = 0x0,
    /// 0.5 nominal data bit periods
    Nom0pt5 = 0x1,
    /// 1.5 nominal data bit periods
    Nom1pt5 = 0x2,
    /// 2.5 nominal data bit periods
    Nom2pt5 = 0x3,
    /// 3.5 nominal data bit periods
    Nom3pt5 = 0x4,
    /// 4.5 nominal data bit periods
    Nom4pt5 = 0x5,
    /// 5.5 nominal data bit periods
    Nom5pt5 = 0x6,
    /// 6.5 nominal data bit periods
    Nom6pt5 = 0x7,
}

/// Configuration register.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Cfgr {
    val: u32,
}

impl Cfgr {
    /// Reset value.
    pub const DEFAULT: Self = Self { val: 0 };

    #[must_use = "set_lstn returns a modified Cfgr"]
    pub const fn set_lstn(mut self, lstn: bool) -> Self {
        if lstn {
            self.val |= 1 << 31;
        } else {
            self.val &= !(1 << 31);
        }
        self
    }

    #[must_use]
    pub const fn oar(&self) -> u16 {
        ((self.val & 0x7FFF_0000) >> 16) as u16
    }

    #[must_use = "set_oar returns a modified Cfgr"]
    pub const fn set_oar(mut self, oar: u16) -> Self {
        self.val &= 0x8000_FFFF;
        self.val |= ((oar as u32) & 0x7FFF) << 16;
        self
    }

    #[must_use = "set_sftop returns a modified Cfgr"]
    pub const fn set_sftop(mut self, sftop: bool) -> Self {
        if sftop {
            self.val |= 1 << 8;
        } else {
            self.val &= !(1 << 8);
        }
        self
    }

    #[must_use = "set_brdnogen returns a modified Cfgr"]
    pub const fn set_brdnogen(mut self, brdnogen: bool) -> Self {
        if brdnogen {
            self.val |= 1 << 7;
        } else {
            self.val &= !(1 << 7);
        }
        self
    }

    #[must_use = "set_lbpegen returns a modified Cfgr"]
    pub const fn set_lbpegen(mut self, lbpegen: bool) -> Self {
        if lbpegen {
            self.val |= 1 << 6;
        } else {
            self.val &= !(1 << 6);
        }
        self
    }

    #[must_use = "set_bregen returns a modified Cfgr"]
    pub const fn set_bregen(mut self, bregen: bool) -> Self {
        if bregen {
            self.val |= 1 << 5;
        } else {
            self.val &= !(1 << 5);
        }
        self
    }

    #[must_use = "set_brestp returns a modified Cfgr"]
    pub const fn set_brestp(mut self, brestp: bool) -> Self {
        if brestp {
            self.val |= 1 << 4;
        } else {
            self.val &= !(1 << 4);
        }
        self
    }

    #[must_use = "set_rxtol returns a modified Cfgr"]
    pub const fn set_rxtol(mut self, rxtol: bool) -> Self {
        if rxtol {
            self.val |= 1 << 3;
        } else {
            self.val &= !(1 << 3);
        }
        self
    }

    #[must_use = "set_sft returns a modified Cfgr"]
    pub const fn set_sft(mut self, sft: Sft) -> Self {
        self.val &= 0xFFFF_FFF8;
        self.val |= sft as u32;
        self
    }
}

impl Default for Cfgr {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<u32> for Cfgr {
    #[inline]
    fn from(val: u32) -> Self {
        Self { val }
    }
}

impl From<Cfgr> for u32 {
    #[inline]
    fn from(cfgr: Cfgr) -> Self {
        cfgr.val
    }
}
