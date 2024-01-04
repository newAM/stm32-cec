mod cfgr;
mod cr;

pub use cfgr::Cfgr;
pub use cr::Cr;

/// HDMI-CEC register access.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Regs<const BASE: usize> {
    pub(crate) _priv: (),
}

impl<const BASE: usize> Regs<BASE> {
    // register addresses
    const CR: *const u32 = BASE as *const u32;
    const CFGR: *const u32 = (BASE + 0x04) as *const u32;
    const TXDR: *mut u32 = (BASE + 0x08) as *mut u32;
    const RXDR: *const u32 = (BASE + 0x0C) as *const u32;
    const ISR: *const u32 = (BASE + 0x10) as *const u32;
    const IER: *const u32 = (BASE + 0x14) as *const u32;

    /// Read the command register.
    #[inline]
    pub fn cr() -> Cr {
        unsafe { Self::CR.read_volatile() }.into()
    }

    /// Write the command register.
    #[inline]
    pub fn set_cr(&mut self, cr: Cr) {
        unsafe { (Self::CR as *mut u32).write_volatile(u32::from(cr) & 0b111) }
    }

    /// Read the configuration register.
    #[inline]
    pub fn cfgr() -> Cfgr {
        unsafe { Self::CFGR.read_volatile() }.into()
    }

    #[inline]
    pub fn set_cfgr(&mut self, cfgr: Cfgr) {
        unsafe { (Self::CFGR as *mut u32).write_volatile(u32::from(cfgr) & 0xFFFF_01FF) }
    }

    /// Write the TX data register.
    #[inline]
    pub fn set_txdr(&mut self, data: u8) {
        unsafe { Self::TXDR.write_volatile(data as u32) }
    }

    /// Read the RX data register.
    #[inline]
    pub fn rxdr(&mut self) -> u8 {
        unsafe { Self::RXDR.read_volatile() as u8 }
    }

    /// Read the interrupt status register.
    #[inline]
    pub fn isr() -> u32 {
        unsafe { Self::ISR.read_volatile() }
    }

    #[inline]
    pub fn set_isr(isr: u32) {
        unsafe { (Self::ISR as *mut u32).write_volatile(isr & super::irq::ALL) }
    }

    /// Read the interrupt enable register.
    #[inline]
    pub fn ier() -> u32 {
        unsafe { Self::IER.read_volatile() }
    }

    /// Write the interrupt enable register.
    #[inline]
    pub fn set_ier(&mut self, ier: u32) {
        unsafe { (Self::IER as *mut u32).write_volatile(ier & super::irq::ALL) }
    }
}
