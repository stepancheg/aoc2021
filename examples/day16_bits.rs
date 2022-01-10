use std::fmt;

const INPUTS_PART_1: &[&str] = &[
    "38006F45291200",
    "8A004A801A8002F478",
    "620080001611562C8802118E34",
    "C0015000016115A2E0802F182340",
    "A0016C880162017C3686B18A3D4780",
    "220D790065B2745FF004672D99A34E5B33439D96CEC80373C0068663101A98C406A5E7395DC1804678BF25A4093BFBDB886CA6E11FDE6D93D16A100325E5597A118F6640600ACF7274E6A5829B00526C167F9C089F15973C4002AA4B22E800FDCFD72B9351359601300424B8C9A00BCBC8EE069802D2D0B945002AB2D7D583E3F00016B05E0E9802BA00B4F29CD4E961491CCB44C6008E80273C393C333F92020134B003530004221347F83A200D47F89913A66FB6620016E24A007853BE5E944297AB64E66D6669FCEA0112AE06009CAA57006A0200EC258FB0440010A8A716A321009DE200D44C8E31F00010887B146188803317A3FC5F30056C0150004321244E88C000874468A91D2291802B25EB875802B28D13550030056C0169FB5B7ECE2C6B2EF3296D6FD5F54858015B8D730BB24E32569049009BF801980803B05A3B41F1007625C1C821256D7C848025DE0040E5016717247E18001BAC37930E9FA6AE3B358B5D4A7A6EA200D4E463EA364EDE9F852FF1B9C8731869300BE684649F6446E584E61DE61CD4021998DB4C334E72B78BA49C126722B4E009C6295F879002093EF32A64C018ECDFAF605989D4BA7B396D9B0C200C9F0017C98C72FD2C8932B7EE0EA6ADB0F1006C8010E89B15A2A90021713610C202004263E46D82AC06498017C6E007901542C04F9A0128880449A8014403AA38014C030B08012C0269A8018E007A801620058003C64009810010722EC8010ECFFF9AAC32373F6583007A48CA587E55367227A40118C2AC004AE79FE77E28C007F4E42500D10096779D728EB1066B57F698C802139708B004A5C5E5C44C01698D490E800B584F09C8049593A6C66C017100721647E8E0200CC6985F11E634EA6008CB207002593785497652008065992443E7872714",
];

const INPUTS_PART_2: &[&str] = &[
    "C200B40A82",
    "04005AC33890",
    "880086C3E88112",
    "CE00C43D881120",
    "D8005AC2A8F0",
    "F600BC2D8F",
    "9C005AC2F8F0",
    "9C0141080250320F1802104A08",
    "220D790065B2745FF004672D99A34E5B33439D96CEC80373C0068663101A98C406A5E7395DC1804678BF25A4093BFBDB886CA6E11FDE6D93D16A100325E5597A118F6640600ACF7274E6A5829B00526C167F9C089F15973C4002AA4B22E800FDCFD72B9351359601300424B8C9A00BCBC8EE069802D2D0B945002AB2D7D583E3F00016B05E0E9802BA00B4F29CD4E961491CCB44C6008E80273C393C333F92020134B003530004221347F83A200D47F89913A66FB6620016E24A007853BE5E944297AB64E66D6669FCEA0112AE06009CAA57006A0200EC258FB0440010A8A716A321009DE200D44C8E31F00010887B146188803317A3FC5F30056C0150004321244E88C000874468A91D2291802B25EB875802B28D13550030056C0169FB5B7ECE2C6B2EF3296D6FD5F54858015B8D730BB24E32569049009BF801980803B05A3B41F1007625C1C821256D7C848025DE0040E5016717247E18001BAC37930E9FA6AE3B358B5D4A7A6EA200D4E463EA364EDE9F852FF1B9C8731869300BE684649F6446E584E61DE61CD4021998DB4C334E72B78BA49C126722B4E009C6295F879002093EF32A64C018ECDFAF605989D4BA7B396D9B0C200C9F0017C98C72FD2C8932B7EE0EA6ADB0F1006C8010E89B15A2A90021713610C202004263E46D82AC06498017C6E007901542C04F9A0128880449A8014403AA38014C030B08012C0269A8018E007A801620058003C64009810010722EC8010ECFFF9AAC32373F6583007A48CA587E55367227A40118C2AC004AE79FE77E28C007F4E42500D10096779D728EB1066B57F698C802139708B004A5C5E5C44C01698D490E800B584F09C8049593A6C66C017100721647E8E0200CC6985F11E634EA6008CB207002593785497652008065992443E7872714",
];

