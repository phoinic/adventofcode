static INPUT_DATA: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<bool> {
    input
        .chars()
        .into_iter()
        .map(|ch| match ch {
            '0' => [false, false, false, false],
            '1' => [false, false, false, true],
            '2' => [false, false, true, false],
            '3' => [false, false, true, true],
            '4' => [false, true, false, false],
            '5' => [false, true, false, true],
            '6' => [false, true, true, false],
            '7' => [false, true, true, true],
            '8' => [true, false, false, false],
            '9' => [true, false, false, true],
            'A' => [true, false, true, false],
            'B' => [true, false, true, true],
            'C' => [true, true, false, false],
            'D' => [true, true, false, true],
            'E' => [true, true, true, false],
            'F' => [true, true, true, true],
            _ => panic!("Wrong input"),
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn bits_to_u64(bits: &[bool]) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, &value)| if value { 1 << i } else { 0 })
        .sum::<u64>()
}

#[derive(Debug)]
struct Packet<'a> {
    bits: &'a [bool],
    type_id: u8,
    version: u8,
    value: Option<u64>,
    sub_packets: Vec<Self>,
}

impl<'a> Packet<'a> {
    pub fn from_bits(bits: &'a [bool], limit: Option<usize>, depth: usize) -> Vec<Self> {
        let mut packets = vec![];
        let mut pos = 0;

        while pos < bits.len() {
            if pos + 6 >= bits.len() {
                break;
            }

            let version = bits_to_u64(&bits[pos..pos + 3]) as u8;
            let type_id = bits_to_u64(&bits[pos + 3..pos + 6]) as u8;

            println!("pos = {}", pos);
            println!(
                "depth = {}, type_id = {}, version = {}",
                depth, type_id, version
            );

            pos += 6;

            if type_id == 4 {
                let mut value_bits = vec![];
                while pos < bits.len() {
                    if bits.len() - pos < 5 {
                        pos = bits.len();
                        break;
                    } else {
                        value_bits.extend_from_slice(&bits[pos + 1..pos + 5]);
                    }
                    pos += 5;
                    if !bits[pos - 5] {
                        break;
                    }
                }
                packets.push(Packet {
                    bits: &bits[..pos],
                    type_id,
                    version,
                    value: Some(bits_to_u64(value_bits.as_slice())),
                    sub_packets: vec![],
                });
            } else {
                if bits[pos] {
                    pos += 1;
                    let sub_count_len = 11;
                    let sub_count = bits_to_u64(&bits[pos..pos + sub_count_len]) as usize;
                    println!("sub_count = {}", sub_count);
                    pos += sub_count_len;
                    let sub_packets = Self::from_bits(&bits[pos..], Some(sub_count), depth + 1);
                    pos += sub_packets.iter().map(|p| p.bits.len()).sum::<usize>();
                    packets.push(Packet {
                        bits,
                        type_id,
                        version,
                        value: None,
                        sub_packets,
                    });
                    if limit.is_none() {
                        break;
                    }
                } else {
                    pos += 1;
                    let sub_size_len = 15;
                    let sub_size = bits_to_u64(&bits[pos..pos + sub_size_len]) as usize;
                    println!("sub_size = {}", sub_size);
                    pos += sub_size_len;
                    let sub_packets = Self::from_bits(&bits[pos..pos + sub_size], None, depth + 1);
                    pos += sub_size;
                    packets.push(Packet {
                        bits: &bits[..pos],
                        type_id,
                        version,
                        value: None,
                        sub_packets,
                    });
                    if limit.is_none() {
                        break;
                    }
                }
            }

            if limit.is_some() && packets.len() >= limit.unwrap() {
                break;
            }
        }

        packets
    }

    pub fn sum_versions(&self) -> u64 {
        let mut sum = self.version as u64;

        for packet in self.sub_packets.iter() {
            sum += packet.sum_versions();
        }

        sum
    }
}

fn main() {
    let input_bits = parse_input(INPUT_DATA);
    let packets = Packet::from_bits(input_bits.as_slice(), None, 0);

    println!("{:?}", input_bits);

    println!(
        "Round 1: {}",
        packets.iter().map(|p| p.sum_versions()).sum::<u64>()
    )
}
