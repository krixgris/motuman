use std::error::Error;
use std::io::{stdin, stdout, Write};
// use std::thread;
// use std::time::Duration;

use midir::{Ignore, MidiInput};

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

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            if message.len() == 3 {
                let (channel, midi_type, midi_num, midi_value) =
                    (message[0] & 0x0F, message[0] >> 4, message[1], message[2]);
                println!(
                    "{}: Channel: {}, Type: {}, Num: {}, Value: {}, (len = {})",
                    stamp, channel, midi_type, midi_num, midi_value, message[0]
                );
                // println!("{}: {:?} (len = {})", stamp, message, message.len());
            }
            match message[0] {
                // bitwise match 4 last bits to be channel number, and first 4 bits to be the message type
                // match channel 16 midi note on
                // bitwise or on message
                0x90 => println!(
                    "{}: Note on: {:?} (len = {})",
                    stamp,
                    message,
                    message.len()
                ),
                0x80 => println!(
                    "{}: Note off: {:?} (len = {})",
                    stamp,
                    message,
                    message.len()
                ),
                // match decimal 176 as hex

                // Alternatively, you can use bitwise OR to combine two u8 values into a single byte:
                // let high = 0b1101;
                // let low = 0b0110;
                // let byte = (high << 4) | low;
                // println!("Byte: {:08b}", byte);
                // match midi cc on channel 16
                _ => println!("{}: Else: {:?} (len = {})", stamp, message, message.len()),
            }
            // println!("{}: {:?} (len = {})", stamp, message, message.len());
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
