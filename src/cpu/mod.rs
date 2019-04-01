bitflags! {
    #[derive(Default)]
    struct ConditionCodes: u8 {
        const Z = 0x01;
        const S = 0x02;
        const P = 0x04;
        const CY = 0x08;
        const AC = 0x10;
    }
}

#[derive(Default)]
pub struct CPU {
    // Registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    // Stack Pointer
    sp: u16,
    // Program Counter
    pc: u16,

    memory: Vec<u8>,
    condition: ConditionCodes,
    int_enable: u8,
}

impl CPU {
    pub fn run(mut self) -> Result<(), &'static str> {
        let opcode = self.memory[self.pc as usize];

        match opcode {
            // NOP
            0x00 => {}
            // ADD B
            0x80 => {
                // Do the math with higher precision so we can capture the carry out
                let answer: u16 = self.a as u16 + self.b as u16;

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADD C
            0x81 => {
                let answer: u16 = self.a as u16 + self.c as u16;

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADD D
            0x82 => {
                let answer: u16 = self.a as u16 + self.d as u16;

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADD E
            0x83 => {
                let answer: u16 = self.a as u16 + self.e as u16;

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADD H
            0x84 => {
                let answer: u16 = self.a as u16 + self.h as u16;

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADD L
            0x85 => {
                let answer: u16 = self.a as u16 + self.l as u16;

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADD M
            0x86 => {
                let m: u16 = ((self.h as u16) << 8) & self.l as u16;
                let answer: u16 = self.a as u16 + m;

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADD A
            0x87 => {
                let answer: u16 = self.a as u16 + self.a as u16;

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADC B
            0x88 => {
                let mut answer: u16 = self.a as u16 + self.b as u16;

                if self.condition.contains(ConditionCodes::CY) {
                    answer += 1;
                }

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADC C
            0x89 => {
                let mut answer: u16 = self.a as u16 + self.c as u16;

                if self.condition.contains(ConditionCodes::CY) {
                    answer += 1;
                }

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADC D
            0x8A => {
                let mut answer: u16 = self.a as u16 + self.d as u16;

                if self.condition.contains(ConditionCodes::CY) {
                    answer += 1;
                }

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADC E
            0x8B => {
                let mut answer: u16 = self.a as u16 + self.e as u16;

                if self.condition.contains(ConditionCodes::CY) {
                    answer += 1;
                }

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADC H
            0x8C => {
                let mut answer: u16 = self.a as u16 + self.h as u16;

                if self.condition.contains(ConditionCodes::CY) {
                    answer += 1;
                }

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADC L
            0x8D => {
                let mut answer: u16 = self.a as u16 + self.l as u16;

                if self.condition.contains(ConditionCodes::CY) {
                    answer += 1;
                }

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADC M
            0x8E => {
                let m: u16 = ((self.h as u16) << 8) & self.l as u16;
                let mut answer: u16 = self.a as u16 + m;

                if self.condition.contains(ConditionCodes::CY) {
                    answer += 1;
                }

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            // ADC A
            0x8F => {
                let mut answer: u16 = self.a as u16 + self.a as u16;

                if self.condition.contains(ConditionCodes::CY) {
                    answer += 1;
                }

                self.set_arth_flags(answer);

                self.a = (answer & 0xff) as u8;
            }
            _ => return Err("Error: Unimplemented instruction"),
        }

        self.pc += 1;
        Ok(())
    }

    fn set_arth_flags(&mut self, value: u16) -> ConditionCodes {
        self.condition = ConditionCodes::empty();

        // Zero flag: if the result is zero,
        // set the flag to zero
        // else clear the flag
        if value & 0xff == 0 {
            self.condition.insert(ConditionCodes::Z);
        } else {
            self.condition.remove(ConditionCodes::Z);
        }

        // Sign flag
        if value & 0x08 > 0 {
            self.condition.insert(ConditionCodes::S);
        } else {
            self.condition.remove(ConditionCodes::S);
        }

        // Carry flag
        if value >= 0xff {
            self.condition.insert(ConditionCodes::CY);
        } else {
            self.condition.remove(ConditionCodes::CY);
        }

        if parity(value) {
            self.condition.insert(ConditionCodes::P);
        } else {
            self.condition.remove(ConditionCodes::P);
        }

        self.condition
    }
}

fn parity(value: u16) -> bool {
    let mut value = value as u8;

    let mut p = 0;
    for _ in 0..8 {
        if value & 0x01 == 1 {
            p += 1;
        }
        value = value >> 1;
    }

    if p & 0x01 == 0 {
        true
    } else {
        false
    }
}
