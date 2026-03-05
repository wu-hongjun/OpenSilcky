use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use slicky_core::{Color, HidSlickyDevice, Preset, SlickyDevice};

#[derive(Parser)]
#[command(
    name = "slicky",
    version,
    about = "Control your Slicky USB status light"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set light to a named preset (e.g., red, busy, available, in-meeting)
    Set { name: String },
    /// Set light to exact RGB values (0-255 each)
    Rgb { r: u8, g: u8, b: u8 },
    /// Set light to a hex color (#RRGGBB or RRGGBB)
    Hex { color: String },
    /// Turn the light off
    Off,
    /// List all available preset names and their colors
    Presets,
    /// List connected Slicky devices
    Devices,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { name } => {
            let preset = Preset::from_name(&name).context("failed to resolve preset")?;
            let color = preset.color();
            let device = open_device()?;
            device.set_color(color).context("failed to set color")?;
            println!("Set to {} ({})", preset.name(), color);
        }
        Commands::Rgb { r, g, b } => {
            let color = Color::new(r, g, b);
            let device = open_device()?;
            device.set_color(color).context("failed to set color")?;
            println!("Set to RGB({}, {}, {}) {}", r, g, b, color);
        }
        Commands::Hex { color: hex } => {
            let color = Color::from_hex(&hex).context("failed to parse hex color")?;
            let device = open_device()?;
            device.set_color(color).context("failed to set color")?;
            println!("Set to {color}");
        }
        Commands::Off => {
            let device = open_device()?;
            device.off().context("failed to turn off")?;
            println!("Light off");
        }
        Commands::Presets => {
            println!("{:<15}COLOR", "NAME");
            println!("{}", "-".repeat(28));
            for p in Preset::all() {
                println!("{:<15}{}", p.name(), p.color());
            }
        }
        Commands::Devices => {
            let devices = HidSlickyDevice::enumerate().context("failed to enumerate devices")?;
            if devices.is_empty() {
                println!("No Slicky devices found");
            } else {
                for (i, d) in devices.iter().enumerate() {
                    println!("Device {}:", i + 1);
                    if let Some(ref s) = d.serial {
                        println!("  Serial:       {s}");
                    }
                    if let Some(ref m) = d.manufacturer {
                        println!("  Manufacturer: {m}");
                    }
                    if let Some(ref p) = d.product {
                        println!("  Product:      {p}");
                    }
                }
            }
        }
    }

    Ok(())
}

fn open_device() -> Result<HidSlickyDevice> {
    HidSlickyDevice::open().context("failed to open Slicky device")
}
