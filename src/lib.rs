//! HDMI CEC driver for STM32 microcontrollers
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_auto_cfg))]

mod reg;

pub use reg::{Cfgr, Cr, Regs};

/// Interrupt flags.
///
/// All interrupts are cleared by writing `1` to the bit field.
pub mod irq {
    /// TX-missing acknowledge error interrupt
    ///
    /// In transmission mode, TXACKE is set by hardware to inform application
    /// that no acknowledge was received.
    /// In case of broadcast transmission, TXACKE informs application that a
    /// negative acknowledge was received.
    /// TXACKE aborts message transmission and clears TXSOM and TXEOM controls.
    pub const TXACK: u32 = 1 << 12;
    /// TX-error
    ///
    /// In transmission mode, TXERR is set by hardware if the CEC initiator
    /// detects low impedance on the CEC line while it is released.
    /// TXERR aborts message transmission and clears TXSOM and TXEOM controls.
    pub const TXERR: u32 = 1 << 11;
    /// TX-buffer underrun
    ///
    /// In transmission mode, TXUDR is set by hardware if application was not in
    /// time to load TXDR before of next byte transmission.
    /// TXUDR aborts message transmission and clears TXSOM and TXEOM control
    /// bits.
    pub const TXUDR: u32 = 1 << 10;
    /// End of transmission
    ///
    /// TXEND is set by hardware to inform application that the last byte of the
    /// CEC message has been successfully transmitted.
    /// TXEND clears the TXSOM and TXEOM control bits.
    pub const TXEND: u32 = 1 << 9;
    /// TX-byte request
    ///
    /// TXBR is set by hardware to inform application that the next transmission
    /// data has to be written to TXDR.
    /// TXBR is set when the 4th bit of currently transmitted byte is sent.
    /// Application must write the next byte to TXDR within six nominal data-bit
    /// periods before transmission underrun error occurs (TXUDR).
    pub const TXBR: u32 = 1 << 8;
    /// Arbitration lost
    ///
    /// ARBLST is set by hardware to inform application that CEC device is
    /// switching to reception due to arbitration lost event following the TXSOM
    /// command.
    /// ARBLST can be due either to a contending CEC device starting earlier or
    /// starting at the same time but with higher HEADER priority.
    /// After ARBLST assertion TXSOM bit keeps pending for next transmission
    /// attempt.
    pub const ARBLST: u32 = 1 << 7;
    /// RX-missing acknowledge.
    ///
    /// In receive mode, RXACKE is set by hardware to inform application that no
    /// acknowledge was seen on the CEC line.
    /// RXACKE applies only for broadcast messages and in listen mode also for
    /// not directly addressed messages (destination address not enabled in OAR).
    /// RXACKE aborts message reception.
    pub const RXACK: u32 = 1 << 6;
    /// RX-long bit period error
    ///
    /// LBPE is set by hardware in case a data-bit waveform is detected with
    /// long bit period error.
    /// LBPE is set at the end of the maximum bit-extension tolerance allowed by
    /// RXTOL, in case falling edge is still longing.
    /// LBPE always stops reception of the CEC message.
    /// LBPE generates an error-bit on the CEC line if LBPEGEN = 1.
    /// In case of broadcast, error-bit is generated even in case of LBPEGEN = 0.
    pub const LBPE: u32 = 1 << 5;
    /// RX-short bit period error
    ///
    /// SBPE is set by hardware in case a data-bit waveform is detected with
    /// short bit period error.
    /// SBPE is set at the time the anticipated falling edge occurs.
    /// SBPE generates an error-bit on the CEC line.
    pub const SBPE: u32 = 1 << 4;
    /// RX-bit rising error
    ///
    /// BRE is set by hardware in case a data-bit waveform is detected with bit
    /// rising error.
    /// BRE is set either at the time the misplaced rising edge occurs, or at
    /// the end of the maximum BRE tolerance allowed by RXTOL, in case rising
    /// edge is still longing.
    /// BRE stops message reception if BRESTP = 1.
    /// BRE generates an error-bit on the CEC line if BREGEN = 1.
    pub const BRE: u32 = 1 << 3;
    /// RX-overrun
    ///
    /// RXOVR is set by hardware if RXBR is not yet cleared at the time a new
    /// byte is received on the CEC line and stored into RXD.
    /// RXOVR assertion stops message reception so that no acknowledge is sent.
    /// In case of broadcast, a negative acknowledge is sent.
    pub const RXOVR: u32 = 1 << 2;
    /// End of reception
    ///
    /// RXEND is set by hardware to inform application that the last byte of a
    /// CEC message is received from the CEC line and stored into the RXD buffer.
    /// RXEND is set at the same time of RXBR.
    pub const RXEND: u32 = 1 << 1;
    /// RX-byte received
    ///
    /// The RXBR bit is set by hardware to inform application that a new byte
    /// has been received from the CEC line and stored into the RXD buffer.
    pub const RXBR: u32 = 1;

