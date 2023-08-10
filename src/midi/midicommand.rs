use crate::motu::MotuCommand;

#[derive(Debug)]
pub struct MidiCommand {
    // message field should be an array of 3 u8
    pub message: [u8; 3],
    midi_value: u8,
    prev_midi_value: u8,
    pub motu_command: MotuCommand,
    timestamp: u64,
    prev_timestamp: u64,
}
impl MidiCommand {
    pub fn new(message: &[u8], motu_command: MotuCommand) -> Option<Self> {
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

    pub fn delta_value(&self) -> f32 {
        // abs delta value
        (self.midi_value as f32 - self.prev_midi_value as f32).abs()
    }

    pub fn delta_time(&self) -> u64 {
        self.timestamp - self.prev_timestamp
    }

    /// Determines whether the MIDI command should be throttled based on the delta time and delta value.
    /// Returns `true` if the command should be throttled, `false` otherwise.
    pub fn do_throttle(&mut self) -> bool {
        let delta_time = self.delta_time();
        let delta_value = {
            if delta_time > 1000 {
                1000.0
            } else {
                self.delta_value()
            }
        };

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
        self.motu_command
            .set_value(easing_circ(midi_value as f32 / 127.0));
        Ok(())
    }
}

trait EasingAlgorithm {
    fn easing(x: f32) -> f32;
}

fn easing_circ(x: f32) -> f32 {
    1.0 - (1.0 - x * x).sqrt()
}
