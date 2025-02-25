//! Direct Memory Access

use crate::{
    dma::pdma::private::*,
    system::{Peripheral, PeripheralClockControl},
};

macro_rules! ImplSpiChannel {
    ($num: literal) => {
        paste::paste! {
            pub struct [<Spi $num DmaChannel>] {}

            impl RegisterAccess for [<Spi $num DmaChannel>] {
                fn init_channel() {
                    // (only) on ESP32 we need to configure DPORT for the SPI DMA channels
                    #[cfg(esp32)]
                    {
                        let dport = unsafe { &*crate::pac::DPORT::PTR };

                        match $num {
                            2 => {
                                dport
                                .spi_dma_chan_sel
                                .modify(|_, w| w.spi2_dma_chan_sel().variant(1));
                            },
                            3 => {
                                dport
                                .spi_dma_chan_sel
                                .modify(|_, w| w.spi3_dma_chan_sel().variant(2));
                            },
                            _ => panic!("Only SPI2 and SPI3 supported"),
                        }
                    }
                }

                fn set_out_burstmode(burst_mode: bool) {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_conf
                        .modify(|_, w| w.outdscr_burst_en().bit(burst_mode));
                }

                fn set_out_priority(_priority: DmaPriority) {}

                fn clear_out_interrupts() {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_int_clr.write(|w| {
                        w.out_done_int_clr()
                            .set_bit()
                            .out_eof_int_clr()
                            .set_bit()
                            .out_total_eof_int_clr()
                            .set_bit()
                            .outlink_dscr_error_int_clr()
                            .set_bit()
                    });
                }

                fn reset_out() {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_conf.modify(|_, w| w.out_rst().set_bit());
                    spi.dma_conf.modify(|_, w| w.out_rst().clear_bit());
                }

                fn set_out_descriptors(address: u32) {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_out_link
                        .modify(|_, w| unsafe { w.outlink_addr().bits(address) });
                }

                fn has_out_descriptor_error() -> bool {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_int_raw.read().outlink_dscr_error_int_raw().bit()
                }

                fn set_out_peripheral(_peripheral: u8) {
                    // no-op
                }

                fn start_out() {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_out_link.modify(|_, w| w.outlink_start().set_bit());
                }

                fn is_out_done() -> bool {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_int_raw.read().out_done_int_raw().bit()
                }

                fn last_out_dscr_address() -> usize {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.out_eof_des_addr.read().dma_out_eof_des_addr().bits() as usize
                }

                fn is_out_eof_interrupt_set() -> bool {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_int_raw.read().out_eof_int_raw().bit()
                }

                fn reset_out_eof_interrupt() {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_int_clr.write(|w| {
                        w.out_eof_int_clr()
                            .set_bit()
                    });
                }

                fn set_in_burstmode(burst_mode: bool) {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_conf
                        .modify(|_, w| w.indscr_burst_en().bit(burst_mode));
                }

                fn set_in_priority(_priority: DmaPriority) {}

                fn clear_in_interrupts() {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_int_clr.write(|w| {
                        w.in_done_int_clr()
                            .set_bit()
                            .in_err_eof_int_clr()
                            .set_bit()
                            .in_suc_eof_int_clr()
                            .set_bit()
                            .inlink_dscr_error_int_clr()
                            .set_bit()
                    });
                }

                fn reset_in() {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_conf.modify(|_, w| w.in_rst().set_bit());
                    spi.dma_conf.modify(|_, w| w.in_rst().clear_bit());
                }

                fn set_in_descriptors(address: u32) {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_in_link
                        .modify(|_, w| unsafe { w.inlink_addr().bits(address) });
                }

                fn has_in_descriptor_error() -> bool {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_int_raw.read().inlink_dscr_error_int_raw().bit()
                }

                fn set_in_peripheral(_peripheral: u8) {
                    // no-op
                }

                fn start_in() {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_in_link.modify(|_, w| w.inlink_start().set_bit());
                }

                fn is_in_done() -> bool {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.dma_int_raw.read().in_done_int_raw().bit()
                }

                fn last_in_dscr_address() -> usize {
                    let spi = unsafe { &*crate::pac::[<SPI $num>]::PTR };
                    spi.inlink_dscr_bf0.read().dma_inlink_dscr_bf0().bits() as usize
                }
            }

            pub struct [<Spi $num DmaChannelTxImpl>] {}

            impl<'a> TxChannel<[<Spi $num DmaChannel>]> for [<Spi $num DmaChannelTxImpl>] {}

            pub struct [<Spi $num DmaChannelRxImpl>] {}

            impl<'a> RxChannel<[<Spi $num DmaChannel>]> for [<Spi $num DmaChannelRxImpl>] {}

            pub struct [<Spi $num DmaChannelCreator>] {}

            impl [<Spi $num DmaChannelCreator>] {
                pub fn configure<'a>(
                    self,
                    burst_mode: bool,
                    tx_descriptors: &'a mut [u32],
                    rx_descriptors: &'a mut [u32],
                    priority: DmaPriority,
                ) -> Channel<
                    ChannelTx<'a,[<Spi $num DmaChannelTxImpl>], [<Spi $num DmaChannel>]>,
                    ChannelRx<'a,[<Spi $num DmaChannelRxImpl>], [<Spi $num DmaChannel>]>,
                    [<Spi $num DmaSuitablePeripheral>],
                > {
                    let mut tx_impl = [<Spi $num DmaChannelTxImpl>] {};
                    tx_impl.init(burst_mode, priority);

                    let tx_channel = ChannelTx {
                        descriptors: tx_descriptors,
                        burst_mode,
                        tx_impl: tx_impl,
                        write_offset: 0,
                        write_descr_ptr: core::ptr::null(),
                        available: 0,
                        last_seen_handled_descriptor_ptr: core::ptr::null(),
                        buffer_start: core::ptr::null(),
                        buffer_len: 0,
                        _phantom: PhantomData::default(),
                    };

                    let mut rx_impl = [<Spi $num DmaChannelRxImpl>] {};
                    rx_impl.init(burst_mode, priority);

                    let rx_channel = ChannelRx {
                        descriptors: rx_descriptors,
                        burst_mode,
                        rx_impl: rx_impl,
                        read_offset: 0,
                        read_descr_ptr: core::ptr::null(),
                        available: 0,
                        last_seen_handled_descriptor_ptr: core::ptr::null(),
                        buffer_start: core::ptr::null(),
                        buffer_len: 0,
                        _phantom: PhantomData::default(),
                    };

                    Channel {
                        tx: tx_channel,
                        rx: rx_channel,
                        _phantom: PhantomData::default(),
                    }
                }
            }
        }
    };
}

