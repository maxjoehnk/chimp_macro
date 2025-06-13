use crate::chimp::*;
use crate::osc::OscConnection;
use rosc::{OscPacket, OscType};
use std::io;
use std::ops::{Range, RangeInclusive};
use std::time::Duration;

mod chimp;
mod osc;

fn main() {
    let connection = OscConnection::new("192.168.1.163".parse().unwrap());
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let mut buffer = [0; 1024];

            loop {
                match connection.socket.recv(&mut buffer) {
                    Ok(count) => {
                        let mut bytes = &buffer[..count];
                        while !bytes.is_empty() {
                            match rosc::decoder::decode_udp(bytes) {
                                Ok((remaining, packet)) => {
                                    handle_osc_packet(packet);
                                    bytes = remaining;
                                }
                                Err(err) => {
                                    // eprintln!("Error decoding OSC packet: {:?}", err);
                                    break;
                                }
                            }
                        }
                    }
                    Err(err) if err.kind() == io::ErrorKind::TimedOut => {}
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
                    Err(err) => eprintln!("{err:?}"),
                }
            }
        });

        scope.spawn(|| {
            loop {
                connection.ping();
                std::thread::sleep(Duration::from_secs(10));
            }
        });

        connection.sync();
        connection.clear();
        connection.clear();
        connection.clear();
        // connection.delete(Key::CueList, 1..1024);

        let mut start_cue_list_id = 601;
        let groups = [3, 4, 6, 7, 8];
        let mut start_cue_list_id = 617;
        let groups = [4];
        for group in groups {
            record_color_cuelists(&connection, group, 1..13, start_cue_list_id);
            start_cue_list_id += 16;
        }

        // let cue_list_id = 585;
        // let preset_count = 13;
        // let group_count = 7;

        // let mut buf = String::new();

        // for i in 0..preset_count {
        //     connection.press_key(Key::Name);
        //     connection.press_key(Key::Name);
        //     connection.press_key(Key::CueList);
        //     for g in 0..group_count {
        //         connection.send_number(cue_list_id + i + (g * 16));
        //         connection.press_key(Key::Plus);
        //     }
        //     connection.press_key(Key::Backspace);
        //     connection.enter();

        //     if let Err(err) = std::io::stdin().read_line(&mut buf) {
        //         panic!("Error reading input: {}", err);
        //     }
        // }

        // record_pixel_groups(&connection, 9, 101..108, 2..8);
        // record_pixel_groups(&connection, 17, 301..308, 1..4);
        // record_pixel_groups(&connection, 25, 601..604, 1..10);

        // connection.select_group(1);
        // connection.select_preset(PresetType::Color, 10);
        // connection.send_key(Key::Group);
        // connection.send_key(Key::Number(1));
        // connection.send_key(Key::Enter);
        // connection.press_key(Key::Preset);
        // connection.press_key(Key::Color);
        // connection.press_key(Key::Number(1));
        // connection.press_key(Key::Enter);
        // connection.press_key(Key::Record);
        // connection.press_key(Key::CueList);
        // connection.press_key(Key::Number(1));

        std::process::exit(0);
    });
}

fn record_color_cuelists(
    conn: &dyn ChimpConnection,
    group: u16,
    presets: Range<u16>,
    cuelist_start: u16,
) {
    for (i, preset) in (presets.start..=presets.end).enumerate() {
        let cuelist_id = cuelist_start + i as u16;
        conn.select_group(group);
        conn.select_preset(PresetType::Color, preset);
        conn.record(Key::CueList, cuelist_id, RecordMode::Replace);
        conn.enter();
        conn.clear();
    }
}

fn record_pixel_groups(
    conn: &dyn ChimpConnection,
    mut group_id: u16,
    fixtures: Range<u16>,
    instances: Range<u16>,
) {
    let count = fixtures.end - fixtures.start;
    conn.delete(Key::Group, group_id..(group_id + count));
    for i in fixtures.start..=fixtures.end {
        conn.send_number(i);
        conn.press_key(Key::Dot);
        conn.send_number(instances.start);
        conn.press_key(Key::Thru);
        conn.send_number(i);
        conn.press_key(Key::Dot);
        conn.send_number(instances.end);
        conn.enter();
        conn.record(Key::Group, group_id, RecordMode::Replace);
        group_id += 1;
    }
}

fn handle_osc_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            if msg.addr == "/chimp/programmer/commandline/content" {
                if let OscType::String(ref content) = msg.args[0] {
                    println!("CMD> {}", content);
                }
            }
        }
        OscPacket::Bundle(bundle) => {
            for content in bundle.content {
                handle_osc_packet(content);
            }
        }
    }
}
