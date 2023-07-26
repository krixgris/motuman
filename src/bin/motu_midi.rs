use std::error::Error;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};
use motuman::motu::{self, MotuCommand};

use motuman::config;

#[derive(Debug)]
enum MidiType {
    CC,
    NoteOn,
    NoteOff,
    Undefined,
}

impl Display for MidiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiType::CC => write!(f, "CC"),
            MidiType::NoteOn => write!(f, "NoteOn"),
            MidiType::NoteOff => write!(f, "NoteOff"),
            MidiType::Undefined => write!(f, "UNDEFINED"),
        }
    }
}

impl From<&[u8]> for MidiType {
    fn from(message: &[u8]) -> Self {
        if message.len() == 3 && message[0] >> 4 == 0xB {
            MidiType::CC
        } else if message.len() == 3 && message[0] >> 4 == 0x9 {
            MidiType::NoteOn
        } else if message.len() == 3 && message[0] >> 4 == 0x8 {
            MidiType::NoteOff
        } else {
            MidiType::Undefined
        }
    }
}

impl From<&u8> for MidiType {
    fn from(midi_type: &u8) -> Self {
        match midi_type >> 4 {
            0xB => MidiType::CC,
            0x9 => MidiType::NoteOn,
            0x8 => MidiType::NoteOff,
            _ => MidiType::Undefined,
        }
    }
}

impl From<MidiType> for u8 {
    fn from(midi_type: MidiType) -> Self {
        match midi_type {
            MidiType::CC => 0xB,
            MidiType::NoteOn => 0x9,
            MidiType::NoteOff => 0x8,
            MidiType::Undefined => 0xFF,
        }
    }
}

impl From<MidiType> for &u8 {
    fn from(midi_type: MidiType) -> Self {
        match midi_type {
            MidiType::CC => &0xB,
            MidiType::NoteOn => &0x9,
            MidiType::NoteOff => &0x8,
            MidiType::Undefined => &0xFF,
        }
    }
}

#[derive(Debug)]
struct MidiCommand {
    // message field should be an array of 3 u8
    message: [u8; 3],
    midi_value: u8,
    prev_midi_value: u8,
    motu_command: MotuCommand,
    timestamp: u64,
    prev_timestamp: u64,
}
impl MidiCommand {
    fn new(message: &[u8], motu_command: MotuCommand) -> Option<Self> {
        if message.len() == 3 {
            let mut message_array: [u8; 3] = [0; 3];
            message_array.copy_from_slice(message);
            Some(Self {
                message: message_array,
                motu_command,
                timestamp: 10000,
                midi_value: 0,
                prev_midi_value: 127,
                prev_timestamp: 0,
            })
        } else {
            None
        }
    }

    fn delta_value(&self) -> f32 {
        // abs delta value
        (self.midi_value as f32 - self.prev_midi_value as f32).abs()
    }

    fn delta_time(&self) -> u64 {
        self.timestamp - self.prev_timestamp
    }

    /// Determines whether the MIDI command should be throttled based on the delta time and delta value.
    /// Returns `true` if the command should be throttled, `false` otherwise.
    fn do_throttle(&mut self) -> bool {
        let delta_time = self.delta_time();
        let delta_value = {
            if delta_time > 1000 {
                1000.0
            } else {
                self.delta_value()
            }
        };

        // dbg!(delta_time, delta_value);

        if (100 >= delta_time && delta_time > 10 && delta_value > 5.0)
            || (150 >= delta_time && delta_time > 100 && delta_value > 2.0)
            || (250 >= delta_time && delta_time > 150 && delta_value > 0.0)
            || (delta_time > 250 && delta_value > 0.0)
            || delta_value > 30.0
            || self.midi_value <= 2
            || self.midi_value >= 125
        {
            // if true, set the prev_value and prev_time to the current values
            self.prev_midi_value = self.midi_value;
            self.prev_timestamp = self.timestamp;
            true
        } else {
            false
        }
    }