struct Bits {
    bits: Vec<bool>,
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for bit in self.bits.iter() {
            write!(f, "{}", if *bit { '1' } else { '0' })?;
        }
        Ok(())
    }
}

impl Bits {
    fn parse(input: &str) -> Bits {
        let mut bits = Vec::new();
        for c in input.chars() {
            let v = u8::from_str_radix(&c.to_string(), 16).unwrap();
            for i in (0..4).rev() {
                bits.push((v >> i) & 1 != 0);
            }
        }
        Bits { bits }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PacketContent {
    Literal(u64),
    Sub(Vec<Packet>),
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u32,
    type_id: TypeId,
    content: PacketContent,
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.version
            + match &self.content {
                PacketContent::Literal(_) => 0,
                PacketContent::Sub(packets) => packets.iter().map(|p| p.version_sum()).sum(),
            }
    }

    fn eval(&self) -> u64 {
        match &self.content {
            PacketContent::Literal(literal) => {
                assert_eq!(TypeId::Literal, self.type_id);
                *literal
            }
            PacketContent::Sub(packets) => match self.type_id {
                TypeId::Sum => {
                    assert!(!packets.is_empty());
                    packets.iter().map(|p| p.eval()).sum()
                }
                TypeId::Product => {
                    assert!(!packets.is_empty());
                    packets.iter().map(|p| p.eval()).product()
                }
                TypeId::Min => packets.iter().map(|p| p.eval()).min().unwrap(),
                TypeId::Max => packets.iter().map(|p| p.eval()).max().unwrap(),
                TypeId::Literal => panic!(),
                TypeId::Gt => match packets.as_slice() {
                    [p0, p1] => match p0.eval() > p1.eval() {
                        true => 1,
                        false => 0,
                    },
                    _ => panic!(),
                },
                TypeId::Lt => match packets.as_slice() {
                    [p0, p1] => match p0.eval() < p1.eval() {
                        true => 1,
                        false => 0,
                    },
                    _ => panic!(),
                },
                TypeId::Eq => match packets.as_slice() {
                    [p0, p1] => match p0.eval() == p1.eval() {
                        true => 1,
                        false => 0,
                    },
                    _ => panic!(),
                },
            },
        }
    }
}

enum LenTypeId {
    Bits,
    Packets,
}

#[derive(Debug, Eq, PartialEq)]
enum Len {
    Bits(u32),
    Packets(u32),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum TypeId {
    Sum,     // 0
    Product, // 1
    Min,     // 2
    Max,     // 3
    Literal, // 4
    Gt,      // 5
    Lt,      // 6
    Eq,      // 7
}

struct Parser<'a> {
    bits: &'a Bits,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn eof(&self) -> bool {
        self.pos == self.bits.bits.len()
    }

    fn next_bit(&mut self) -> bool {
        let bit = self.bits.bits[self.pos];
        self.pos += 1;
        bit
    }

    fn next_n_bits_be(&mut self, n: u32) -> u64 {
        let mut v = 0;
        for _ in 0..n {
            v <<= 1;
            v |= self.next_bit() as u64;
        }
        v
    }

    fn next_version(&mut self) -> u32 {
        self.next_n_bits_be(3) as u32
    }

    fn next_type_id(&mut self) -> TypeId {
        match self.next_n_bits_be(3) {
            0 => TypeId::Sum,
            1 => TypeId::Product,
            2 => TypeId::Min,
            3 => TypeId::Max,
            4 => TypeId::Literal,
            5 => TypeId::Gt,
            6 => TypeId::Lt,
            7 => TypeId::Eq,
            _ => panic!(),
        }
    }

    fn next_len_type_id(&mut self) -> LenTypeId {
        match self.next_bit() {
            false => LenTypeId::Bits,
            true => LenTypeId::Packets,
        }
    }

    fn next_len(&mut self) -> Len {
        match self.next_len_type_id() {
            LenTypeId::Bits => Len::Bits(self.next_n_bits_be(15) as u32),
            LenTypeId::Packets => Len::Packets(self.next_n_bits_be(11) as u32),
        }
    }

    fn next_bits(&mut self, count: u32) -> Bits {
        let mut bits = Vec::new();
        for _ in 0..count {
            bits.push(self.next_bit());
        }
        Bits { bits }
    }

