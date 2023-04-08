use crate::bus::BUS;

enum FLAGS6502{
    C = (1 << 0), //C Carry flag 0 false 1 true
    Z = (1 << 1), //Z Zero flag 0 result not zero 1 result zero
    I = (1 << 2), //I IRQ disable flag 0 enable 1 disable
    D = (1 << 3), //D Decimal Mode Flag 0 false 1 true
    B = (1 << 4), //B Break Command Flag 0 No break 1 Break
    U = (1 << 5), //U Unused
    V = (1 << 6), //V Overflow flag 0 false 1 true
    N = (1 << 7), //N Negative flag 0 positive 1 negative
}

struct INSTRUCTION{
    name:String,
    operate:fn()->u8,
    addrmode:fn()->u8,
    cycles:u8,
}

pub struct OLC6502{
   
    pc: u16, //accumulator
    sp: u8, //stack pointer

    //8bit registers
    a: u8,
    x: u8,
    y: u8,

    status: u8, //NV_BDIZC

    fetched: u8,

    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,

    lookup:Vec<INSTRUCTION>,

    bus:BUS,
}
#[allow(non_snake_case)]
impl OLC6502{
    pub fn new ()->OLC6502{
        OLC6502{
            pc :0x0000,
            sp : 0x00,
            a: 0,
            x: 0,
            y: 0,
            status: 0b0000_0000,

            fetched: 0x00,
            
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0, 


            bus:BUS::new(),

            //lookup : vec![INSTRUCTION{name:String::from("BRK"), operate: OLC6502::BRK, addrmode:OLC6502::IMM, cycles:7}]

            lookup: vec![
                INSTRUCTION{name:String::from("BRK"), operate: OLC6502::BRK, addrmode: OLC6502::IMM, cycles:7 },INSTRUCTION{name:String::from("ORA"), operate: OLC6502::ORA, addrmode: OLC6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:3 },INSTRUCTION{name:String::from("ORA"), operate: OLC6502::ORA, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("ASL"), operate: OLC6502::ASL, addrmode: OLC6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("PHP"), operate: OLC6502::PHP, addrmode: OLC6502::IMP, cycles:3 },INSTRUCTION{name:String::from("ORA"), operate: OLC6502::ORA, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("ASL"), operate: OLC6502::ASL, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ORA"), operate: OLC6502::ORA, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("ASL"), operate: OLC6502::ASL, addrmode: OLC6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BPL"), operate: OLC6502::BPL, addrmode: OLC6502::REL, cycles:2 },INSTRUCTION{name:String::from("ORA"), operate: OLC6502::ORA, addrmode: OLC6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ORA"), operate: OLC6502::ORA, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("ASL"), operate: OLC6502::ASL, addrmode: OLC6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("CLC"), operate: OLC6502::CLC, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("ORA"), operate: OLC6502::ORA, addrmode: OLC6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ORA"), operate: OLC6502::ORA, addrmode: OLC6502::ABX, cycles:4 },INSTRUCTION{name:String::from("ASL"), operate: OLC6502::ASL, addrmode: OLC6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("JSR"), operate: OLC6502::JSR, addrmode: OLC6502::ABS, cycles:6 },INSTRUCTION{name:String::from("AND"), operate: OLC6502::AND, addrmode: OLC6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("BIT"), operate: OLC6502::BIT, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("AND"), operate: OLC6502::AND, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("ROL"), operate: OLC6502::ROL, addrmode: OLC6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("PLP"), operate: OLC6502::PLP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("AND"), operate: OLC6502::AND, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("ROL"), operate: OLC6502::ROL, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("BIT"), operate: OLC6502::BIT, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("AND"), operate: OLC6502::AND, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("ROL"), operate: OLC6502::ROL, addrmode: OLC6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BMI"), operate: OLC6502::BMI, addrmode: OLC6502::REL, cycles:2 },INSTRUCTION{name:String::from("AND"), operate: OLC6502::AND, addrmode: OLC6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("AND"), operate: OLC6502::AND, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("ROL"), operate: OLC6502::ROL, addrmode: OLC6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("SEC"), operate: OLC6502::SEC, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("AND"), operate: OLC6502::AND, addrmode: OLC6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("AND"), operate: OLC6502::AND, addrmode: OLC6502::ABX, cycles:4 },INSTRUCTION{name:String::from("ROL"), operate: OLC6502::ROL, addrmode: OLC6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("RTI"), operate: OLC6502::RTI, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("EOR"), operate: OLC6502::EOR, addrmode: OLC6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:3 },INSTRUCTION{name:String::from("EOR"), operate: OLC6502::EOR, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("LSR"), operate: OLC6502::LSR, addrmode: OLC6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("PHA"), operate: OLC6502::PHA, addrmode: OLC6502::IMP, cycles:3 },INSTRUCTION{name:String::from("EOR"), operate: OLC6502::EOR, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("LSR"), operate: OLC6502::LSR, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("JMP"), operate: OLC6502::JMP, addrmode: OLC6502::ABS, cycles:3 },INSTRUCTION{name:String::from("EOR"), operate: OLC6502::EOR, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("LSR"), operate: OLC6502::LSR, addrmode: OLC6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BVC"), operate: OLC6502::BVC, addrmode: OLC6502::REL, cycles:2 },INSTRUCTION{name:String::from("EOR"), operate: OLC6502::EOR, addrmode: OLC6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("EOR"), operate: OLC6502::EOR, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("LSR"), operate: OLC6502::LSR, addrmode: OLC6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("CLI"), operate: OLC6502::CLI, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("EOR"), operate: OLC6502::EOR, addrmode: OLC6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("EOR"), operate: OLC6502::EOR, addrmode: OLC6502::ABX, cycles:4 },INSTRUCTION{name:String::from("LSR"), operate: OLC6502::LSR, addrmode: OLC6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("RTS"), operate: OLC6502::RTS, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("ADC"), operate: OLC6502::ADC, addrmode: OLC6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:3 },INSTRUCTION{name:String::from("ADC"), operate: OLC6502::ADC, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("ROR"), operate: OLC6502::ROR, addrmode: OLC6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("PLA"), operate: OLC6502::PLA, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ADC"), operate: OLC6502::ADC, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("ROR"), operate: OLC6502::ROR, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("JMP"), operate: OLC6502::JMP, addrmode: OLC6502::IND, cycles:5 },INSTRUCTION{name:String::from("ADC"), operate: OLC6502::ADC, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("ROR"), operate: OLC6502::ROR, addrmode: OLC6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BVS"), operate: OLC6502::BVS, addrmode: OLC6502::REL, cycles:2 },INSTRUCTION{name:String::from("ADC"), operate: OLC6502::ADC, addrmode: OLC6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ADC"), operate: OLC6502::ADC, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("ROR"), operate: OLC6502::ROR, addrmode: OLC6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("SEI"), operate: OLC6502::SEI, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("ADC"), operate: OLC6502::ADC, addrmode: OLC6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ADC"), operate: OLC6502::ADC, addrmode: OLC6502::ABX, cycles:4 },INSTRUCTION{name:String::from("ROR"), operate: OLC6502::ROR, addrmode: OLC6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("STA"), operate: OLC6502::STA, addrmode: OLC6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("STY"), operate: OLC6502::STY, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("STA"), operate: OLC6502::STA, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("STX"), operate: OLC6502::STX, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:3 },INSTRUCTION{name:String::from("DEY"), operate: OLC6502::DEY, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("TXA"), operate: OLC6502::TXA, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("STY"), operate: OLC6502::STY, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("STA"), operate: OLC6502::STA, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("STX"), operate: OLC6502::STX, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:4 },
                INSTRUCTION{name:String::from("BCC"), operate: OLC6502::BCC, addrmode: OLC6502::REL, cycles:2 },INSTRUCTION{name:String::from("STA"), operate: OLC6502::STA, addrmode: OLC6502::IZY, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("STY"), operate: OLC6502::STY, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("STA"), operate: OLC6502::STA, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("STX"), operate: OLC6502::STX, addrmode: OLC6502::ZPY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("TYA"), operate: OLC6502::TYA, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("STA"), operate: OLC6502::STA, addrmode: OLC6502::ABY, cycles:5 },INSTRUCTION{name:String::from("TXS"), operate: OLC6502::TXS, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("STA"), operate: OLC6502::STA, addrmode: OLC6502::ABX, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },
                INSTRUCTION{name:String::from("LDY"), operate: OLC6502::LDY, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("LDA"), operate: OLC6502::LDA, addrmode: OLC6502::IZX, cycles:6 },INSTRUCTION{name:String::from("LDX"), operate: OLC6502::LDX, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("LDY"), operate: OLC6502::LDY, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("LDA"), operate: OLC6502::LDA, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("LDX"), operate: OLC6502::LDX, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:3 },INSTRUCTION{name:String::from("TAY"), operate: OLC6502::TAY, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("LDA"), operate: OLC6502::LDA, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("TAX"), operate: OLC6502::TAX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("LDY"), operate: OLC6502::LDY, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("LDA"), operate: OLC6502::LDA, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("LDX"), operate: OLC6502::LDX, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:4 },
                INSTRUCTION{name:String::from("BCS"), operate: OLC6502::BCS, addrmode: OLC6502::REL, cycles:2 },INSTRUCTION{name:String::from("LDA"), operate: OLC6502::LDA, addrmode: OLC6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("LDY"), operate: OLC6502::LDY, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("LDA"), operate: OLC6502::LDA, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("LDX"), operate: OLC6502::LDX, addrmode: OLC6502::ZPY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("CLV"), operate: OLC6502::CLV, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("LDA"), operate: OLC6502::LDA, addrmode: OLC6502::ABY, cycles:4 },INSTRUCTION{name:String::from("TSX"), operate: OLC6502::TSX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("LDY"), operate: OLC6502::LDY, addrmode: OLC6502::ABX, cycles:4 },INSTRUCTION{name:String::from("LDA"), operate: OLC6502::LDA, addrmode: OLC6502::ABX, cycles:4 },INSTRUCTION{name:String::from("LDX"), operate: OLC6502::LDX, addrmode: OLC6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:4 },
                INSTRUCTION{name:String::from("CPY"), operate: OLC6502::CPY, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("CMP"), operate: OLC6502::CMP, addrmode: OLC6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("CPY"), operate: OLC6502::CPY, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("CMP"), operate: OLC6502::CMP, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("DEC"), operate: OLC6502::DEC, addrmode: OLC6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("INY"), operate: OLC6502::INY, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("CMP"), operate: OLC6502::CMP, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("DEX"), operate: OLC6502::DEX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("CPY"), operate: OLC6502::CPY, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("CMP"), operate: OLC6502::CMP, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("DEC"), operate: OLC6502::DEC, addrmode: OLC6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BNE"), operate: OLC6502::BNE, addrmode: OLC6502::REL, cycles:2 },INSTRUCTION{name:String::from("CMP"), operate: OLC6502::CMP, addrmode: OLC6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("CMP"), operate: OLC6502::CMP, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("DEC"), operate: OLC6502::DEC, addrmode: OLC6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("CLD"), operate: OLC6502::CLD, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("CMP"), operate: OLC6502::CMP, addrmode: OLC6502::ABY, cycles:4 },INSTRUCTION{name:String::from("NOP"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("CMP"), operate: OLC6502::CMP, addrmode: OLC6502::ABX, cycles:4 },INSTRUCTION{name:String::from("DEC"), operate: OLC6502::DEC, addrmode: OLC6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("CPX"), operate: OLC6502::CPX, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("SBC"), operate: OLC6502::SBC, addrmode: OLC6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("CPX"), operate: OLC6502::CPX, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("SBC"), operate: OLC6502::SBC, addrmode: OLC6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("INC"), operate: OLC6502::INC, addrmode: OLC6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:5 },INSTRUCTION{name:String::from("INX"), operate: OLC6502::INX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("SBC"), operate: OLC6502::SBC, addrmode: OLC6502::IMM, cycles:2 },INSTRUCTION{name:String::from("NOP"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::SBC, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("CPX"), operate: OLC6502::CPX, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("SBC"), operate: OLC6502::SBC, addrmode: OLC6502::ABS, cycles:4 },INSTRUCTION{name:String::from("INC"), operate: OLC6502::INC, addrmode: OLC6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BEQ"), operate: OLC6502::BEQ, addrmode: OLC6502::REL, cycles:2 },INSTRUCTION{name:String::from("SBC"), operate: OLC6502::SBC, addrmode: OLC6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("SBC"), operate: OLC6502::SBC, addrmode: OLC6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("INC"), operate: OLC6502::INC, addrmode: OLC6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:6 },INSTRUCTION{name:String::from("SED"), operate: OLC6502::SED, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("SBC"), operate: OLC6502::SBC, addrmode: OLC6502::ABY, cycles:4 },INSTRUCTION{name:String::from("NOP"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::NOP, addrmode: OLC6502::IMP, cycles:4 },INSTRUCTION{name:String::from("SBC"), operate: OLC6502::SBC, addrmode: OLC6502::ABX, cycles:4 },INSTRUCTION{name:String::from("INC"), operate: OLC6502::INC, addrmode: OLC6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: OLC6502::XXX, addrmode: OLC6502::IMP, cycles:7 },
            ]
            
        }
    }

