use std::{io::Read};
use serialport::{SerialPort};

pub struct Serial {
    device : Box<dyn SerialPort>
}

impl Serial {
    pub fn open(path: &str, baud_rate: u32) -> std::io::Result<Self>
    {
        let mut device = serialport::new(path, baud_rate)
            .timeout(std::time::Duration::from_millis(100))
            .flow_control(serialport::FlowControl::None)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .parity(serialport::Parity::None)
            .open()?;

        device.write_data_terminal_ready(true)?;
        device.write_request_to_send(true)?;

        Ok (Self { device })
    }

    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
    {
        self.device.read(buf)
    }

    pub fn write(&mut self, buf : &[u8]) -> std::io::Result<usize>
    {
        self.device.write(buf)
    }

    pub fn flush(&mut self) -> std::io::Result<()>
    {
        self.device.flush()
    }

    pub fn print_available()
    {
        let ports = match serialport::available_ports()
        {
            Ok(ports) => ports,
            Err(_) =>
            {
                print!("No ports were found! \r\n");
                return;
            },
        };

        if ports.len() == 0 {
            print!("\nNo other available ports were found! \r\n\n");
            return;
        }

        print!("\nFound {} available port(s): \r\n\n", ports.len());

        for port in ports
        {
            match port.port_type
            {
                serialport::SerialPortType::UsbPort(info) =>
                {
                    print!("Port name: {} \r\n", port.port_name);
                    print!("  VID: {:04x} \r\n", info.vid);
                    print!("  PID: {:04x} \r\n", info.pid);

                    if let Some(sn) = info.serial_number
                    {
                        print!("  SER: {} \r\n", sn);
                    }

                    if let Some(man) = info.manufacturer
                    {
                        print!("  MAN: {} \r\n", man);
                    }

                    print!("\r\n");
                },
                _ =>
                {
                    eprint!("No ports found! \r\n");
                }
            }
        }
    }
}
