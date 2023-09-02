use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};
use motuman::motu::{self};

use motuman::config;
use motuman::midi::{midicommand::MidiCommand, miditype::MidiType};

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
            let midi_command = MidiCommand::new(&midi_message, value.clone());
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
            let midi_command = MidiCommand::new(&midi_message, value.clone());
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
            let midi_command = MidiCommand::new(&midi_message, value.clone());
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

    println!("Initializing midi...");
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    println!("Midi initialized.");

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    for port in &in_ports {
        println!("Found input port: {}", midi_in.port_name(port).unwrap());
    }

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

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            if message.is_midi() {
                // match incoming message with the list of midi_commands, where the message field can match on the first 2 elements
                let midi_command = midi_commands.iter_mut().find(|midi_command| {
                    midi_command.message[0] == message[0] && midi_command.message[1] == message[1]
                });
                if let Some(midi_command) = midi_command {
                    // will always be Ok()
                    let _ = midi_command.set_midi_value(message[2]);

                    if midi_command.do_throttle() {
                        motu_interface
                            .run(&midi_command.motu_commands())
                            .expect("error running motu command.");
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
