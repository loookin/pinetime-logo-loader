/* automatically generated by rust-bindgen */

use
super::*;

pub const HAL_ADC_MODULE_ENABLED: u32 = 1;
pub const HAL_SPI_TYPE_MASTER: u32 = 0;
pub const HAL_SPI_TYPE_SLAVE: u32 = 1;
pub const HAL_SPI_MODE0: u32 = 0;
pub const HAL_SPI_MODE1: u32 = 1;
pub const HAL_SPI_MODE2: u32 = 2;
pub const HAL_SPI_MODE3: u32 = 3;
pub const HAL_SPI_MSB_FIRST: u32 = 0;
pub const HAL_SPI_LSB_FIRST: u32 = 1;
pub const HAL_SPI_WORD_SIZE_8BIT: u32 = 0;
pub const HAL_SPI_WORD_SIZE_9BIT: u32 = 1;
pub type __uint8_t = ::cty::c_uchar;
pub type __uint16_t = ::cty::c_ushort;
pub type __uint32_t = ::cty::c_ulong;
pub type hal_spi_txrx_cb =
    ::core::option::Option<unsafe extern "C" fn(arg: *mut ::cty::c_void, len: ::cty::c_int)>;
#[doc = " SPI controller hardware settings"]
#[repr(C)]
#[derive(Default)]
pub struct hal_spi_hw_settings {
    pub pin_sck: ::cty::c_int,
    pub pin_mosi: ::cty::c_int,
    pub pin_miso: ::cty::c_int,
    pub pin_ss: ::cty::c_int,
}
#[doc = " since one spi device can control multiple devices, some configuration"]
#[doc = " can be changed on the fly from the hal"]
#[repr(C)]
#[derive(Default)]
pub struct hal_spi_settings {
    #[doc = " Data mode of SPI driver, defined by HAL_SPI_MODEn"]
    pub data_mode: u8,
    #[doc = " Data order, either HAL_SPI_MSB_FIRST or HAL_SPI_LSB_FIRST"]
    pub data_order: u8,
    #[doc = " The word size of the SPI transaction, either 8-bit or 9-bit"]
    pub word_size: u8,
    #[doc = " Baudrate in kHz"]
    pub baudrate: u32,
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Initialize the SPI, given by spi_num."]
    #[doc = ""]
    #[doc = " - __`spi_num`__: The number of the SPI to initialize"]
    #[doc = " - __`cfg`__: HW/MCU specific configuration,"]
    #[doc = "            passed to the underlying implementation, providing extra"]
    #[doc = "            configuration."]
    #[doc = " - __`spi_type`__: SPI type (master or slave)"]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    pub fn hal_spi_init(
        spi_num: ::cty::c_int,
        cfg: *mut ::cty::c_void,
        spi_type: u8,
    ) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Initialize SPI controller"]
    #[doc = ""]
    #[doc = " This initializes SPI controller hardware before 1st use. Shall be called"]
    #[doc = " only once."]
    #[doc = ""]
    #[doc = " - __`spi_num`__:  Number of SPI controller"]
    #[doc = " - __`cfg`__:      Configuration"]
    #[doc = ""]
    #[doc = " Return: 0 on success, non-zero error code on failure"]
    pub fn hal_spi_init_hw(
        spi_num: u8,
        spi_type: u8,
        cfg: *const hal_spi_hw_settings,
    ) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Configure the spi. Must be called after the spi is initialized (after"]
    #[doc = " hal_spi_init is called) and when the spi is disabled (user must call"]
    #[doc = " hal_spi_disable if the spi has been enabled through hal_spi_enable prior"]
    #[doc = " to calling this function). Can also be used to reconfigure an initialized"]
    #[doc = " SPI (assuming it is disabled as described previously)."]
    #[doc = ""]
    #[doc = " - __`spi_num`__: The number of the SPI to configure."]
    #[doc = " - __`psettings`__: The settings to configure this SPI with"]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    pub fn hal_spi_config(spi_num: ::cty::c_int, psettings: *mut hal_spi_settings) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Sets the txrx callback (executed at interrupt context) when the"]
    #[doc = " buffer is transferred by the master or the slave using the non-blocking API."]
    #[doc = " Cannot be called when the spi is enabled. This callback will also be called"]
    #[doc = " when chip select is de-asserted on the slave."]
    #[doc = ""]
    #[doc = " NOTE: This callback is only used for the non-blocking interface and must"]
    #[doc = " be called prior to using the non-blocking API."]
    #[doc = ""]
    #[doc = " - __`spi_num`__:   SPI interface on which to set callback"]
    #[doc = " - __`txrx`__:      Callback function"]
    #[doc = " - __`arg`__:       Argument to be passed to callback function"]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    pub fn hal_spi_set_txrx_cb(
        spi_num: ::cty::c_int,
        txrx_cb: hal_spi_txrx_cb,
        arg: *mut ::cty::c_void,
    ) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Enables the SPI. This does not start a transmit or receive operation;"]
    #[doc = " it is used for power mgmt. Cannot be called when a SPI transfer is in"]
    #[doc = " progress."]
    #[doc = ""]
    #[doc = " @param spi_num"]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    pub fn hal_spi_enable(spi_num: ::cty::c_int) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Disables the SPI. Used for power mgmt. It will halt any current SPI transfers"]
    #[doc = " in progress."]
    #[doc = ""]
    #[doc = " @param spi_num"]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    pub fn hal_spi_disable(spi_num: ::cty::c_int) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Blocking call to send a value on the SPI. Returns the value received from the"]
    #[doc = " SPI slave."]
    #[doc = ""]
    #[doc = " MASTER: Sends the value and returns the received value from the slave."]
    #[doc = " SLAVE: Invalid API. Returns 0xFFFF"]
    #[doc = ""]
    #[doc = " - __`spi_num`__:   Spi interface to use"]
    #[doc = " - __`val`__:       Value to send"]
    #[doc = ""]
    #[doc = " Return: uint16_t Value received on SPI interface from slave. Returns 0xFFFF"]
    #[doc = " if called when the SPI is configured to be a slave"]
    pub fn hal_spi_tx_val(spi_num: ::cty::c_int, val: u16) -> u16;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Blocking interface to send a buffer and store the received values from the"]
    #[doc = " slave. The transmit and receive buffers are either arrays of 8-bit (uint8_t)"]
    #[doc = " values or 16-bit values depending on whether the spi is configured for 8 bit"]
    #[doc = " data or more than 8 bits per value. The 'cnt' parameter is the number of"]
    #[doc = " 8-bit or 16-bit values. Thus, if 'cnt' is 10, txbuf/rxbuf would point to an"]
    #[doc = " array of size 10 (in bytes) if the SPI is using 8-bit data; otherwise"]
    #[doc = " txbuf/rxbuf would point to an array of size 20 bytes (ten, uint16_t values)."]
    #[doc = ""]
    #[doc = " NOTE: these buffers are in the native endian-ness of the platform."]
    #[doc = ""]
    #[doc = "     MASTER: master sends all the values in the buffer and stores the"]
    #[doc = "             stores the values in the receive buffer if rxbuf is not NULL."]
    #[doc = "             The txbuf parameter cannot be NULL."]
    #[doc = "     SLAVE: cannot be called for a slave; returns -1"]
    #[doc = ""]
    #[doc = " - __`spi_num`__:   SPI interface to use"]
    #[doc = " - __`txbuf`__:     Pointer to buffer where values to transmit are stored."]
    #[doc = " - __`rxbuf`__:     Pointer to buffer to store values received from peer."]
    #[doc = " - __`cnt`__:       Number of 8-bit or 16-bit values to be transferred."]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    pub fn hal_spi_txrx(
        spi_num: ::cty::c_int,
        txbuf: *mut ::cty::c_void,
        rxbuf: *mut ::cty::c_void,
        cnt: ::cty::c_int,
    ) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Non-blocking interface to send a buffer and store received values. Can be"]
    #[doc = " used for both master and slave SPI types. The user must configure the"]
    #[doc = " callback (using hal_spi_set_txrx_cb); the txrx callback is executed at"]
    #[doc = " interrupt context when the buffer is sent."]
    #[doc = ""]
    #[doc = " The transmit and receive buffers are either arrays of 8-bit (uint8_t)"]
    #[doc = " values or 16-bit values depending on whether the spi is configured for 8 bit"]
    #[doc = " data or more than 8 bits per value. The 'cnt' parameter is the number of"]
    #[doc = " 8-bit or 16-bit values. Thus, if 'cnt' is 10, txbuf/rxbuf would point to an"]
    #[doc = " array of size 10 (in bytes) if the SPI is using 8-bit data; otherwise"]
    #[doc = " txbuf/rxbuf would point to an array of size 20 bytes (ten, uint16_t values)."]
    #[doc = ""]
    #[doc = " NOTE: these buffers are in the native endian-ness of the platform."]
    #[doc = ""]
    #[doc = "     MASTER: master sends all the values in the buffer and stores the"]
    #[doc = "             stores the values in the receive buffer if rxbuf is not NULL."]
    #[doc = "             The txbuf parameter cannot be NULL"]
    #[doc = "     SLAVE: Slave \"preloads\" the data to be sent to the master (values"]
    #[doc = "            stored in txbuf) and places received data from master in rxbuf"]
    #[doc = "            (if not NULL). The txrx callback occurs when len values are"]
    #[doc = "            transferred or master de-asserts chip select. If txbuf is NULL,"]
    #[doc = "            the slave transfers its default byte. Both rxbuf and txbuf cannot"]
    #[doc = "            be NULL."]
    #[doc = ""]
    #[doc = " - __`spi_num`__:   SPI interface to use"]
    #[doc = " - __`txbuf`__:     Pointer to buffer where values to transmit are stored."]
    #[doc = " - __`rxbuf`__:     Pointer to buffer to store values received from peer."]
    #[doc = " - __`cnt`__:       Number of 8-bit or 16-bit values to be transferred."]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    pub fn hal_spi_txrx_noblock(
        spi_num: ::cty::c_int,
        txbuf: *mut ::cty::c_void,
        rxbuf: *mut ::cty::c_void,
        cnt: ::cty::c_int,
    ) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Sets the default value transferred by the slave. Not valid for master"]
    #[doc = ""]
    #[doc = " - __`spi_num`__: SPI interface to use"]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    pub fn hal_spi_slave_set_def_tx_val(spi_num: ::cty::c_int, val: u16) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " This aborts the current transfer but keeps the spi enabled."]
    #[doc = ""]
    #[doc = " - __`spi_num`__:   SPI interface on which transfer should be aborted."]
    #[doc = ""]
    #[doc = " Return: int 0 on success, non-zero error code on failure."]
    #[doc = ""]
    #[doc = " NOTE: does not return an error if no transfer was in progress."]
    pub fn hal_spi_abort(spi_num: ::cty::c_int) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Extracts CPOL and CPHA values from a data-mode constant."]
    #[doc = " Utility function, defined once for every MCU."]
    #[doc = ""]
    #[doc = " - __`data_mode`__:             The HAL_SPI_MODE value to convert."]
    #[doc = " - __`out_cpol`__:              The CPOL gets written here on success."]
    #[doc = " - __`out_cpha`__:              The CPHA gets written here on success."]
    #[doc = ""]
    #[doc = " Return:                      0 on success; nonzero on invalid input."]
    pub fn hal_spi_data_mode_breakout(
        data_mode: u8,
        out_cpol: *mut ::cty::c_int,
        out_cpha: *mut ::cty::c_int,
    ) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " System reset."]
    pub fn hal_system_reset();
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Called by bootloader to start loaded program."]
    pub fn hal_system_start(img_start: *mut ::cty::c_void);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Called by split app loader to start the app program."]
    pub fn hal_system_restart(img_start: *mut ::cty::c_void);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Returns non-zero if there is a HW debugger attached."]
    pub fn hal_debugger_connected() -> ::cty::c_int;
}
#[doc = " Power on Reset"]
pub const hal_reset_reason_HAL_RESET_POR: hal_reset_reason = 1;
#[doc = " Caused by Reset Pin"]
pub const hal_reset_reason_HAL_RESET_PIN: hal_reset_reason = 2;
#[doc = " Caused by Watchdog"]
pub const hal_reset_reason_HAL_RESET_WATCHDOG: hal_reset_reason = 3;
#[doc = " Soft reset, either system reset or crash"]
pub const hal_reset_reason_HAL_RESET_SOFT: hal_reset_reason = 4;
#[doc = " Low supply voltage"]
pub const hal_reset_reason_HAL_RESET_BROWNOUT: hal_reset_reason = 5;
#[doc = " Restart due to user request"]
pub const hal_reset_reason_HAL_RESET_REQUESTED: hal_reset_reason = 6;
#[doc = " System Off, wakeup on external interrupt"]
pub const hal_reset_reason_HAL_RESET_SYS_OFF_INT: hal_reset_reason = 7;
#[doc = " Reboot reason"]
pub type hal_reset_reason = u32;
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Return the reboot reason"]
    #[doc = ""]
    #[doc = " Return: A reboot reason"]
    pub fn hal_reset_cause() -> hal_reset_reason;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Return the reboot reason as a string"]
    #[doc = ""]
    #[doc = " Return: String describing previous reset reason"]
    pub fn hal_reset_cause_str() -> *const ::cty::c_char;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Starts clocks needed by system"]
    pub fn hal_system_clock_start();
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Reset callback to be called before an reset happens inside hal_system_reset()"]
    pub fn hal_system_reset_cb();
}