    /// Bitmask of all interrupts.
    pub const ALL: u32 = TXACK
        | TXERR
        | TXUDR
        | TXEND
        | TXBR
        | ARBLST
        | RXACK
        | LBPE
        | SBPE
        | BRE
        | RXOVR
        | RXEND
        | RXBR;
}

/// Physical Address
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct PhysAddr {
    addr: u32,
}

#[cfg(feature = "defmt")]
impl defmt::Format for PhysAddr {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "{:X}.{:X}.{:X}.{:X}",
            (self.addr >> 24) as u8,
            (self.addr >> 16) as u8,
            (self.addr >> 8) as u8,
            self.addr as u8,
        )
    }
}

impl core::fmt::Display for PhysAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{:X}.{:X}.{:X}.{:X}",
            (self.addr >> 24) as u8,
            (self.addr >> 16) as u8,
            (self.addr >> 8) as u8,
            self.addr as u8,
        )
    }
}

/// Logical Address
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum LogiAddr {
    /// Television
    Tv = 0x0,
    /// Recording device 1
    RecDev1 = 0x1,
    /// Recording device 2
    RecDev2 = 0x2,
    /// Tuner 1
    Tuner1 = 0x3,
    /// Playback device 1
    PlaybackDev = 0x4,
    // Audio system
    AudioSys = 0x5,
    /// Tuner 2
    Tuner2 = 0x6,
    /// Tuner 3
    Tuner3 = 0x7,
    /// Playback Device 2
    PlaybackDev2 = 0x8,
    /// Recording Device 3
    RecDev3 = 0x9,
    /// Tuner 4
    Tuner4 = 0xA,
    /// Playback Device 3
    PlaybackDev3 = 0xB,
    /// Reserved 1
    Rsvd1 = 0xC,
    /// Reserved 2
    Rsvd2 = 0xD,
    /// Free use
    FreeUse = 0xE,
    /// Unregistered (as source address) or Broadcast (as destination address).
    Broadcast = 0xF,
}

impl From<LogiAddr> for u8 {
    #[inline]
    fn from(addr: LogiAddr) -> Self {
        addr as u8
    }
}

impl TryFrom<u8> for LogiAddr {
    type Error = u8;

    fn try_from(addr: u8) -> Result<Self, Self::Error> {
        match addr {
            x if x == Self::Tv as u8 => Ok(Self::Tv),
            x if x == Self::RecDev1 as u8 => Ok(Self::RecDev1),
            x if x == Self::RecDev2 as u8 => Ok(Self::RecDev2),
            x if x == Self::Tuner1 as u8 => Ok(Self::Tuner1),
            x if x == Self::PlaybackDev as u8 => Ok(Self::PlaybackDev),
            x if x == Self::AudioSys as u8 => Ok(Self::AudioSys),
            x if x == Self::Tuner2 as u8 => Ok(Self::Tuner2),
            x if x == Self::Tuner3 as u8 => Ok(Self::Tuner3),
            x if x == Self::PlaybackDev2 as u8 => Ok(Self::PlaybackDev2),
            x if x == Self::RecDev3 as u8 => Ok(Self::RecDev3),
            x if x == Self::Tuner4 as u8 => Ok(Self::Tuner4),
            x if x == Self::PlaybackDev3 as u8 => Ok(Self::PlaybackDev3),
            x if x == Self::Rsvd1 as u8 => Ok(Self::Rsvd1),
            x if x == Self::Rsvd2 as u8 => Ok(Self::Rsvd2),
            x if x == Self::FreeUse as u8 => Ok(Self::FreeUse),
            x if x == Self::Broadcast as u8 => Ok(Self::Broadcast),
            _ => Err(addr),
        }
    }
}

