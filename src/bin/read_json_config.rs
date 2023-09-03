// open the json file from ./pyrefs/oscconfig.json
// deserialize with serde_json. the file contains oscConfig as the top level, and under that is a list of properties, where we want to deserialize into control_change, note_on, note_off which are all structs containing a list of either note numbers or control change numbers, and from those numbers we want to save the address fields
// we want to save the address fields into a hashmap, where the key is the note number or control change number, and the value is the address

// generate this as a main function in this binary crate
// use serde_json to deserialize the json file into a struct
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

struct TopLevel {
    #[serde(rename = "oscConfig")]
    osc_config: OscConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct OscConfig {
    control_change: HashMap<u8, ControlChange>,
    note_on: HashMap<u8, NoteOn>,
    note_off: HashMap<u8, NoteOff>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ControlChange {
    #[serde(rename = "type")]
    cc_type: String,
    // #[serde(rename = "address")]
    address: Option<String>,
    command: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NoteOn {
    address: Option<String>,
    command: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NoteOff {
    address: Option<String>,
    command: Option<String>,
}

fn main() {
    let file = File::open("./pyrefs/oscconfig.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let osc_config: TopLevel = serde_json::from_reader(reader).expect("Failed to deserialize JSON");
    // println!("{:#?}", osc_config);

    // match osc_config.osc_config.control_change.get(&1) {
    //     Some(cc) => println!("cc: {:#?}", cc),
    //     None => println!("no cc found"),
    // }
    let cc_config: HashMap<u8, String> =
    osc_config.osc_config.control_change.into_iter()
    .filter(|(_, cc)| cc.address.is_some())
    .map(|(key, cc)|
        // (key, cc.address.unwrap())
        {
            // match with condition when address contains /fader and when it contains /send
            // and if send, replace the "/mix/chan/42/matrix/aux/0/send" string with send(42,0)
            match cc.address.unwrap() {
                address if address.contains("/fader") => (key, address.replace("/mix/chan/", "vol(").replace("/matrix/fader", ")")),
                address if address.contains("/send") => (key, address.replace("/mix/chan/", "send(").replace("/matrix/aux/", ",").replace("/send", ")")),
                _ => (key, String::from("no address")),
            }
        }
    )
    .collect();

    let note_on_config: HashMap<u8, String> =
    osc_config.osc_config.note_on.into_iter()
    .filter(|(_, note_on)| note_on.address.is_some())
    .map(|(key, note_on)|
        // (key, cc.address.unwrap())
        {
            // match with condition when address contains /fader and when it contains /send
            // and if send, replace the "/mix/chan/42/matrix/aux/0/send" string with send(42,0)
            match note_on.address.unwrap() {
                address if address.contains("/fader") => (key, address.replace("/mix/chan/", "vol(").replace("/matrix/fader", ")")),
                address if address.contains("/send") => (key, address.replace("/mix/chan/", "send(").replace("/matrix/aux/", ",").replace("/send", ")")),
                _ => (key, String::from("no address")),
            }
        }
    )
    .collect();

    let note_off_config: HashMap<u8, String> =
    osc_config.osc_config.note_off.into_iter()
    .filter(|(_, note_off)| note_off.address.is_some())
    .map(|(key, note_off)|
        // (key, cc.address.unwrap())
        {
            // match with condition when address contains /fader and when it contains /send
            // and if send, replace the "/mix/chan/42/matrix/aux/0/send" string with send(42,0)
            match note_off.address.unwrap() {
                address if address.contains("/fader") => (key, address.replace("/mix/chan/", "vol(").replace("/matrix/fader", ")")),
                address if address.contains("/send") => (key, address.replace("/mix/chan/", "send(").replace("/matrix/aux/", ",").replace("/send", ")")),
                _ => (key, String::from("no address")),
            }
        }
    )
    .collect();

    // order cc_config by key
    // let cc_config = cc_config.into_iter().collect::<Vec<_>>().sort_by(|a, b| a.0.cmp(&b.0));
    // let note_on_config = note_on_config.into_iter().collect::<Vec<_>>().sort_by(|a, b| a.0.cmp(&b.0));
    // let note_off_config = note_off_config.into_iter().collect::<Vec<_>>().sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:#?}", cc_config);
    // sort cc_config by key and store into a new hashmap
    // let cc_config = cc_config.into_iter().collect::<Vec<_>>().sort_by(|a, b| a.0.cmp(&b.0));

    let mut sorted_cc: Vec<_> = cc_config.into_iter().collect();
    sorted_cc.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:#?}", sorted_cc);
    // let cc_config: HashMap<u8, String> = sorted_cc.into_iter().collect();
    let mut sorted_note_on: Vec<_> = note_on_config.into_iter().collect();
    sorted_note_on.sort_by(|a, b| a.0.cmp(&b.0));
    // let note_on_config: HashMap<u8, String> = sorted_note_on.into_iter().collect();
    // println!("{:#?}", sorted_note_on);
    let mut sorted_note_off: Vec<_> = note_off_config.into_iter().collect();
    sorted_note_off.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:#?}", sorted_note_off);
    // let note_off_config: HashMap<u8, String> = sorted_note_off.into_iter().collect();
    // let mut cc_config: HashMap<u8, String> = HashMap::new();
    // let mut note_on_config: HashMap<u8, String> = HashMap::new();
    // let mut note_off_config: HashMap<u8, String> = HashMap::new();
    // for (key, cc) in sorted_cc {
    //     cc_config.insert(key, cc);
    // }
    println!("[midi_mapping_cc]");
    for (key, cc) in sorted_cc {
        // cc_config.insert(key, cc);
        println!("{} = \"{}\"", key, cc);
    }
    println!("[midi_mapping_note_on]");
    for (key, note_on) in sorted_note_on {
        // note_on_config.insert(key, note_on);
        println!("{} = \"{}\"", key, note_on);
    }
    println!("[midi_mapping_note_off]");
    for (key, note_off) in sorted_note_off {
        // note_off_config.insert(key, note_off);
        println!("{} = \"{}\"", key, note_off);
    }

    // println!("{:#?}", cc_config);
    // println!("{:#?}", note_on_config);
    // println!("{:#?}", note_off_config);
    // .for_each(|(key, cc)| println!("key: {}, cc: {:#?}", key, cc));
}