    pub fn set_midi_value(&mut self, midi_value: u8) -> Result<(), String> {
        self.midi_value = midi_value;
        self.prev_timestamp = self.timestamp;
        self.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_millis() as u64;

        let mut motu_command = match self.motu_command {
            MotuCommand::Volume { channel, volume: _ } => MotuCommand::Volume {
                channel,
                volume: easing_circ(midi_value as f32 / 127.0),
            },
            MotuCommand::Send {
                channel,
                aux_channel,
                value: _,
            } => MotuCommand::Send {
                channel,
                aux_channel,
                value: easing_circ(midi_value as f32 / 127.0),
            },
            _ => self.motu_command,
        };
        std::mem::swap(&mut motu_command, &mut self.motu_command);
        Ok(())
    }
}

trait EasingAlgorithm {
    fn easing(x: f32) -> f32;
}

fn easing_circ(x: f32) -> f32 {
    1.0 - (1.0 - x * x).sqrt()
}
trait MidiMessage {
    fn is_midi(&self) -> bool {
        false
    }

    fn channel(&self) -> Option<u8> {
        None
    }

    fn midi_type(&self) -> Option<MidiType> {
        None
    }
}

impl MidiMessage for &[u8] {
    fn is_midi(&self) -> bool {
        self.midi_type().is_some()
    }

    fn channel(&self) -> Option<u8> {
        if self.is_midi() {
            Some((self[0] & 0x0F) + 1)
        } else {
            None
        }
    }

    fn midi_type(&self) -> Option<MidiType> {
        match self {
            &[midi_type, _, _] => Some(midi_type.into()),
            _ => None,
        }
    }
}