//addressing modes

    fn IMP()->u8{0}
    fn IMM()->u8{0}
    fn ZP0()->u8{0}
    fn ZPX()->u8{0}
    fn ZPY()->u8{0}
    fn REL()->u8{0}
    fn ABS()->u8{0}
    fn ABX()->u8{0}
    fn ABY()->u8{0}
    fn IND()->u8{0}
    fn IZX()->u8{0}
    fn IZY()->u8{0}

//opcodes

    fn ADC()->u8{0}
    fn AND()->u8{0}
    fn ASL()->u8{0}
    fn BCC()->u8{0}

    fn BCS()->u8{0}
    fn BEQ()->u8{0}
    fn BIT()->u8{0}
    fn BMI()->u8{0}

    fn BNE()->u8{0}
    fn BPL()->u8{0}
    fn BRK()->u8{0}
    fn BVC()->u8{0}

    fn BVS()->u8{0}
    fn CLC()->u8{0}
    fn CLD()->u8{0}
    fn CLI()->u8{0}

    fn CLV()->u8{0}
    fn CMP()->u8{0}
    fn CPX()->u8{0}
    fn CPY()->u8{0}

    fn DEC()->u8{0}
    fn DEX()->u8{0}
    fn DEY()->u8{0}
    fn EOR()->u8{0}

    fn INC()->u8{0}
    fn INX()->u8{0}
    fn INY()->u8{0}
    fn JMP()->u8{0}

    fn JSR()->u8{0}
    fn LDA()->u8{0}
    fn LDX()->u8{0}
    fn LDY()->u8{0}

    fn LSR()->u8{0}
    fn NOP()->u8{0}
    fn ORA()->u8{0}
    fn PHA()->u8{0}

    fn PHP()->u8{0}
    fn PLA()->u8{0}
    fn PLP()->u8{0}
    fn ROL()->u8{0}

    fn ROR()->u8{0}
    fn RTI()->u8{0}
    fn RTS()->u8{0}
    fn SBC()->u8{0}

    fn SEC()->u8{0}
    fn SED()->u8{0}
    fn SEI()->u8{0}
    fn STA()->u8{0}

    fn STX()->u8{0}
    fn STY()->u8{0}
    fn TAX()->u8{0}
    fn TAY()->u8{0}

    fn TSX()->u8{0}
    fn TXA()->u8{0}
    fn TXS()->u8{0}
    fn TYA()->u8{0}

    fn XXX()->u8{0}