    fn next_varint(&mut self) -> u64 {
        let mut r = 0;
        loop {
            let group = self.next_n_bits_be(5);
            let last = (group & 0b10000) == 0;
            let group = group & 0b01111;
            r = (r << 4) | group;
            if last {
                return r;
            }
        }
    }

    fn trailing_zeros(&mut self) {
        // assert!(self.bits.bits.len() - self.pos < 4);
        while !self.eof() {
            assert!(!self.next_bit());
        }
    }

    fn next_packet_content(&mut self, type_id: TypeId) -> PacketContent {
        if type_id == TypeId::Literal {
            let literal = self.next_varint();
            PacketContent::Literal(literal as u64)
        } else {
            let len = self.next_len();
            match len {
                Len::Bits(bits) => {
                    let bits = self.next_bits(bits);
                    let mut parser = Parser {
                        bits: &bits,
                        pos: 0,
                    };
                    let mut packets = Vec::new();
                    while !parser.eof() {
                        let packet = parser.next_packet();
                        packets.push(packet);
                    }
                    PacketContent::Sub(packets)
                }
                Len::Packets(packet_count) => {
                    let mut packets = Vec::new();
                    for _ in 0..packet_count {
                        let packet = self.next_packet();
                        packets.push(packet);
                    }
                    PacketContent::Sub(packets)
                }
            }
        }
    }

    fn next_packet(&mut self) -> Packet {
        let version = self.next_version();
        let type_id = self.next_type_id();
        let content = self.next_packet_content(type_id);
        Packet {
            version,
            type_id,
            content,
        }
    }

    fn parse_bits(bits: &Bits) -> Packet {
        let mut parser = Parser { bits, pos: 0 };
        let packet = parser.next_packet();
        parser.trailing_zeros();
        packet
    }
}

fn test1() {
    let bits = Bits::parse("D2FE28");
    assert_eq!("110100101111111000101000", bits.to_string());
    let packet = Parser::parse_bits(&bits);
    assert_eq!(
        Packet {
            version: 6,
            type_id: TypeId::Literal,
            content: PacketContent::Literal(2021),
        },
        packet
    );
}

fn test2() {
    let bits = Bits::parse("38006F45291200");
    assert_eq!(
        "00111000000000000110111101000101001010010001001000000000",
        bits.to_string()
    );
    let packet = Parser::parse_bits(&bits);
    assert_eq!(
        Packet {
            version: 1,
            type_id: TypeId::Lt,
            content: PacketContent::Sub(vec![
                Packet {
                    version: 6,
                    type_id: TypeId::Literal,
                    content: PacketContent::Literal(10),
                },
                Packet {
                    version: 2,
                    type_id: TypeId::Literal,
                    content: PacketContent::Literal(20),
                },
            ]),
        },
        packet
    );
}

fn test3() {
    let bits = Bits::parse("EE00D40C823060");
    assert_eq!(
        "11101110000000001101010000001100100000100011000001100000",
        bits.to_string()
    );
    let packet = Parser::parse_bits(&bits);
    assert_eq!(
        Packet {
            version: 7,
            type_id: TypeId::Max,
            content: PacketContent::Sub(vec![
                Packet {
                    version: 2,
                    type_id: TypeId::Literal,
                    content: PacketContent::Literal(1),
                },
                Packet {
                    version: 4,
                    type_id: TypeId::Literal,
                    content: PacketContent::Literal(2),
                },
                Packet {
                    version: 1,
                    type_id: TypeId::Literal,
                    content: PacketContent::Literal(3),
                },
            ]),
        },
        packet
    );
}

fn part1() {
    println!();
    println!("part1");
    for (i, input) in INPUTS_PART_1.iter().enumerate() {
        println!("{}", i);
        let bits = Bits::parse(input);
        let packet = Parser::parse_bits(&bits);
        println!("version_sum: {}", packet.version_sum());
        // println!("eval: {}", packet.eval());
    }
}

fn part2() {
    println!();
    println!("part2");
    for (i, input) in INPUTS_PART_2.iter().enumerate() {
        println!("{}", i);
        let bits = Bits::parse(input);
        let packet = Parser::parse_bits(&bits);
        // println!("version_sum: {}", packet.version_sum());
        // println!("{:?}", packet);
        println!("eval: {}", packet.eval());
    }
}

fn main() {
    println!("tests");
    test1();
    test2();
    test3();

    part1();
    part2();
}
