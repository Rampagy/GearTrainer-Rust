# GearTrainer-Rust

Project that does the Gear Trainer, but in rust using a NUCLEO-F446RE (STM32F446RE MCU) instead of an Arduino Uno (ATmega328P MCU)

## Getting set-up

1. Install cargo/rust
    1. https://www.rust-lang.org/tools/install
1. Update cargo/rust
    1. Run `rustup update` in command line
1. Install arm platform support
    1. Run `rustup target add thumbv7em-none-eabihf` in yout terminal
    1. https://doc.rust-lang.org/rustc/platform-support/arm-none-eabi.html
1. Navigate to desired project folder
    1. ExerciseX/
1. Add the llvm tools
    1. Run `rustup component add llvm-tools` in your terminal
1.  Add cargo-binutils 
    1. Run `cargo install cargo-binutils` in your terminal
1. Add cargo embed tools
    1. Run `cargo install cargo-embed`

## Building and Flashing
1. Build the bin file
    1. Run `cargo build --debug` in your terminal
    1. Run `cargo build -- release` in your terminal
1. Flash the bin file (make sure it's plugged in and that you only have 1 plugged in)
    1. Run `cargo embed --chip STM32F446RETx` in your terminal
    1. Or `cargo embed` as it's listed in the Embed.toml


## Resources

1. User Manual - https://www.st.com/resource/en/reference_manual/rm0390-stm32f446xx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf 