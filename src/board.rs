use crate::avrdude;

pub trait Board {
    fn display_name(&self) -> &str;
    fn needs_reset(&self) -> Option<&str>;
    fn avrdude_options(&self) -> avrdude::AvrdudeOptions;
    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>>;
}

pub fn get_board(board: &str) -> Option<Box<dyn Board>> {
    Some(match board {
        "niti-v1" => Box::new(NitiV1),
        "uno" => Box::new(ArduinoUno),
        "nano" => Box::new(ArduinoNano),
        "mega2560" => Box::new(ArduinoMega2560),
        _ => return None,
    })
}

// ----------------------------------------------------------------------------

fn find_port_from_vid_pid_list(list: &[(u16, u16)]) -> anyhow::Result<std::path::PathBuf> {
    for serialport::SerialPortInfo {
        port_name,
        port_type,
    } in serialport::available_ports().unwrap()
    {
        if let serialport::SerialPortType::UsbPort(usb_info) = port_type {
            for (vid, pid) in list.iter() {
                if usb_info.vid == *vid && usb_info.pid == *pid {
                    return Ok(port_name.into());
                }
            }
        }
    }
    Err(anyhow::anyhow!("Serial port not found."))
}

// ----------------------------------------------------------------------------

struct NitiV1;

impl Board for NitiV1 {
    fn display_name(&self) -> &str {
        "Niti Dev Board"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "wiring",
            partno: "atmega2560",
            baudrate: Some(115200),
            do_chip_erase: false,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(find_port_from_vid_pid_list(&[
            (0x2341, 0x0010),
            (0x2341, 0x0042),
            (0x2A03, 0x0010),
            (0x2A03, 0x0042),
            (0x2341, 0x0210),
            (0x2341, 0x0242),
        ]))
    }
}

struct ArduinoUno;

impl Board for ArduinoUno {
    fn display_name(&self) -> &str {
        "Arduino Uno"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: None,
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(find_port_from_vid_pid_list(&[
            (0x2341, 0x0043),
            (0x2341, 0x0001),
            (0x2A03, 0x0043),
            (0x2341, 0x0243),
        ]))
    }
}


struct ArduinoNano;

impl Board for ArduinoNano {
    fn display_name(&self) -> &str {
        "Arduino Nano"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: Some(57600),
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(Err(anyhow::anyhow!("Not able to guess port")))
    }
}


struct ArduinoMega2560;

impl Board for ArduinoMega2560 {
    fn display_name(&self) -> &str {
        "Arduino Mega 2560"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "wiring",
            partno: "atmega2560",
            baudrate: Some(115200),
            do_chip_erase: false,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(find_port_from_vid_pid_list(&[
            (0x2341, 0x0010),
            (0x2341, 0x0042),
            (0x2A03, 0x0010),
            (0x2A03, 0x0042),
            (0x2341, 0x0210),
            (0x2341, 0x0242),
        ]))
    }
}