//cpu functions
    fn clock(){}    
    fn reset(){}
    fn irq(){}
    fn nmi(){}


    fn fetch()->u8{0}

//memory operations
    pub fn read(&self, a: u16) -> u8{
        self.bus.read(a, false)
    }

    pub fn write(&mut self, a:u16, d:u8){
        self.bus.write(a, d)        
    }

//flag operations
    pub fn get_flag(&self, f:FLAGS6502)->u8{
        match f {
            FLAGS6502::C => self.status & 0b0000_0001,
            FLAGS6502::Z => self.status & 0b0000_0010,
            FLAGS6502::I => self.status & 0b0000_0100,
            FLAGS6502::D => self.status & 0b0000_1000,
            FLAGS6502::B => self.status & 0b0001_0000,
            FLAGS6502::U => self.status & 0b0010_0000,
            FLAGS6502::V => self.status & 0b0100_0000,
            FLAGS6502::N => self.status & 0b1000_0000,
        }   
    }

    pub fn set_flag(&mut self, f:FLAGS6502, v:bool){
        match f {
            //FLAGS6502::C => self.status |= 0b0000_0001,
            FLAGS6502::C => self.status |= ( v as u8 ) << 0,
            FLAGS6502::Z => self.status |= ( v as u8 ) << 1,
            FLAGS6502::I => self.status |= ( v as u8 ) << 2,
            FLAGS6502::D => self.status |= ( v as u8 ) << 3,
            FLAGS6502::B => self.status |= ( v as u8 ) << 4,
            FLAGS6502::U => self.status |= ( v as u8 ) << 5,
            FLAGS6502::V => self.status |= ( v as u8 ) << 6,
            FLAGS6502::N => self.status |= ( v as u8 ) << 7,            
        }

    }
}