use rosc::{OscPacket, OscType};
use std::io;
use std::ops::Range;
use std::time::Duration;
use crate::chimp::{ChimpConnection, Key, PresetType};
use crate::osc::OscConnection;

mod osc;
mod chimp;

fn main() {
    let connection = OscConnection::new("192.168.1.161".parse().unwrap());
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
        connection.delete(Key::CueList, 1..1024);
        std::thread::sleep(Duration::from_secs(1));
        record_color_cuelists(&connection, 1, 1..8, 500);
        // record_color_cuelists(&connection, 2, 1..8, 520);
        // record_color_cuelists(&connection, 2, 1..8, 540);
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
    });
}

fn record_color_cuelists(conn: &dyn ChimpConnection, group: u16, presets: Range<u16>, cuelist_start: u16) {
    for (i, preset) in presets.enumerate() {
        let cuelist_id = cuelist_start + i as u16;
        conn.select_group(group);
        conn.select_preset(PresetType::Color, preset);
        conn.press_key(Key::Record);
        conn.press_key(Key::CueList);
        conn.send_number(cuelist_id);
        conn.press_key(Key::Enter);
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
