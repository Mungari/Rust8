pub struct Cpu {
    I: u16,
    Vx: [u8; 16],
    PC: u16,
    SP: u8,
    STACK: [u16; 16],
    DELAY: u8,
    STIMER, u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            I: 0,
            Vx: [0; 16],
            PC: 0x200,
            SP: 0,
            STACK: [0; 16],
            DELAY: 0,
            STIMER: 0
        }
    }

    pub fn read_instruction(word: u8){
        match word {
            0x00E00 => {
                print!("FOUND CLS", word);
            },
            0x1___ => {
                print!("SET PC TO {:X?}", word);
            },
            0x2___ => {
                print!("CALL SUBROUTINE AT {:X?}", word);
            },
            0x3___ => {
                print!("SKIP NEXT ISNTRUCTION IF Vx = kk PC + 2 {:X?}", word);
            },
            0x4___ => {
                print!("SKIP NEXT ISNTRUCTION IF Vx != kk PC + 2 {:X?}", word);
            },
            0x5__0 => {
                print!("SKIP NEXT ISNTRUCTION IF Vx = Vy PC + 2 {:X?}", word);
            },
            0x6___ => {
                print!("PUT VALUE KK in Vx {:X?}", word);
            },
            0x7___ => {
                print!("ADD KK and Vx, store in Vx {:X?}", word);
            },
            0x8__0 => {
                print!("PUT VALUE Vy in Vx {:X?}", word);
            },
            0x8__1 => {
                print!("OR VALUE Vy and Vx, store in Vx {:X?}", word);
            },
            0x8__2 => {
                print!("AND VALUE Vy and Vx, store in Vx {:X?}", word);
            },
            0x8__3 => {
                print!("XOR VALUE Vy and Vx, store in Vx {:X?}", word);
            },
            0x8__4 => {
                print!("SET Vx= Vx+ Vy, VF = carry if res > 255 {:X?}", word);
            },
            0x8__5 => {
                print!("SET Vx = Vy - Vx. if Vx > Vy, then VF = 1.  {:X?}", word);
            },
            0x8__6 => {
                print!("SET Vx = Vx SHR 1. If Least Sign bit of Vx is 1, VF = 1. Vx//2 {:X?}", word);
            },
            0x8__7 => {
                print!("Set Vx = Vy - Vx, set VF = NOT borrow. Vy > Vx -> VF = 1. Vx - Vy -> Vx {:X?}", word);
            },
            0x8__E => {
                print!("Set Vx = Vx SHL 1. If LS Vx = 1, VF = 1. Vx * 2 {:X?}", word);
            },
            0x9__0 => {
                print!("SNE Vx , Vy. PC + 2 {:X?}", word);
            },
            0xA___ => {
                print!("SET I to nnn {:X?}", word);
            },
            0xB___ => {
                print!("JMP to nnn + V0 {:X?}", word);
            },
            0xC___ => {
                print!("SET Vx = kk AND rand_byte {:X?}", word);
            },
            0xD___ => {
                print!("DRAW at mem I at Vx, Vy {:X?}", word);
            },
            0xE_9E => {
                print!("SKIP Next Instr if Key with val Vx is pressed. PC + 2 {:X?}", word);
            },
            0xE_A1 => {
                print!("SKIP Next Instr if Key with val Vx is not pressed. PC + 2 {:X?}", word);
            },
            0xF_07 => {
                print!("Set Vx=DT (Delay Timer) {:X?}", word);
            },
            0xF_0A => {
                print!("Wait for Keypress, store val in Vx. Halt exec. {:X?}", word);
            },
            0xF_15 => {
                print!("Set DT = Vx. {:X?}", word);
            },
            0xF_18 => {
                print!("Set ST = Vx. {:X?}", word);
            },
            0xF_1E => {
                print!("Set I = I + Vx. {:X?}", word);
            },
            0xF_29 => {
                print!("Set I = location of sprite for digit Vx. {:X?}", word);
            },
            0xF_33 => {
                print!("Set BCD to I, I+1 and I+2 {:X?}", word);
            },
            0xF_55 => {
                print!("Store regs V0 .. Vx in mem starting at I. {:X?}", word);
            },
            0xF_65 => {
                print!("Read regs V0 .. Vx from mem starting at I. {:X?}", word);
            },
            _ => panic!()
        }
    }
}