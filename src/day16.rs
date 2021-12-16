use std::ops::Deref;

use anyhow::{Context, Result};
use bitvec::prelude::*;

pub fn run() -> Result<()> {
    let content = std::fs::read_to_string("inputs/day16.txt")?;

    let mut bv = parse_hex_data(&content)?;
    let packet = parse_packet(&mut bv);

    let sum = packet.version_sum();
    println!("day16 part1 = {}", sum);

    let eval = packet.eval();
    println!("day16 part2 = {}", eval);
    Ok(())
}

fn parse_hex_data(data: &str) -> Result<BitBuffer> {
    let bytes = data
        .trim()
        .as_bytes()
        .chunks(2)
        .map(|src| {
            u8::from_str_radix(String::from_utf8_lossy(src).as_ref(), 16).context("parsing error")
        })
        .collect::<Result<Vec<_>>>()?;
    let bv = BitVec::<Msb0, _>::from_slice(&bytes).context("Failed to build BitVec")?;

    Ok(BitBuffer(bv))
}

fn parse_header(data: &mut BitBuffer) -> Header {
    let header = data.take(6);
    let version: u8 = header[0..3].load_be();
    let type_id: u8 = header[3..6].load_be();

    Header::new(version, type_id)
}

fn parse_packet(data: &mut BitBuffer) -> Packet {
    let header = parse_header(data);

    let payload = match header.type_id {
        4 => {
            let mut literal = BitVec::<Msb0, u8>::new();
            loop {
                let chunk = data.take(5);
                literal.extend_from_bitslice(&chunk[1..]);
                if !chunk[0] {
                    break;
                }
            }
            PacketData::Literal(literal[..].load_be::<u64>())
        }
        _n => {
            let length_type_id = data.take(1)[0];
            let mut packets = Vec::new();
            if length_type_id {
                let num_packets_bits = data.take(11);
                let num_packets: usize = num_packets_bits.load_be();
                for _ in 0..num_packets {
                    let packet = parse_packet(data);
                    packets.push(packet);
                }
            } else {
                let len_bits = data.take(15);
                let len: usize = len_bits.load_be();
                let mut packets_data = data.take(len);
                packets.append(&mut parse_packets(&mut packets_data));
            }
            PacketData::Operator(packets)
        }
    };

    Packet {
        header,
        data: payload,
    }
}

fn parse_packets(data: &mut BitBuffer) -> Vec<Packet> {
    let mut packets = Vec::new();
    loop {
        let packet = parse_packet(data);
        packets.push(packet);
        if data.is_empty() {
            break;
        }
    }

    packets
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Header {
    version: u8,
    type_id: u8,
}

impl Header {
    fn new(version: u8, type_id: u8) -> Self {
        Self { version, type_id }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    header: Header,
    data: PacketData,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match self.data {
            PacketData::Literal(_) => self.header.version as u64,
            PacketData::Operator(ref packets) => {
                let sum: u64 = packets.iter().map(|p| p.version_sum()).sum();
                self.header.version as u64 + sum
            }
        }
    }

    fn eval(&self) -> u64 {
        match self.data {
            PacketData::Literal(n) => n,
            PacketData::Operator(ref packets) => match self.header.type_id {
                0 => packets.iter().map(|p| p.eval()).sum(),
                1 => packets.iter().map(|p| p.eval()).product(),
                2 => packets.iter().map(|p| p.eval()).min().unwrap(),
                3 => packets.iter().map(|p| p.eval()).max().unwrap(),
                5 => (packets[0].eval() > packets[1].eval()) as u64,
                6 => (packets[0].eval() < packets[1].eval()) as u64,
                7 => (packets[0].eval() == packets[1].eval()) as u64,
                _ => panic!("Unknown op code"),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PacketData {
    Literal(u64),
    Operator(Vec<Packet>),
}

struct BitBuffer(BitVec<Msb0, u8>);

impl BitBuffer {
    fn take(&mut self, count: usize) -> BitBuffer {
        let mut rest = self.0.split_off(count);
        std::mem::swap(&mut self.0, &mut rest);

        Self(rest)
    }
}

impl Deref for BitBuffer {
    type Target = BitSlice<Msb0, u8>;

    fn deref(&self) -> &BitSlice<Msb0, u8> {
        self.0.deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let content = "D2FE28";
        let mut bv = parse_hex_data(content).unwrap();
        let packet = parse_packet(&mut bv);
        assert_eq!(
            Packet {
                header: Header::new(6, 4),
                data: PacketData::Literal(2021)
            },
            packet
        );
    }

    #[test]
    fn test_operator1() {
        let content = "38006F45291200";
        let mut bv = parse_hex_data(content).unwrap();
        let packet = parse_packet(&mut bv);

        dbg!(&packet);
        assert_eq!(Header::new(1, 6), packet.header);
        assert!(matches!(packet.data, PacketData::Operator(packets) if packets.len() == 2));
    }

    #[test]
    fn test_operator2() {
        let content = "EE00D40C823060";
        let mut bv = parse_hex_data(content).unwrap();
        let packet = parse_packet(&mut bv);

        dbg!(&packet);
        assert_eq!(Header::new(7, 3), packet.header);
        assert!(matches!(packet.data, PacketData::Operator(packets) if packets.len() == 3));
    }
}
