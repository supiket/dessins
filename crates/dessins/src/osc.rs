use crate::adjustable_variable::types::Context;
use nannou::prelude::*;
use nannou_osc::{receiver, rosc::OscPacket, Bundle, Message, Packet, Receiver};
use std::{collections::HashMap, net::SocketAddr};

const PORT: u16 = 34254;
const MAX_PACKETS: usize = 10;

pub struct Osc {
    receiver: Option<Receiver>,
    received_packets: Vec<(SocketAddr, Packet)>,
}

impl Osc {
    pub fn update(&mut self, ui: &mut egui::Ui) -> Context {
        let mut disconnect = false;
        let mut context = Context::new(Default::default());

        match &self.receiver {
            Some(receiver) => {
                if ui.button("disconnect").clicked() {
                    disconnect = true;
                }

                for (packet, addr) in receiver.try_iter() {
                    self.received_packets.push((addr, packet));
                }

                while self.received_packets.len() > MAX_PACKETS {
                    self.received_packets.remove(0);
                }

                // let mut packets_text = format!("listening on port {}\nreceived packets:\n", PORT);
                for &(_addr, ref packet) in self.received_packets.iter().rev() {
                    // packets_text.push_str(&format!("{}: {:?}\n", addr, packet));
                    *context = process_packet(packet);
                }

                // ui.label(packets_text);
            }
            None => {
                if ui.button(format!("connect (port {})", PORT)).clicked() {
                    self.receiver = receiver(PORT).ok();
                }
            }
        }

        if disconnect {
            self.receiver = None;
        }

        context
    }
}

fn process_packet(packet: &Packet) -> HashMap<String, f32> {
    let mut map = HashMap::new();
    if let Packet::Bundle(Bundle {
        timetag: _timetag,
        content,
    }) = packet
    {
        if let Some(packet) = content.first() {
            if let OscPacket::Message(Message { addr: _addr, args }) = packet {
                let mut prefix = "";

                for pair in args.chunks(2) {
                    if let (Some(nannou_osc::Type::String(key)), Some(value)) =
                        (pair.get(0), pair.get(1))
                    {
                        match value {
                            nannou_osc::Type::Double(d) => {
                                map.insert(key.clone(), *d as f32);
                            }
                            nannou_osc::Type::Float(f) => {
                                map.insert(key.clone(), *f);
                            }
                            nannou_osc::Type::String(s) => {
                                if key == "s" {
                                    prefix = s;
                                }
                            }
                            _ => continue,
                        };
                    }
                }

                let map = map
                    .into_iter()
                    .map(|(k, v)| (format!("{}{}", prefix, k), v))
                    .collect();

                return map;
            }
        }
    }

    map
}

impl Default for Osc {
    fn default() -> Self {
        Self {
            receiver: None,
            received_packets: vec![],
        }
    }
}
