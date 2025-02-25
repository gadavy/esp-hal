fn main() {
    let esp32 = cfg!(feature = "esp32");
    let esp32c2 = cfg!(feature = "esp32c2");
    let esp32c3 = cfg!(feature = "esp32c3");
    let esp32s2 = cfg!(feature = "esp32s2");
    let esp32s3 = cfg!(feature = "esp32s3");

    // Ensure that exactly one chip has been specified
    let chip_features = [esp32, esp32c2, esp32c3, esp32s2, esp32s3];
    match chip_features.iter().filter(|&&f| f).count() {
        1 => {}
        n => panic!("Exactly 1 chip must be enabled via its Cargo feature, {n} provided"),
    }

    // Define all required configuration symbols for the enabled chip.
    //
    // When adding a new device, at the bare minimum the following symbols MUST be
    // defined:
    //   - the name of the device
    //   - the architecture ('riscv' or 'xtensa')
    //   - the core count ('single_core' or 'multi_core')
    //
    // Additionally, the following symbols MAY be defined if present:
    //   - 'dac'
    //   - 'gdma'
    //   - 'i2c1'
    //   - 'pdma'
    //   - 'rmt'
    //   - 'spi3'
    //   - 'systimer'
    //   - 'timg1'
    //   - 'uart2'
    //   - 'usb_otg'
    //   - 'usb_serial_jtag'
    //
    // New symbols can be added as needed, but please be sure to update both this
    // comment and the required vectors below.
    let symbols = if esp32 {
        vec![
            "esp32",
            "xtensa",
            "multi_core",
            "dac",
            "i2c1",
            "pdma",
            "rmt",
            "spi3",
            "timg1",
            "uart2",
        ]
    } else if esp32c2 {
        vec!["esp32c2", "riscv", "single_core", "gdma", "systimer"]
    } else if esp32c3 {
        vec![
            "esp32c3",
            "riscv",
            "single_core",
            "gdma",
            "rmt",
            "spi3",
            "systimer",
            "timg1",
            "usb_serial_jtag",
        ]
    } else if esp32s2 {
        vec![
            "esp32s2",
            "xtensa",
            "single_core",
            "dac",
            "i2c1",
            "pdma",
            "rmt",
            "spi3",
            "systimer",
            "timg1",
            "usb_otg",
        ]
    } else if esp32s3 {
        vec![
            "esp32s3",
            "xtensa",
            "multi_core",
            "gdma",
            "i2c1",
            "rmt",
            "spi3",
            "systimer",
            "timg1",
            "uart2",
            "usb_otg",
            "usb_serial_jtag",
        ]
    } else {
        unreachable!(); // We've already confirmed exactly one chip was selected
    };

    for symbol in symbols {
        println!("cargo:rustc-cfg={symbol}");
    }
}