/*
fn main() {
    let mut args = std::env::args().skip(1);
    let midi_in = args.next().expect("Usage: motu_midid <midi-in>");
    let midi_in_ports = midir::MidiInput::new("motu_midid").unwrap().ports();
    if midi_in == "--help" {
        println!("Available MIDI inputs:");
        for (i, port) in midi_in_ports.iter().enumerate() {
            println!("{}: {}", i, port.into());
        }
        return;
    }
    let midi_in_port = midi_in_ports[midi_in.parse::<usize>().unwrap()];
    let mut midi_in_conn = MidiInput::new("motumidid").unwrap().connect(
        &midi_in_port,
        "motumidid input",
        move |_, message, _| {
            if let MidiMessage::ControlChange(_, 66, 66) = message {
                println!("Aborting...");
                std::process::exit(0);
            }
            println!("Received MIDI message: {:?}", message);
        },
        (),
    ).unwrap();
    println!("Listening for MIDI input on {}...", midi_in_port);
    loop {
        thread::sleep(Duration::from_millis(10));
        if let Ok(true) = stdin().lock().bytes().next().map(|b| b == b'q' || b == b'Q') {
            println!("Quitting...");
            std::process::exit(0);
        }
    }
}
*/

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config_file_name = String::from("./motu_config.toml");
    let config = config::Config::build(config_file_name, None).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    let midi_input_device = config.midi_config.clone().unwrap().input;

    let mut midi_commands: Vec<MidiCommand> = Vec::new();
    let midi_channel = config.midi_config.clone().unwrap().midi_channel - 1;

    let midi_commands_cc: Vec<MidiCommand> = config
        .midi_mapping_cc
        .iter()
        .map(|(key, value)| {
            let cc_num: u8 = *key as u8;
            let midi_channel_type = (midi_channel) + (0x0B << 4);
            let midi_message: [u8; 3] = [midi_channel_type, cc_num, 0];
            let midi_command = MidiCommand::new(&midi_message, *value);
            midi_command.unwrap()
        })
        .collect();

    let midi_commands_note_on: Vec<MidiCommand> = config
        .midi_mapping_note_on
        .iter()
        .map(|(key, value)| {
            let note_num: u8 = *key as u8;
            let midi_channel_type = (midi_channel) + (0x09 << 4);
            let midi_message: [u8; 3] = [midi_channel_type, note_num, 0];
            let midi_command = MidiCommand::new(&midi_message, *value);
            midi_command.unwrap()
        })
        .collect();

    let midi_commands_note_off: Vec<MidiCommand> = config
        .midi_mapping_note_off
        .iter()
        .map(|(key, value)| {
            let note_num: u8 = *key as u8;
            let midi_channel_type = (midi_channel) + (0x08 << 4);
            let midi_message: [u8; 3] = [midi_channel_type, note_num, 0];
            let midi_command = MidiCommand::new(&midi_message, *value);
            midi_command.unwrap()
        })
        .collect();

    midi_commands.extend(midi_commands_cc);
    midi_commands.extend(midi_commands_note_on);
    midi_commands.extend(midi_commands_note_off);

    // println!("MIDI Commands: {:?}", midi_commands);

    let ip: &str = &config.ip_address.address.to_string();
    let port = &config.ip_address.port.to_string();
    let motu_interface = motu::Motu::new(ip, port, &config)
        .expect("Error creating Motu object, check motu_config.toml file.");

    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();

    // if midi_input_device exists in in_ports, then use that port, otherwise, use the match statement below
    let in_port = match in_ports.iter().find(|port| {
        midi_in
            .port_name(port)
            .unwrap()
            .to_lowercase()
            .contains(&midi_input_device.to_lowercase())
    }) {
        Some(port) => port,
        None => {
            println!(
                "No MIDI input device found with name: {}",
                midi_input_device
            );
            // return Err("no input port found".into());
            match in_ports.len() {
                0 => return Err("no input port found".into()),
                1 => {
                    println!(
                        "Choosing the only available input port: {}",
                        midi_in.port_name(&in_ports[0]).unwrap()
                    );
                    &in_ports[0]
                }
                _ => {
                    println!("\nAvailable input ports:");
                    for (i, p) in in_ports.iter().enumerate() {
                        println!("{}: {}", i, midi_in.port_name(p).unwrap());
                    }
                    print!("Please select input port: ");
                    stdout().flush()?;
                    let mut input = String::new();
                    stdin().read_line(&mut input)?;
                    in_ports
                        .get(input.trim().parse::<usize>()?)
                        .ok_or("invalid input port selected")?
                }
            }
        }
    };

    // let in_port = match in_ports.len() {
    //     0 => return Err("no input port found".into()),
    //     1 => {
    //         println!(
    //             "Choosing the only available input port: {}",
    //             midi_in.port_name(&in_ports[0]).unwrap()
    //         );
    //         &in_ports[0]
    //     }
    //     _ => {
    //         println!("\nAvailable input ports:");
    //         for (i, p) in in_ports.iter().enumerate() {
    //             println!("{}: {}", i, midi_in.port_name(p).unwrap());
    //         }
    //         print!("Please select input port: ");
    //         stdout().flush()?;
    //         let mut input = String::new();
    //         stdin().read_line(&mut input)?;
    //         in_ports
    //             .get(input.trim().parse::<usize>()?)
    //             .ok_or("invalid input port selected")?
    //     }
    // };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            if message.is_midi() {
                // match incoming message with the list of midi_commands, where the message field can match on the first 2 elements
                let midi_command = midi_commands.iter_mut().find(|midi_command| {
                    midi_command.message[0] == message[0] && midi_command.message[1] == message[1]
                });
                match midi_command {
                    Some(midi_command) => {
                        // will always be Ok()
                        let _ = midi_command.set_midi_value(message[2]);

                        if midi_command.do_throttle() {
                            // println!("MIDI Command: {:?}", midi_command);
                            motu_interface
                                .run(vec![midi_command.motu_command])
                                .expect("Error running MOTU command.");
                            // } else {
                            //     println!("Throttling MIDI Command: {:?}", midi_command);
                        }
                        // println!("MIDI Command: {:?}", midi_command);
                    }
                    None => {
                        println!("MIDI Command not found: {:?}", message);
                    }
                }
                // println!(
                //     "{}: Channel: {}, Type: {}, Num: {}, Value: {}, (len = {})",
                //     stamp,
                //     message.channel().unwrap(),
                //     message.midi_type().unwrap(),
                //     message[1],
                //     message[2],
                //     message.len()
                // );
                // let (channel, message) =
                //     (message.channel().unwrap() - 1, message.midi_type().unwrap());
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (type 'Q' and hit enter to exit) ...",
        in_port_name
    );
    // loop {
    //     std::thread::sleep(Duration::from_millis(1));
    // }
    loop {
        input.clear();
        stdin().read_line(&mut input)?; // wait for next enter key press
        if input.contains('Q') || input.contains('q') {
            break;
        }
    }

    println!("Closing connection");
    Ok(())
}
