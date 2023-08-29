use criterion::{black_box, criterion_group, criterion_main, Criterion};
use motuman::motu::channel::Channel;
use motuman::motu::{channel, MotuCommand};

fn bench_endpoint_command(c: &mut Criterion) {
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
    c.bench_function("motu_command", |b| {
        b.iter(|| {
            black_box(
                motu_commands
                    .clone()
                    .into_iter()
                    .filter(|cmd| cmd.endpoint_command().is_some())
                    .for_each(|cmd| {
                        cmd.endpoint_command().unwrap();
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
    c.bench_function("motu_command", |b| {
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

criterion_group!(benches, bench_midi_command, bench_endpoint_command);
criterion_main!(benches);
