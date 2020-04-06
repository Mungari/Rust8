pub struct Cpu {
    pub I: u16,
    pub Vx: [u8; 16],
    pub PC: u16,
    pub SP: u8,
    pub STACK: [u16; 16],
    pub DT: u8,
    pub ST: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            I: 0x200,
            Vx: [0; 16],
            PC: 0x200,
            SP: 0,
            STACK: [0; 16],
            DT: 0,
            ST: 0
        }
    }

    pub fn exec_instr(&mut self, word: u16){
        match word {
            0x00E0 => {
                println!("FOUND CLS {:X?}", word);
            },
            n if n  & 0x1000 != 0 => {
                println!("SET PC TO {:X?}", word);
            },
            n if n  & 0x2000 != 0 => {
                println!("CALL SUBROUTINE AT {:X?}", word);
            },
            n if n  & 0x3000 != 0 => {
                println!("SKIP NEXT ISNTRUCTION IF Vx = kk PC + 2 {:X?}", word);
            },
            n if n  & 0x4000 != 0 => {
                println!("SKIP NEXT ISNTRUCTION IF Vx != kk PC + 2 {:X?}", word);
            },
            n if n  & 0x5000 != 0 => {
                println!("SKIP NEXT ISNTRUCTION IF Vx = Vy PC + 2 {:X?}", word);
            },
            n if n  & 0x6000 != 0 => {
                println!("PUT VALUE KK in Vx {:X?}", word);
            },
            n if n  & 0x7000 != 0 => {
                println!("ADD KK and Vx, store in Vx {:X?}", word);
            },
            n if n  & 0x8000 != 0 => {
                println!("PUT VALUE Vy in Vx {:X?}", word);
            },
            n if n  & 0x8001 != 0 => {
                println!("OR VALUE Vy and Vx, store in Vx {:X?}", word);
            },
            n if n  & 0x8002 != 0 => {
                println!("AND VALUE Vy and Vx, store in Vx {:X?}", word);
            },
            n if n  & 0x8003 != 0 => {
                println!("XOR VALUE Vy and Vx, store in Vx {:X?}", word);
            },
            n if n  & 0x8004 != 0 => {
                println!("SET Vx= Vx+ Vy, VF = carry if res > 255 {:X?}", word);
            },
            n if n  & 0x8005 != 0 => {
                println!("SET Vx = Vy - Vx. if Vx > Vy, then VF = 1.  {:X?}", word);
            },
            n if n  & 0x8006 != 0 => {
                println!("SET Vx = Vx SHR 1. If Least Sign bit of Vx is 1, VF = 1. Vx//2 {:X?}", word);
            },
            n if n  & 0x8007 != 0 => {
                println!("Set Vx = Vy - Vx, set VF = NOT borrow. Vy > Vx -> VF = 1. Vx - Vy -> Vx {:X?}", word);
            },
            n if n  & 0x800E != 0 => {
                println!("Set Vx = Vx SHL 1. If LS Vx = 1, VF = 1. Vx * 2 {:X?}", word);
            },
            n if n  & 0x9000 != 0 => {
                println!("SNE Vx , Vy. PC + 2 {:X?}", word);
            },
            n if n  & 0xA000 != 0 => {
                println!("SET I to nnn {:X?}", word);
            },
            n if n  & 0xB000 != 0 => {
                println!("JMP to nnn + V0 {:X?}", word);
            },
            n if n  & 0xC000 != 0 => {
                println!("SET Vx = kk AND rand_byte {:X?}", word);
            },
            n if n  & 0xD000 != 0 => {
                println!("DRAW at mem I at Vx, Vy {:X?}", word);
            },
            n if n  & 0xE09E != 0 => {
                println!("SKIP Next Instr if Key with val Vx is pressed. PC + 2 {:X?}", word);
            },
            n if n  & 0xE0A1 != 0 => {
                println!("SKIP Next Instr if Key with val Vx is not pressed. PC + 2 {:X?}", word);
            },
            n if (n & 0xF0FF) == 0xF007 => {
                println!("Set Vx=DT (Delay Timer) {:X?}", word);
            },
            n if n  & 0xF00A != 0 => {
                println!("Wait for Keypress, store val in Vx. Halt exec. {:X?}", word);
            },
            n if n  & 0xF015 != 0 => {
                println!("Set DT = Vx. {:X?}", word);
            },
            n if n  & 0xF018 != 0 => {
                println!("Set ST = Vx. {:X?}", word);
            },
            n if n  & 0xF01E != 0 => {
                println!("Set I = I + Vx. {:X?}", word);
            },
            n if n  & 0xF029 != 0 => {
                println!("Set I = location of sprite for digit Vx. {:X?}", word);
            },
            n if n  & 0xF033 != 0 => {
                println!("Set BCD to I, I+1 and I+2 {:X?}", word);
            },
            n if n  & 0xF055 != 0 => {
                println!("Store regs V0 .. Vx in mem starting at I. {:X?}", word);
            },
            n if n  & 0xF065 != 0 => {
                println!("Read regs V0 .. Vx from mem starting at I. {:X?}", word);
            },
            _ => { 
                println!("Unercognized instr {:X?}", word);
            }
        }
    }
}