macro_rules! ImplI2sChannel {
    ($num: literal, $peripheral: literal) => {
        paste::paste! {
            pub struct [<I2s $num DmaChannel>] {}

            impl RegisterAccess for [<I2s $num DmaChannel>] {
                fn init_channel() {
                    // nothing to do
                }

                fn set_out_burstmode(burst_mode: bool) {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.lc_conf
                        .modify(|_, w| w.outdscr_burst_en().bit(burst_mode));
                }

                fn set_out_priority(_priority: DmaPriority) {}

                fn clear_out_interrupts() {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.int_clr.write(|w| {
                        w.out_done_int_clr()
                            .set_bit()
                            .out_eof_int_clr()
                            .set_bit()
                            .out_total_eof_int_clr()
                            .set_bit()
                            .out_dscr_err_int_clr()
                            .set_bit()
                    });
                }

                fn reset_out() {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.lc_conf.modify(|_, w| w.out_rst().set_bit());
                    reg_block.lc_conf.modify(|_, w| w.out_rst().clear_bit());
                }

                fn set_out_descriptors(address: u32) {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.out_link
                        .modify(|_, w| unsafe { w.outlink_addr().bits(address) });
                }

                fn has_out_descriptor_error() -> bool {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.int_raw.read().out_dscr_err_int_raw().bit()
                }

                fn set_out_peripheral(_peripheral: u8) {
                    // no-op
                }

                fn start_out() {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.out_link.modify(|_, w| w.outlink_start().set_bit());
                }

                fn is_out_done() -> bool {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.int_raw.read().out_done_int_raw().bit()
                }

                fn last_out_dscr_address() -> usize {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.out_eof_des_addr.read().out_eof_des_addr().bits() as usize
                }

                fn is_out_eof_interrupt_set() -> bool {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.int_raw.read().out_eof_int_raw().bit()
                }

                fn reset_out_eof_interrupt() {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.int_clr.write(|w| {
                        w.out_eof_int_clr()
                            .set_bit()
                    });
                }

                fn set_in_burstmode(burst_mode: bool) {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.lc_conf
                        .modify(|_, w| w.indscr_burst_en().bit(burst_mode));
                }

                fn set_in_priority(_priority: DmaPriority) {}

                fn clear_in_interrupts() {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.int_clr.write(|w| {
                        w.in_done_int_clr()
                            .set_bit()
                            .in_err_eof_int_clr()
                            .set_bit()
                            .in_suc_eof_int_clr()
                            .set_bit()
                            .in_dscr_err_int_clr()
                            .set_bit()
                    });
                }

                fn reset_in() {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.lc_conf.modify(|_, w| w.in_rst().set_bit());
                    reg_block.lc_conf.modify(|_, w| w.in_rst().clear_bit());
                }

                fn set_in_descriptors(address: u32) {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.in_link
                        .modify(|_, w| unsafe { w.inlink_addr().bits(address) });
                }

                fn has_in_descriptor_error() -> bool {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.int_raw.read().in_dscr_err_int_raw().bit()
                }

                fn set_in_peripheral(_peripheral: u8) {
                    // no-op
                }

                fn start_in() {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.in_link.modify(|_, w| w.inlink_start().set_bit());
                }

                fn is_in_done() -> bool {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.int_raw.read().in_done_int_raw().bit()
                }

                fn last_in_dscr_address() -> usize {
                    let reg_block = unsafe { &*crate::pac::[<$peripheral>]::PTR };
                    reg_block.inlink_dscr_bf0.read().inlink_dscr_bf0().bits() as usize
                }
            }

            pub struct [<I2s $num DmaChannelTxImpl>] {}

            impl<'a> TxChannel<[<I2s $num DmaChannel>]> for [<I2s $num DmaChannelTxImpl>] {}

            pub struct [<I2s $num DmaChannelRxImpl>] {}

            impl<'a> RxChannel<[<I2s $num DmaChannel>]> for [<I2s $num DmaChannelRxImpl>] {}

            pub struct [<I2s $num DmaChannelCreator>] {}

            impl [<I2s $num DmaChannelCreator>] {
                pub fn configure<'a>(
                    self,
                    burst_mode: bool,
                    tx_descriptors: &'a mut [u32],
                    rx_descriptors: &'a mut [u32],
                    priority: DmaPriority,
                ) -> Channel<
                    ChannelTx<'a,[<I2s $num DmaChannelTxImpl>], [<I2s $num DmaChannel>]>,
                    ChannelRx<'a,[<I2s $num DmaChannelRxImpl>], [<I2s $num DmaChannel>]>,
                    [<I2s $num DmaSuitablePeripheral>],
                > {
                    let mut tx_impl = [<I2s $num DmaChannelTxImpl>] {};
                    tx_impl.init(burst_mode, priority);

                    let tx_channel = ChannelTx {
                        descriptors: tx_descriptors,
                        burst_mode,
                        tx_impl: tx_impl,
                        write_offset: 0,
                        write_descr_ptr: core::ptr::null(),
                        available: 0,
                        last_seen_handled_descriptor_ptr: core::ptr::null(),
                        buffer_start: core::ptr::null(),
                        buffer_len: 0,
                        _phantom: PhantomData::default(),
                    };

                    let mut rx_impl = [<I2s $num DmaChannelRxImpl>] {};
                    rx_impl.init(burst_mode, priority);

                    let rx_channel = ChannelRx {
                        descriptors: rx_descriptors,
                        burst_mode,
                        rx_impl: rx_impl,
                        read_offset: 0,
                        read_descr_ptr: core::ptr::null(),
                        available: 0,
                        last_seen_handled_descriptor_ptr: core::ptr::null(),
                        buffer_start: core::ptr::null(),
                        buffer_len: 0,
                        _phantom: PhantomData::default(),
                    };

                    Channel {
                        tx: tx_channel,
                        rx: rx_channel,
                        _phantom: PhantomData::default(),
                    }
                }
            }
        }
    };
}

