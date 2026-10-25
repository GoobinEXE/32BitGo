use std::process::ExitCode;

use anyhow::{bail, Context, Result};
use bitgo_core::{DeviceDriver, DeviceIdentity, DeviceKind, UsbId};
use bitgo_device_retro_keyboard::{detect_edition, is_retro_keyboard_candidate, RetroKeyboardDriver};
use bitgo_hid::enumerate_eight_bit_do;
use bitgo_theme::resolve_from_hint;
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "bitgo")]
#[command(about = "32BitGo — native 8BitDo Retro Keyboard tooling (NES Edition first)")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Theme {
        #[arg(long, default_value = "nes")]
        edition: String,
    },
    Inspect,
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => cmd_list(),
        Commands::Theme { edition } => cmd_theme(&edition),
        Commands::Inspect => cmd_inspect(),
    }
}

fn cmd_list() -> Result<()> {
    let devices = enumerate_eight_bit_do().context("enumerating HID devices")?;
    if devices.is_empty() {
        println!("Nenhum dispositivo 8BitDo (VID 0x2DC8) encontrado.");
        println!("No Linux, confira packaging/udev/99-32bitgo-8bitdo.rules e o replug.");
        return Ok(());
    }

    for (idx, d) in devices.iter().enumerate() {
        let usb = UsbId {
            vid: d.vid,
            pid: d.pid,
        };
        let candidate = is_retro_keyboard_candidate(usb, &d.product);
        let edition = detect_edition(&d.product);
        println!(
            "[{idx}] {vid:04x}:{pid:04x}  {product}  (iface {iface}, usage_page=0x{up:04x}){tag}",
            vid = d.vid,
            pid = d.pid,
            product = if d.product.is_empty() {
                "<unknown product>"
            } else {
                d.product.as_str()
            },
            iface = d.interface_number,
            up = d.usage_page,
            tag = if candidate {
                match edition {
                    Some(e) => format!("  [retro-keyboard candidate: {e:?}]"),
                    None => "  [retro-keyboard candidate]".to_string(),
                }
            } else {
                String::new()
            }
        );
        if !d.path.is_empty() {
            println!("     path: {}", d.path);
        }
    }
    Ok(())
}

fn cmd_theme(edition: &str) -> Result<()> {
    let hint = match edition.to_ascii_lowercase().as_str() {
        "nes" | "n" => bitgo_core::ThemeHint::Nes,
        other => bail!("tema/edição desconhecida neste marco: {other} (apenas nes)"),
    };
    let tokens = resolve_from_hint(hint);
    println!("theme: {} ({:?})", tokens.display_name, tokens.id);
    println!(
        "  background=#{:06x} accent=#{:06x} text=#{:06x}",
        tokens.colors.background & 0x00FF_FFFF,
        tokens.colors.accent & 0x00FF_FFFF,
        tokens.colors.text & 0x00FF_FFFF
    );
    Ok(())
}

fn cmd_inspect() -> Result<()> {
    let devices = enumerate_eight_bit_do().context("enumerating HID devices")?;
    let Some(d) = devices.into_iter().find(|d| {
        is_retro_keyboard_candidate(
            UsbId {
                vid: d.vid,
                pid: d.pid,
            },
            &d.product,
        )
    }) else {
        bail!("nenhum candidato a Retro Keyboard encontrado");
    };

    let edition = detect_edition(&d.product).unwrap_or(bitgo_core::KeyboardEdition::Nes);
    let identity = DeviceIdentity {
        usb: UsbId {
            vid: d.vid,
            pid: d.pid,
        },
        kind: DeviceKind::RetroKeyboard,
        edition: Some(edition),
        product_name: d.product.clone(),
        manufacturer: d.manufacturer.clone(),
        path: d.path.clone(),
    };

    let driver = RetroKeyboardDriver::new(identity);
    let caps = driver.capabilities();
    let theme = resolve_from_hint(caps.theme_hint);

    info!(
        product = %driver.identity().product_name,
        edition = ?driver.identity().edition,
        theme = %theme.display_name,
        "retro keyboard plugin bound"
    );

    println!("identity: {:?}", driver.identity());
    println!("capabilities: {caps:?}");
    println!("resolved theme: {}", theme.display_name);
    println!(
        "protocol maps: pending USB capture (see PLAN.md). DFU/OTA paths blocked in bitgo-core."
    );
    Ok(())
}
