use criterion::{black_box, criterion_group, criterion_main, Criterion};
use motuman::motu::{channel::Channel, json_payload};
use motuman::motu::{channel, MotuCommand};

fn bench_osc_command(c: &mut Criterion) {
    let motu_commands: Vec<MotuCommand> = {
        let mut commands = Vec::new();
        for i in 0..1 {
            commands.push(MotuCommand::Volume {
                channel: Channel::new(i, channel::ChannelType::Chan),
                volume: 0.5,
            });
        }
        commands
    };
    c.bench_function("motu_osc_command", |b| {
        b.iter(|| {
            black_box(
                motu_commands
                    .clone()
                    .into_iter()
                    .filter(|cmd| cmd.osc_command().is_some())
                    .for_each(|cmd| {
                        cmd.osc_command().unwrap();
                    }),
            )
        });
    });
}
fn bench_midi_command(c: &mut Criterion) {
    let motu_commands: Vec<MotuCommand> = {
        let mut commands = Vec::new();
        for i in 0..1 {
            commands.push(MotuCommand::Volume {
                channel: Channel::new(i, channel::ChannelType::Chan),
                volume: 0.5,
            });
        }
        commands
    };
    c.bench_function("bench_hash_map_command", |b| {
        b.iter(|| {
            black_box(
                motu_commands
                    .clone()
                    .into_iter()
                    .filter(|cmd| cmd.hash_map().is_some())
                    .for_each(|cmd| {
                        cmd.hash_map().unwrap();
                    }),
            )
        });
    });
}
fn create_json_payload_realistic() {
    let mut commands = Vec::new();
    for i in 0..40 {
        commands.push(MotuCommand::Volume {
            channel: Channel::new(i, channel::ChannelType::Chan),
            volume: 0.5,
        });
    }
    json_payload(&commands);
}
fn create_json_payload() {
    let mut commands = Vec::new();
    for i in 0..1000 {
        commands.push(MotuCommand::Volume {
            channel: Channel::new(i, channel::ChannelType::Chan),
            volume: 0.5,
        });
    }
    json_payload(&commands);
}
fn create_json_payload_vec() {
    let mut commands = Vec::new();
    for i in 0..1000 {
        commands.push(MotuCommand::Volume {
            channel: Channel::new(i, channel::ChannelType::Chan),
            volume: 0.5,
        });
    }
    let mut pairs: Vec<String> = Vec::new();

    for command in commands {
        if let Some((k, v)) = command.osc_command() {
            let formatted_key = k.replace("/mix/", "mix/");
            pairs.push(format!("\"{}\": {}", formatted_key, v));
        }
    }

    let s = format!("{{{}}}", pairs.join(", "));
}
fn bench_json_payload(c: &mut Criterion) {
    c.bench_function("bench_json_payload", |b| {
        b.iter(|| {
            black_box(create_json_payload());
        });
    });
    c.bench_function("bench_json_payload_realistic", |b| {
        b.iter(|| {
            black_box(create_json_payload_realistic());
        });
    });
    c.bench_function("bench_json_payload_vec", |b| {
        b.iter(|| {
            black_box(create_json_payload_vec());
        });
    });
}

criterion_group!(
    benches,
    bench_midi_command,
    bench_osc_command,
    bench_json_payload
);
criterion_main!(benches);
