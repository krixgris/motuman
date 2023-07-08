use std::error::Error;
use std::io::{stdin, stdout, Write};
// use std::thread;
// use std::time::Duration;

use midir::{Ignore, MidiInput};
use motuman::motu::MotuCommand;

struct MidiCommand {
    // message field should be an array of 3 u8
    message: [u8; 3],
    motu_command: MotuCommand,
}
impl MidiCommand {
    fn new(message: &[u8], motu_command: MotuCommand) -> Option<Self> {
        if message.len() == 3 {
            let mut message_array: [u8; 3] = [0; 3];
            message_array.copy_from_slice(message);
            Some(Self {
                message: message_array,
                motu_command,
            })
        } else {
            None
        }
    }
}

trait MidiMessage {
    fn is_midi_cc(&self) -> bool;

    fn is_midi_note_on(&self) -> bool {
        // if self.len() == 3 && self[0] >> 4 == 0x9 {
        //     return true;
        // }
        false
    }

    fn is_midi_note_off(&self) -> bool {
        // if self.len() == 3 && self[0] >> 4 == 0x8 {
        //     return true;
        // }
        false
    }

    fn is_midi(&self) -> bool {
        false
    }

    fn channel(&self) -> Option<u8> {
        None
    }

    fn midi_type(&self) -> Option<u8> {
        None
    }
}

impl MidiMessage for &[u8] {
    fn is_midi_cc(&self) -> bool {
        if self.len() == 3 && self[0] >> 4 == 0xB {
            return true;
        }
        false
    }

    fn is_midi_note_off(&self) -> bool {
        if self.len() == 3 && self[0] >> 4 == 0x8 {
            return true;
        }
        false
    }

    fn is_midi_note_on(&self) -> bool {
        if self.len() == 3 && self[0] >> 4 == 0x9 {
            return true;
        }
        false
    }

    fn is_midi(&self) -> bool {
        if self.is_midi_cc() || self.is_midi_note_on() || self.is_midi_note_off() {
            return true;
        }
        false
    }

    fn channel(&self) -> Option<u8> {
        if self.is_midi() {
            Some(self[0] & 0x0F)
        } else {
            None
        }
    }

    fn midi_type(&self) -> Option<u8> {
        if self.is_midi() {
            Some(self[0] >> 4)
        } else {
            None
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
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
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
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            if message.is_midi() {
                println!(
                    "{}: Channel: {}, Type: {}, Num: {}, Value: {}, (len = {})",
                    stamp,
                    message.channel().unwrap(),
                    message.midi_type().unwrap(),
                    message[1],
                    message[2],
                    message.len()
                );
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
