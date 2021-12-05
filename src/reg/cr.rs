/// Control register.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Cr {
    val: u32,
}

impl Cr {
    /// Reset value.
    pub const DEFAULT: Self = Self { val: 0 };

    pub(crate) const EN: Self = Self::DEFAULT.set_en(true);
    pub(crate) const SOM: Self = Self::EN.set_txsom();
    pub(crate) const EOM: Self = Self::EN.set_txeom();

    /// Returns `true` if the CEC peripheral is enabled.
    #[must_use]
    pub const fn en(&self) -> bool {
        self.val & 0b1 == 0b1
    }

    /// Set the CEC peripheral enable.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32_cec::Cr;
    ///
    /// let cr: Cr = Cr::DEFAULT;
    /// assert_eq!(cr.en(), false);
    ///
    /// let cr: Cr = cr.set_en(true);
    /// assert_eq!(cr.en(), true);
    ///
    /// let cr: Cr = cr.set_en(false);
    /// assert_eq!(cr.en(), false);
    /// ```
    #[must_use = "set_en returns a modified Cr"]
    pub const fn set_en(mut self, en: bool) -> Self {
        if en {
            self.val |= 0b1;
        } else {
            self.val &= !0b1;
        }
        self
    }

    #[must_use]
    pub const fn txsom(&self) -> bool {
        self.val & 0b10 == 0b10
    }

    #[must_use = "set_txsom returns a modified Cr"]
    pub const fn set_txsom(mut self) -> Self {
        self.val |= 0b10;
        self
    }

    #[must_use]
    pub const fn txeom(&self) -> bool {
        self.val & 0b100 == 0b100
    }

    #[must_use = "set_txeom returns a modified Cr"]
    pub const fn set_txeom(mut self) -> Self {
        self.val |= 0b100;
        self
    }
}

impl Default for Cr {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<u32> for Cr {
    #[inline]
    fn from(val: u32) -> Self {
        Self { val }
    }
}

impl From<Cr> for u32 {
    #[inline]
    fn from(cr: Cr) -> Self {
        cr.val
    }
}
