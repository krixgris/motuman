use std::fmt::Display;

#[derive(Debug)]
pub enum MidiType {
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
