use std::collections::HashMap;

#[derive(Debug)]
pub struct Processor {
    pub pc: i32,          // program counter
    pub cod_machine: i32, // codigo mauqina da instrucao
    pub regs: [i32; 32],  // arr com 32 registos [0..31] todos do tipo i32
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            pc: 0x10000,    // valor inicial do pc
            regs: [0; 32],  // inicializar os registos a 0(zero)
            cod_machine: 0, // valor inicial do codigo maquina
        }
    }

    //devolve a soma do que esta no registo rs1 com o que esta no registo rs2
    pub fn add(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        self.regs[r1] + self.regs[r2]
    }

    //devolve a subtraçao do que esta no registo rs1 com o que esta no registo rs2
    pub fn sub(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        self.regs[r1] - self.regs[r2]
    }

    //compara se o que esta no registo rs1 é menor que o que esta no registo rs2, devolvendo 1, caso contrario devolve 0
    pub fn slt(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;

        if self.regs[r1] < self.regs[r2] {
            1
        } else {
            0
        }
    }

    //compara se o que esta no registo rs1 é menor que o que esta no registo rs2, devolvendo 1, caso contrario devolve 0
    //mas sem se preocupar com o sinal
    pub fn sltu(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;

        if (self.regs[r1] as u32) < (self.regs[r2] as u32) {
            1
        } else {
            0
        }
    }

    //devolve o and do que esta no registo rs1 com o que esta no registo rs2
    // 0 | 0 | 0
    // 0 | 1 | 0
    // 1 | 0 | 0
    // 1 | 1 | 1
    pub fn and(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        self.regs[r1] & self.regs[r2]
    }

    //devolve o or do que esta no registo rs1 com o que esta no registo rs2
    // 0 | 0 | 0
    // 0 | 1 | 1
    // 1 | 0 | 1
    // 1 | 1 | 1
    pub fn or(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        self.regs[r1] | self.regs[r2]
    }

    //devolve o xor do que esta no registo rs1 com o que esta no registo rs2
    // 0 | 0 | 0
    // 0 | 1 | 1
    // 1 | 0 | 1
    // 1 | 1 | 0
    pub fn xor(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        self.regs[r1] ^ self.regs[r2]
    }

    //devolve a deslocaçao para a esquerda com o que esta no registo rs1 o numero de casa que o que esta no registo rs2 tem
    pub fn sll(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        self.regs[r1] << self.regs[r2]
    }

    //devolve a deslocaçao para a direita com o que esta no registo rs1 o numero de casa que o que esta no registo rs2 tem
    pub fn srl(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;

        (self.regs[r1] as u32 >> ((self.regs[r2] as u32) & 0x1f)) as i32
    }

    //devolve a deslocaçao para a direita com o que esta no registo rs1 o numero de casa que o que esta no registo rs2 tem, com a condiçao que pode ser negativo
    pub fn sra(&self, rs1: i32, rs2: i32) -> i32 {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        if self.regs[r1] >= 0 {
            self.srl(rs1, rs2);
        }

        self.regs[r1] >> (self.regs[r2] & 0x1f)
    }

    //devolve a deslocaçao para a esquerda com o que esta no registo rs1 o numero de casa que o que esta no immidiate
    pub fn slli(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;
        self.regs[r1] << imm
    }
    //devolve a deslocaçao para a direita com o que esta no registo rs1 o numero de casa que o que esta no immidiate
    pub fn srli(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;
        self.regs[r1] >> imm
    }

    //devolve a deslocaçao para a direita com o que esta no registo rs1 o numero de casa que o que esta no immidiate, com a condiçao que pode ser negativo
    pub fn srai(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;
        self.regs[r1] >> imm
    }

    //devolve a soma do que esta no registo rs1 com o que esta no immidiate
    pub fn addi(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;
        self.regs[r1] + imm
    }

    //compara se o que esta no registo rs1 é menor que o que esta no immidiate, devolvendo 1, caso contrario devolve 0
    pub fn slti(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;

        if self.regs[r1] < imm {
            1
        } else {
            0
        }
    }

    //compara se o que esta no registo rs1 é menor que o que esta no immidiate, devolvendo 1, caso contrario devolve 0
    //mas sem se preocupar com o sinal
    pub fn sltiu(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;

        if self.regs[r1] < imm {
            1
        } else {
            0
        }
    }

    //devolve o and do que esta no registo rs1 com o que esta no immidiate
    // 0 | 0 | 0
    // 0 | 1 | 0
    // 1 | 0 | 0
    // 1 | 1 | 1
    pub fn andi(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;
        self.regs[r1] & imm
    }

    //devolve o or do que esta no registo rs1 com o que esta no immidiate
    // 0 | 0 | 0
    // 0 | 1 | 1
    // 1 | 0 | 1
    // 1 | 1 | 1
    pub fn ori(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;
        self.regs[r1] | imm
    }

    //devolve o xor do que esta no registo rs1 com o que esta no immidiate
    // 0 | 0 | 0
    // 0 | 1 | 1
    // 1 | 0 | 1
    // 1 | 1 | 0
    pub fn xori(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;
        self.regs[r1] ^ imm
    }

    //devolve a soma do que esta no registo rs1 com o que esta no immidiate para fazer o salto
    pub fn jalr(&self, rs1: i32, imm: i32) -> i32 {
        let r1: usize = rs1 as usize;

        (self.regs[r1] + imm) & (-2)
    }

    //compara se o que esta no registo rs1 é igual ao que esta no registo rs2, devolvendo true caso a igualdade se cumpra e false no caso contrario
    pub fn beq(&self, rs1: i32, rs2: i32) -> bool {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;

        self.regs[r1] == self.regs[r2]
    }

    //compara se o que esta no registo rs1 nao é igual ao que esta no registo rs2, devolvendo true caso a igualdade nao se cumpra e false no caso contrario
    pub fn bne(&self, rs1: i32, rs2: i32) -> bool {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;

        self.regs[r1] == self.regs[r2]
    }

    //compara se o que esta no registo rs1 é menor do que o que esta no registo rs2, devolvendo true caso se cumpra a condiçao e false caso contrario
    pub fn blt(&self, rs1: i32, rs2: i32) -> bool {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;

        self.regs[r1] < self.regs[r2]
    }

    //compara se o que esta no registo rs1 é maior ou igual do que o que esta no registo rs2, devolvendo true caso se cumpra a condiçao e false caso contrario
    pub fn bge(&self, rs1: i32, rs2: i32) -> bool {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;

        self.regs[r1] >= self.regs[r2]
    }

    //compara se o que esta no registo rs1 é menor do que o que esta no registo rs2, devolvendo true caso se cumpra a condiçao e false caso contrario
    //mas sem se preocupar com o sinal
    pub fn bltu(&self, rs1: i32, rs2: i32) -> bool {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        self.regs[r1] < self.regs[r2]
    }
    //compara se o que esta no registo rs1 é maior ou igual do que o que esta no registo rs2, devolvendo true caso se cumpra a condiçao e false caso contrario
    //mas sem se preocupar com o sinal
    pub fn bgeu(&self, rs1: i32, rs2: i32) -> bool {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        self.regs[r1] >= self.regs[r2]
    }

    //devolve a soma do que esta no program counter com o que esta no immidiate para fazer o salto
    pub fn jal(&self, imm: i32) -> i32 {
        self.pc + imm
    }

    //devolve a word que esta na posiçao de memoria da soma do que esta no registo rs1 com o que esta no immidiate
    pub fn lw(&self, rs1: i32, imm: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        let r1: usize = rs1 as usize;

        if (self.regs[r1] + imm) % 4 != 0 {
            panic!("The imidiate must be multiple of 4");
        }

        *mem.get(&(self.regs[r1] + imm)).unwrap()
    }

    //devolve metade de uma word na posiçao de memoria
    //como é apenas half word temos de ver o que lá esta na memora primeiro e em seguida devolver a metade pertendida
    pub fn lh(&self, rs1: i32, imm: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        let r1: usize = rs1 as usize;
        let mut immidiate = imm;

        if (self.regs[r1] + imm) % 4 != 0 {
            if (self.regs[r1] + imm) % 2 != 0 {
                panic!("The must be multiple of 4");
            }
            immidiate -= 2;
        }

        match mem.get(&(self.regs[r1] + immidiate)) {
            Some(e) => {
                if (self.regs[r1] + imm) % 4 != 0 {
                    (*e as u32 & 0x0000ffff as u32) as i32
                } else {
                    (*e as u32 & 0xffff0000 as u32) as i32
                }
            }
            None => 0,
        }
    }

    //lhu faz o mesmo que lh mas sem se preocupar com o sinal
    pub fn lhu(&self, rs1: i32, imm: i32, mem: &mut HashMap<i32, i32>) -> u32 {
        let r1: usize = rs1 as usize;
        let mut immidiate = imm;

        if (self.regs[r1] + imm) % 4 != 0 {
            if (self.regs[r1] + imm) % 2 != 0 {
                panic!("The must be multiple of 4");
            }
            immidiate -= 2;
        }

        match mem.get(&(self.regs[r1] + immidiate)) {
            Some(e) => {
                if (self.regs[r1] + imm) % 4 != 0 {
                    *e as u32 & 0x0000ffff as u32
                } else {
                    *e as u32 & 0xffff0000 as u32
                }
            }
            None => 0,
        }
    }

    //devolve 1byte da word que esta na posiçao de memoria
    //como é apenas 1byte da word temos de ver o que lá esta na memora primeiro e em seguida devolver o byte pertendido
    pub fn lb(&self, rs1: i32, imm: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        let r1: usize = rs1 as usize;
        let mut immidiate = imm;

        loop {
            if (self.regs[r1] + immidiate) % 4 != 0 {
                immidiate -= 1;
            } else {
                break;
            };
        }

        match mem.get(&(self.regs[r1] + immidiate)) {
            Some(e) => match imm {
                0 => (*e as u32 & 0x000000ff as u32) as i32,

                1 => (*e as u32 & 0x0000ff00 as u32) as i32,

                2 => (*e as u32 & 0x00ff0000 as u32) as i32,

                3 => (*e as u32 & 0xff000000 as u32) as i32,

                _ => panic!("Not a valid machine code!"),
            },
            None => 0,
        }
    }

    //lbu faz o mesmo que lb mas sem se preocupar com o sinal
    pub fn lbu(&self, rs1: i32, imm: i32, mem: &mut HashMap<i32, i32>) -> u32 {
        let r1: usize = rs1 as usize;
        let mut immidiate = imm;

        loop {
            if (self.regs[r1] + immidiate) % 4 != 0 {
                immidiate -= 1;
            } else {
                break;
            };
        }

        match mem.get(&(self.regs[r1] + immidiate)) {
            Some(e) => match imm {
                0 => (*e as u32 & 0x000000ff as u32) as u32,

                1 => (*e as u32 & 0x0000ff00 as u32) as u32,

                2 => (*e as u32 & 0x00ff0000 as u32) as u32,

                3 => (*e as u32 & 0xff000000 as u32) as u32,

                _ => panic!("Not a valid machine code!"),
            },
            None => 0,
        }
    }

    //guarda em memoria uma word
    pub fn sw(&self, rs1: i32, rs2: i32, imm: i32, mem: &mut HashMap<i32, i32>) {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;

        if (self.regs[r1] + imm) % 4 != 0 {
            panic!("The must be multiple of 4");
        }

        mem.insert(self.regs[r1] + imm, self.regs[r2]);
    }

    //guarda em memoria metade de uma word
    //como apenas guarda half word temos de retirar o que está em memoria colocar a zeros a metade que queremos substituir
    //colocar a half word nesse espaço e guardar de novo em memoria
    pub fn sh(&self, rs1: i32, rs2: i32, imm: i32, mem: &mut HashMap<i32, i32>) {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        let mut immidiate = imm;

        if (self.regs[r1] + imm) % 4 != 0 {
            if (self.regs[r1] + imm) % 2 != 0 {
                panic!("The must be multiple of 4");
            }
            immidiate -= 2;
        }

        let data_in;
        let data_out = match mem.get(&(self.regs[r1] + immidiate)) {
            Some(e) => {
                if (self.regs[r1] + imm) % 4 != 0 {
                    *e as u32 & 0x0000ffff as u32
                } else {
                    *e as u32 & 0xffff0000 as u32
                }
            }
            None => 0,
        };

        let data = (self.regs[r2] as u32) & (0x0000ffff as u32);

        if (self.regs[r1] + imm) % 4 != 0 {
            data_in = (data << 16) | data_out as u32;
        } else {
            data_in = data_out as u32 | data;
        }

        mem.insert(self.regs[r1] + immidiate, data_in as i32);
    }

    //guarda em memoria 1byte de uma word
    //como apenas guarda 1byte de uma word temos de retirar o que está em memoria colocar a zeros o byte que queremos substituir
    //colocar o byte nesse espaço e guardar de novo em memoria
    pub fn sb(&self, rs1: i32, rs2: i32, imm: i32, mem: &mut HashMap<i32, i32>) {
        let r1: usize = rs1 as usize;
        let r2: usize = rs2 as usize;
        let mut immidiate = imm;

        loop {
            if (self.regs[r1] + immidiate) % 4 != 0 {
                immidiate -= 1;
            } else {
                break;
            };
        }

        let data_out = self.regs[r2] & 0x000000ff;

        let data_in = match mem.get(&(self.regs[r1] + immidiate)) {
            Some(e) => match imm {
                0 => (*e as u32 & 0xffffff00 as u32) as i32 | data_out,

                1 => (*e as u32 & 0xffff00ff as u32) as i32 | data_out << 8,

                2 => (*e as u32 & 0xff00ffff as u32) as i32 | data_out << 16,

                3 => (*e as u32 & 0x00ffffff as u32) as i32 | data_out << 24,

                _ => panic!("Not a valid machine code!"),
            },
            None => data_out,
        };

        mem.insert(self.regs[r1] + immidiate, data_in);
    }

    //devolve a deslocaçao para a esquerda do immidiate 12 bytes
    pub fn lui(&self, imm: i32) -> i32 {
        imm << 12
    }

    //devolve a soma da deslocaçao para a esquerda do immidiate 12 bytes com o program counter
    pub fn auipc(&self, imm: i32) -> i32 {
        self.pc + imm << 12
    }
}
