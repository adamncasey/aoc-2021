
#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    value: Value,
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Literal(u32),
    Operator(Operator),
}

#[derive(Debug, PartialEq, Eq)]
struct Operator {
    typeid: u8,
    subpackets: Vec<Packet>
}

#[derive(Debug)]
struct BitVec {
    data: Vec<bool>,
}

impl BitVec {
    /// Read at most 8 bits
    fn read(&mut self, n: u8) -> u8 {
        if n > 8 {
            panic!("Asked for more than 8 bits");
        }

        let mut value = 0;
        for _ in 0..n {
            let bit = self.data.pop().unwrap();
            value = value << 1;
            value = value | if bit { 0b1 } else { 0b0 };
        }

        value
    }

    /// Read at most 16 bits
    fn read16 (&mut self, n: u8) -> u16 {
        if n > 16 {
            panic!("Asked for more than 16 bits");
        }

        if n < 8 {
            self.read(8) as u16
        } else {
            let remainder = n - 8;
            ((self.read(8) as u16) << remainder) | self.read(remainder) as u16
        }
    }
}

fn base64(ch: u8) -> u8 {
    match ch as char {
        '0' => 0b0000,
        '1' => 0b0001,
        '2' => 0b0010,
        '3' => 0b0011,
        '4' => 0b0100,
        '5' => 0b0101,
        '6' => 0b0110,
        '7' => 0b0111,
        '8' => 0b1000,
        '9' => 0b1001,
        'A' => 0b1010,
        'B' => 0b1011,
        'C' => 0b1100,
        'D' => 0b1101,
        'E' => 0b1110,
        'F' => 0b1111,
        _ => panic!("Bad base64 char {}", ch),
    }
}

fn base64_decode(input: &str) -> BitVec {
    let mut output = Vec::new();

    for chunk in input.as_bytes().chunks(2) {
        let (char1, char2) = (chunk[0], chunk[1]);

        let mut byte = ((base64(char1)) << 4) | base64(char2);

        for _ in 0..8 {
            output.push((byte & 0b10000000) != 0);
            byte = byte << 1;
        }
    }

    output.reverse();

    BitVec { data: output }
}

fn count_versions(packet: &Packet) -> usize {
    let version_count = packet.version as usize;

    match &packet.value {
        Value::Literal(_) => version_count,
        Value::Operator(Operator {_typeid, subpackets}) => {
            version_count + subpackets.iter().map(count_versions).sum::<usize>()
        }
    }
}

fn resolve_operator(op: &Operator) -> usize {
    match op.typeid {
        0 => op.subpackets.iter().map(resolve_packet).sum::<usize>(),
        1 => op.subpackets.iter().map(resolve_packet).fold(1, |acc, next| acc * next),
        2 => op.subpackets.iter().map(resolve_packet).min().unwrap(),
        3 => op.subpackets.iter().map(resolve_packet).max().unwrap(),
        5 => (resolve_packet(&op.subpackets[0]) > resolve_packet(&op.subpackets[1])) as usize,
        6 => (resolve_packet(&op.subpackets[0]) < resolve_packet(&op.subpackets[1])) as usize,
        7 => (resolve_packet(&op.subpackets[0]) == resolve_packet(&op.subpackets[1])) as usize,
        _ => panic!("At the disco"),
    }
}

fn resolve_packet(packet: &Packet) -> usize {
    match &packet.value {
        Value::Literal(x) => (*x) as usize,
        Value::Operator(op) => {
            resolve_operator(&op)
        }
    }
}

fn day16(packet: Packet) -> usize {
    count_versions(&packet)
}

fn day16_2(packet: Packet) -> usize {
    dbg!(&packet);
    resolve_packet(&packet)
}

fn read_literal(bits: &mut BitVec) -> Value {
    let mut num: u32 = 0;
    
    loop {
        let last = bits.read(1) == 0;

        num = num << 4;
        num = num | (bits.read(4) as u32);

        if last {
            break;
        }
    }

    Value::Literal(num)
}

fn read_operator(typeid: u8, bits: &mut BitVec) -> Value {
    let length_typeid = bits.read(1) != 0;

    let mut subpackets = Vec::new();
    if length_typeid {
        let num_packets = bits.read16(11);
        for _ in 0..num_packets {
            subpackets.push(read_packet(bits));
        }
    } else {
        let num_bits = bits.read16(15) as usize;
        let start_len = bits.data.len();
        dbg!((num_bits, start_len));

        while (start_len - bits.data.len()) < num_bits {
            
            dbg!((subpackets.len(), start_len - bits.data.len(), num_bits));
            subpackets.push(read_packet(bits));
        }
    }

    Value::Operator(Operator {typeid, subpackets})
}

fn read_packet(bits: &mut BitVec) -> Packet {
    let version = bits.read(3);
    let typeid = bits.read(3);

    dbg!((version, typeid));

    let value = match typeid {
        4 => read_literal(bits),
        _ => read_operator(typeid, bits),
    };

    Packet { version, value}
}

fn read_input(input: &str) -> Packet {
    let mut decoded = base64_decode(input);

    read_packet(&mut decoded)
}

#[test]
fn day16_input_literal() {
    assert_eq!(read_input("D2FE28"), Packet { version: 6, value: Value::Literal(2021)});
}

#[test]
fn day16_input_operator() {
    assert_eq!(read_input("38006F45291200"), Packet { version: 1, value: Value::Operator(Operator {
        typeid: 6,
        subpackets: vec![Packet { version: 6, value: Value::Literal(10)}, Packet { version: 2, value: Value::Literal(20)}]
    })});
}

#[test]
fn day16_bitvec_read16() {
    let mut x = BitVec { data: vec![true, true, false, true, false, true, false, true, false]};

    assert_eq!(x.read16(9), 171);
}

#[test]
fn day16_example1() {
    let input = read_input("8A004A801A8002F478");
    assert_eq!(day16(input), 16);

    let input = read_input("620080001611562C8802118E34");
    assert_eq!(day16(input), 12);

    let input = read_input("C0015000016115A2E0802F182340");
    assert_eq!(day16(input), 23);

    let input = read_input("A0016C880162017C3686B18A3D4780");
    assert_eq!(day16(input), 31);
}

#[test]
fn day16_actual() {
    let input = std::fs::read_to_string("./input/day16.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day16(input), 1615);
}

#[test]
fn day16_2_example1() {
    let input = read_input("C200B40A82");
    assert_eq!(day16_2(input), 3);

    let input = read_input("04005AC33890");
    assert_eq!(day16_2(input), 54);

    let input = read_input("880086C3E88112");
    assert_eq!(day16_2(input), 7);

    let input = read_input("CE00C43D881120");
    assert_eq!(day16_2(input), 9);

    let input = read_input("D8005AC2A8F0");
    assert_eq!(day16_2(input), 1);

    let input = read_input("F600BC2D8F");
    assert_eq!(day16_2(input), 0);

    let input = read_input("9C005AC2F8F0");
    assert_eq!(day16_2(input), 0);

    let input = read_input("9C0141080250320F1802104A08");
    assert_eq!(day16_2(input), 1);
}


#[test]
fn day16_2_actual() {
    let input = std::fs::read_to_string("./input/day16.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day16_2(input), 1615);
}