/// Crate private implementation details
pub(crate) mod private {
    use crate::dma::{private::*, *};

    pub struct Spi2DmaSuitablePeripheral {}
    impl PeripheralMarker for Spi2DmaSuitablePeripheral {}
    impl SpiPeripheral for Spi2DmaSuitablePeripheral {}
    impl Spi2Peripheral for Spi2DmaSuitablePeripheral {}

    pub struct Spi3DmaSuitablePeripheral {}
    impl PeripheralMarker for Spi3DmaSuitablePeripheral {}
    impl SpiPeripheral for Spi3DmaSuitablePeripheral {}
    impl Spi3Peripheral for Spi3DmaSuitablePeripheral {}

    ImplSpiChannel!(2);
    ImplSpiChannel!(3);

    pub struct I2s0DmaSuitablePeripheral {}
    impl PeripheralMarker for I2s0DmaSuitablePeripheral {}
    impl I2sPeripheral for I2s0DmaSuitablePeripheral {}
    impl I2s0Peripheral for I2s0DmaSuitablePeripheral {}

    #[cfg(esp32)]
    ImplI2sChannel!(0, "I2S0");

    #[cfg(esp32s2)]
    ImplI2sChannel!(0, "I2S");

    pub struct I2s1DmaSuitablePeripheral {}
    impl PeripheralMarker for I2s1DmaSuitablePeripheral {}
    impl I2sPeripheral for I2s1DmaSuitablePeripheral {}
    impl I2s1Peripheral for I2s1DmaSuitablePeripheral {}

    #[cfg(esp32)]
    ImplI2sChannel!(1, "I2S1");
}

/// DMA Peripheral
///
/// This offers the available DMA channels.
pub struct Dma {
    _inner: crate::system::Dma,
    pub spi2channel: Spi2DmaChannelCreator,
    pub spi3channel: Spi3DmaChannelCreator,
    pub i2s0channel: I2s0DmaChannelCreator,
    #[cfg(esp32)]
    pub i2s1channel: I2s1DmaChannelCreator,
}

impl Dma {
    /// Create a DMA instance.
    pub fn new(
        dma: crate::system::Dma,
        peripheral_clock_control: &mut PeripheralClockControl,
    ) -> Dma {
        peripheral_clock_control.enable(Peripheral::Dma);

        Dma {
            _inner: dma,
            spi2channel: Spi2DmaChannelCreator {},
            spi3channel: Spi3DmaChannelCreator {},
            i2s0channel: I2s0DmaChannelCreator {},
            #[cfg(esp32)]
            i2s1channel: I2s1DmaChannelCreator {},
        }
    }
}
