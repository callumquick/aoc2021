/// Solution to Advent of Code Challenge Day 16.
use aoc2021::{get_day_input, print_elapsed_time};

const DAY: &str = "16";

#[derive(Debug, Clone)]
enum LengthMode {
    Length,
    Count,
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    type_: u8,
    length_mode: Option<LengthMode>,
    packets: Option<Vec<Packet>>,
    literal: Option<u64>,
}

fn read_str<'a>(input: &'a str, upto: &mut usize, len: usize) -> &'a str {
    if *upto + len > input.len() {
        panic!("Exceeded bounds");
    }
    let read = &input[*upto..(*upto + len)];
    *upto += len;
    read
}

fn parse_packet(input: &str, upto: &mut usize) -> Packet {
    let mut packet = Packet {
        version: 0,
        type_: 0,
        length_mode: None,
        packets: None,
        literal: None,
    };

    let ver_str = read_str(input, upto, 3);
    let type_str = read_str(input, upto, 3);
    packet.version = u8::from_str_radix(ver_str, 2).unwrap();
    packet.type_ = u8::from_str_radix(type_str, 2).unwrap();

    match packet.type_ {
        4 => {
            let mut cont = true;
            let mut literal_str = String::new();

            while cont {
                if read_str(input, upto, 1) == "0" {
                    cont = false;
                }

                literal_str.push_str(read_str(input, upto, 4));
            }

            packet.literal = Some(u64::from_str_radix(&literal_str, 2).unwrap());
        }
        _ => {
            let mut subpackets = Vec::new();
            let len_id_str = read_str(input, upto, 1);
            packet.length_mode = Some(match len_id_str {
                "0" => LengthMode::Length,
                "1" => LengthMode::Count,
                _ => panic!(),
            });

            match packet.length_mode {
                Some(LengthMode::Length) => {
                    let len_str = read_str(input, upto, 15);
                    let len = u16::from_str_radix(len_str, 2).unwrap();
                    let max_upto = *upto + len as usize;

                    while *upto < max_upto {
                        subpackets.push(parse_packet(input, upto));
                    }

                    // Assert there was a whole number of packets in the length given.
                    assert!(max_upto == *upto);
                }
                Some(LengthMode::Count) => {
                    let count_str = read_str(input, upto, 11);
                    let count = u16::from_str_radix(count_str, 2).unwrap();

                    for _ in 0..count {
                        subpackets.push(parse_packet(input, upto));
                    }
                }
                _ => panic!(),
            }

            packet.packets = Some(subpackets);
        }
    }

    packet
}

fn parse_hexadecimal(input: &str) -> String {
    let mut bin_str = String::new();
    for ch in input.chars() {
        bin_str.push_str(match ch {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Non upper hex character given"),
        })
    }
    bin_str
}

fn packet_from(input: &str) -> Packet {
    let mut upto = 0;
    parse_packet(&parse_hexadecimal(input), &mut upto)
}

fn sum_version(input: &Packet) -> u64 {
    let mut sum = input.version as u64;
    if let Some(packets) = &input.packets {
        for packet in packets {
            sum += sum_version(packet);
        }
    }
    sum
}

fn calculate_packet(input: &Packet) -> u64 {
    if input.type_ == 4 {
        return input.literal.unwrap();
    }

    let mut subpackets = input.packets.as_ref().unwrap().iter().map(calculate_packet);
    match input.type_ {
        0 => subpackets.sum(),
        1 => subpackets.product(),
        2 => subpackets.min().unwrap(),
        3 => subpackets.max().unwrap(),
        5 => (subpackets.next().unwrap() > subpackets.next().unwrap()) as u64,
        6 => (subpackets.next().unwrap() < subpackets.next().unwrap()) as u64,
        7 => (subpackets.next().unwrap() == subpackets.next().unwrap()) as u64,
        _ => panic!("Unsupported packet type: {}", input.type_),
    }
}

fn part_one(input: &Packet) -> u64 {
    sum_version(input)
}

fn part_two(input: &Packet) -> u64 {
    calculate_packet(input)
}

fn main() {
    let input = get_day_input(DAY);
    let inputs = packet_from(&input);
    println!("Day {}:", DAY);
    println!("==========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&inputs)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let test1 = packet_from("D2FE28");
        let test2 = packet_from("38006F45291200");
        let test3 = packet_from("EE00D40C823060");
        let test4 = packet_from("8A004A801A8002F478");
        let test5 = packet_from("620080001611562C8802118E34");
        let test6 = packet_from("C0015000016115A2E0802F182340");
        let test7 = packet_from("A0016C880162017C3686B18A3D4780");

        // Check each gives the right answer.
        assert_eq!(part_one(&test1), 6);
        assert_eq!(part_one(&test2), 9);
        assert_eq!(part_one(&test3), 14);
        assert_eq!(part_one(&test4), 16);
        assert_eq!(part_one(&test5), 12);
        assert_eq!(part_one(&test6), 23);
        assert_eq!(part_one(&test7), 31);
    }

    #[test]
    fn test_part_two_example() {
        let test1 = packet_from("C200B40A82");
        let test2 = packet_from("04005AC33890");
        let test3 = packet_from("880086C3E88112");
        let test4 = packet_from("CE00C43D881120");
        let test5 = packet_from("D8005AC2A8F0");
        let test6 = packet_from("F600BC2D8F");
        let test7 = packet_from("9C005AC2F8F0");
        let test8 = packet_from("9C0141080250320F1802104A08");

        // Check each gives the right answer.
        assert_eq!(part_two(&test1), 3);
        assert_eq!(part_two(&test2), 54);
        assert_eq!(part_two(&test3), 7);
        assert_eq!(part_two(&test4), 9);
        assert_eq!(part_two(&test5), 1);
        assert_eq!(part_two(&test6), 0);
        assert_eq!(part_two(&test7), 0);
        assert_eq!(part_two(&test8), 1);
    }
}