/// HDMI-CEC driver.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Cec<const BASE: usize> {
    regs: Regs<BASE>,
}

impl<const BASE: usize> Cec<BASE> {
    /// Create a new HDMI CEC driver.
    ///
    /// # Safety
    ///
    /// 1. The HDMI CEC source clock must be enabled.
    /// 2. The HDMI CEC pin must be configured.
    /// 3. The HDMI CEC peripheral should be reset before.
    /// 4. The HDMI CEC registers provided by the PAC should be dropped.
    /// 5. The generic BASE parameter must be correct or bad things will happen.
    ///
    /// # Panics
    ///
    /// Panics if reading CFGR does not return the value written.
    /// This occurs when the HDMI CEC peripheral clocks are not configured
    /// correctly.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use stm32_cec::Cec;
    ///
    /// // device specific setup occurs here
    /// // ...
    ///
    /// // valid address for the STM32H7
    /// let mut cec: Cec<0x40006C00> = unsafe { Cec::<0x40006C00>::new() };
    /// ```
    #[inline]
    pub unsafe fn new() -> Cec<BASE> {
        let mut regs: Regs<BASE> = Regs { _priv: () };
        regs.set_cr(Cr::DEFAULT);
        const MY_CFGR: Cfgr = Cfgr::DEFAULT
            .set_lstn(true)
            .set_oar(0x8)
            .set_lbpegen(true)
            .set_bregen(true)
            .set_brestp(true);
        regs.set_cfgr(MY_CFGR);
        assert_eq!(Regs::<BASE>::cfgr().oar(), 0x8);
        regs.set_ier(irq::ALL);
        regs.set_cr(Cr::EN);

        Cec { regs }
    }

    fn poll_isr() -> u32 {
        // TODO: timeout
        loop {
            let isr: u32 = Regs::<BASE>::isr();
            if isr != 0 {
                Regs::<BASE>::set_isr(isr);
                return isr;
            }
        }
    }

    fn send_byte(&mut self, src: LogiAddr, dst: LogiAddr, data: u8) -> Result<(), u32> {
        self.regs.set_txdr((u8::from(src) << 4) | u8::from(dst));
        self.regs.set_cr(Cr::SOM);

        let isr: u32 = Self::poll_isr();
        if isr & irq::TXBR == irq::TXBR {
            self.regs.set_txdr(data);
            self.regs.set_cr(Cr::EOM);
            let isr: u32 = Self::poll_isr();
            if isr & irq::TXEND == irq::TXEND {
                Ok(())
            } else {
                Err(isr)
            }
        } else {
            Err(isr)
        }
    }

    /// Power off devices.
    ///
    /// # Example
    ///
    /// Turn everything off.
    ///
    /// ```no_run
    /// use stm32_cec::{Cec, LogiAddr};
    ///
    /// let mut cec = unsafe { stm32_cec::Cec::<0x40006C00>::new() };
    /// cec.set_standby(LogiAddr::Broadcast, LogiAddr::Broadcast)?;
    /// # Ok::<(), u32>(())
    /// ```
    pub fn set_standby(&mut self, src: LogiAddr, dst: LogiAddr) -> Result<(), u32> {
        self.send_byte(src, dst, 0x36)
    }

    /// Power on the TV.
    ///
    /// # Example
    ///
    /// Turn everything on.
    ///
    /// ```no_run
    /// use stm32_cec::{Cec, LogiAddr};
    ///
    /// let mut cec = unsafe { stm32_cec::Cec::<0x40006C00>::new() };
    /// cec.set_image_view_on(LogiAddr::Broadcast, LogiAddr::Broadcast)?;
    /// # Ok::<(), u32>(())
    /// ```
    pub fn set_image_view_on(&mut self, src: LogiAddr, dst: LogiAddr) -> Result<(), u32> {
        self.send_byte(src, dst, 0x04)
    }
}
