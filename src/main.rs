mod log;
mod output_writer;
mod serial;

use chrono::Local;
use clap::{Parser, CommandFactory};
use colored::*;

use crate::{
    log::{FileLogger, Logger, NoOpLogger},
    output_writer::OutputWriter,
    serial::Serial,
};

use std::{
    io::{self, Read},
    sync::{
        Arc, Once,
        atomic::{AtomicBool, Ordering::Relaxed},
        mpsc,
    },
    thread,
};

const EXIT_KEYS: [u8; 2] = [0x51, 0x71]; // T and Q keys
const CTRL_KEY: u8 = 0x14; // Ctrl key
static PRINT_PORT_STATUS: Once = Once::new();

#[derive(Parser)]
#[command(name = "rtio")]
#[command(version)]
#[command(long_about = "\n
Small serial/UART tool written in Rust.
In session, use 'Ctrl + T, Q' to exit from rtio.")]
#[command(author = "nullsych")]
struct Cli {
    #[arg(short, long, default_value_t = String::new(), help = "tty-device")]
    dev: String,

    #[arg(
        short,
        long,
        default_value_t = 115200,
        help = "Baud rate of the device"
    )]
    baud: u32,

    #[arg(short, long, help = "Log to file")]
    log: Option<String>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Prefix each new line with a timestamp"
    )]
    timestamp: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "List available serial devices"
    )]
    list: bool,
}

macro_rules! print_available {
    ($dev:expr) => {
        rtio_error!(
            "Failed to open serial port '{}', no such port! \r\n",
            $dev
        );
        Serial::print_available();
    };
}

macro_rules! rtio_message {
    ( $($arg:tt)* ) => {
        print!(
            "{} {}\r\n", format!("[rtio {}]", Local::now().format("%H:%M:%S")).yellow(),
            format!($($arg)*));
    };
}

macro_rules! rtio_error {
    ( $($arg:tt)* ) => {
        print!(
            "{} {}\r\n", format!("[rtio {}]", Local::now().format("%H:%M:%S")).red(),
            format!($($arg)*));
    };
}

fn open_port_forever(dev: &str, baud: u32, exit: &AtomicBool) -> Option<Serial> {
    loop {
        if exit.load(Relaxed) {
            return None;
        }

        match Serial::open(dev, baud) {
            Ok(port) => {
                rtio_message!(
                    "Connected to device {}, baud rate {}. Ctrl+T, Q to quit",
                    dev,
                    baud
                );
                PRINT_PORT_STATUS.call_once(|| { /* do nothing  */ });
                return Some(port);
            }

            Err(_) => {
                // if it's first entry - exit immediately
                if !PRINT_PORT_STATUS.is_completed() {
                    print_available!(dev);
                    return None;
                }

                PRINT_PORT_STATUS.call_once(|| {
                    print_available!(dev);
                });

                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let _ = crossterm::terminal::enable_raw_mode();

    let mut logger: Box<dyn Logger> = match cli.log {
        Some(path) => {
            rtio_message!("File logger started in {}", path);
            Box::new(FileLogger::open(&path)?)
        }
        None => {
            // no logger started
            Box::new(NoOpLogger)
        }
    };

    if cli.list {
        Serial::print_available();
        return Ok(());
    }

    if cli.dev.is_empty() {
        let _ = crossterm::terminal::disable_raw_mode();
        Cli::command().print_long_help().unwrap();
        println!();
        return Ok(());
    }

    if cli.timestamp {
        rtio_message!("Prefix each new line with a timestamp enabled");
    }

    let mut output = OutputWriter::new(cli.timestamp);
    let exit_rtio = Arc::new(AtomicBool::new(false));
    let (tx_serial, rx_serial) = mpsc::channel::<u8>();

    //
    // stdin thread
    //
    {
        let exit_rtio = Arc::clone(&exit_rtio);

        thread::spawn(move || {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();

            let mut exit_sequence = false;

            loop {
                let mut byte = [0u8; 1];

                match stdin.read(&mut byte) {
                    Ok(1) => {
                        let b = byte[0];

                        if exit_sequence {
                            exit_sequence = false;

                            if EXIT_KEYS.contains(&b) {
                                exit_rtio.store(true, Relaxed);
                                break;
                            }

                            continue;
                        }

                        if b == CTRL_KEY {
                            exit_sequence = true;
                            continue;
                        }

                        let _ = tx_serial.send(b);
                    }
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        });
    }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut rx_window = [0u8; 2048];

    //
    // output writer
    //
    loop {
        if exit_rtio.load(Relaxed) {
            break;
        }

        let mut port = match open_port_forever(&cli.dev, cli.baud, &exit_rtio) {
            Some(port) => port,
            None => break,
        };

        loop {
            if exit_rtio.load(Relaxed) {
                break;
            }

            //
            // TX channel
            //
            while let Ok(byte) = rx_serial.try_recv() {
                if port.write(&[byte]).is_err() {
                    rtio_error!("Serial TX error");
                    break;
                }

                let _ = port.flush();
            }

            //
            // RX channel
            //
            match port.read(&mut rx_window) {
                Ok(count) if count > 0 => {
                    let _ = output.write(&rx_window[..count], &mut stdout);
                    logger.log(&rx_window[..count]);
                }

                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}

                Ok(_) => {}

                Err(_) => {
                    rtio_message!("Serial disconnected!");
                    break;
                }
            }
        }

        if exit_rtio.load(Relaxed) {
            break;
        }
    }

    rtio_message!("Quit rtio");

    let _ = crossterm::terminal::disable_raw_mode();

    Ok(())
}
