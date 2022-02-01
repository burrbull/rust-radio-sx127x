use simplelog::LevelFilter;
use structopt::StructOpt;
use driver_pal::hal::{DeviceConfig};
use radio::helpers::Operation;

#[derive(StructOpt)]
#[structopt(name = "Sx127x-util")]
/// A Command Line Interface (CLI) for interacting with a local Sx127x radio device
pub struct Options {
    #[structopt(subcommand)]
    /// Subcommand to execute
    pub command: Command,
    
    #[structopt(flatten)]
    pub spi_config: DeviceConfig,

    /// Log verbosity setting
    #[structopt(long, default_value = "info")]
    pub log_level: LevelFilter,
}

#[derive(StructOpt, PartialEq, Debug)]
pub enum Command {
    #[structopt(name = "chip-version")]
    /// Fetch the device silicon/firmware version
    SiliconVersion,

    #[structopt(name = "lora")]
    /// LoRa mode configuration and operations
    LoRa(LoRaCommand),

    #[structopt(name = "gfsk")]
    /// GFSK mode configuration and operations
    Gfsk(GfskCommand),
}

#[derive(StructOpt, PartialEq, Debug)]
pub struct LoRaCommand {
    /// LoRa frequency in MHz
    #[structopt(long = "freq-mhz")]
    pub freq_mhz: Option<u32>,

    #[structopt(subcommand)]
    /// Operation to execute
    pub operation: Operation,
}

#[derive(StructOpt, PartialEq, Debug)]
pub struct GfskCommand {
    /// GFSK frequency in MHz
    #[structopt(long = "freq-mhz")]
    pub freq_mhz: Option<u32>,

    #[structopt(subcommand)]
    /// Operation to execute
    pub operation: Operation,
}
