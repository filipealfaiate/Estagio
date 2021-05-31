mod processor;
extern crate queues;
use crate::processor::Processor;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    //criar um vector que contem os argumentos do terminal
    let args: Vec<String> = env::args().collect();

    //inicialização da memoria
    let mut mem = HashMap::new();
    //inicialização do processador
    let mut processor = Processor::new();

    //buffer que vai conter todas as instruções do projecto
    let buffer;

    //verificar qual o tamanho dos argumentos recebidos pelo terminal
    match args.len() {
        1 => panic!("Nome do ficheiro em falta"),

        2 => {
            buffer = fs::read(&args[1])?;
        } //leitura de todo o ficheiro que esta no args[1], seja um caminho absoluto ou relativo, para o buffer

        _ => panic!("Demasiados argumentos, apenas necessita do nome do ficheiro"),
    };

    //colocar todas as instruções que estao no buffer em memoria
    codigo_em_memoria(buffer, &mut processor, &mut mem);

    //intrepeta e corre o projecto RISC
    intrepetar_codigo(&mut processor, &mut mem);

    //termina o programa num estado ok, caso haja algum erro a cima será automáticamente devolvido
    Ok(())
}

fn codigo_em_memoria(buffer: Vec<u8>, processor: &mut Processor, mem: &mut HashMap<i32, i32>) {
    //contador para verificar se chegou ao fim
    let mut i = 0;

    //ciclo para ler todo o codigo maquina ate ao seu comprimento
    loop {
        //se chegou ao fim para
        if i > buffer.len() - 1 {
            break;
        }

        //colocar em memoria arranjando, pois o ficheiro encontra-se em Little Endian
        mem.insert(
            processor.pc,
            ((buffer[i + 3] as i32) << 24)
                | ((buffer[i + 2] as i32) << 16)
                | ((buffer[i + 1] as i32) << 8)
                | (buffer[i] as i32),
        );

        //salta 4 posicoes, pois cada instrucao que é montada na linha de cima tem 4 bytes (8bits por byte)
        processor.pc += 4;
        i += 4;
    }

    //colocar o program counter na posição inicial para ler o programa
    processor.pc = 0x10000;
}

fn intrepetar_codigo(processor: &mut Processor, mem: &mut HashMap<i32, i32>) {
    //ciclo que intrepeta e executa cada instruçao
    loop {
        //um sinalizador para verificar se entrou em algum salto
        let mut flag_branch = false;

        //verificar se o valor que existe no program counter existe
        //se existir guarda esse valor no codigo maquina
        //se nao existir termina o ciclo
        processor.cod_machine = match mem.get(&(processor.pc)) {
            Some(expr) => *expr,
            None => break,
        };

        //divisao do codigo maquina nas respetivas componentes
        let funct_31_25: i32 = ((processor.cod_machine as u32 & 0xfe000000 as u32) >> 25) as i32;

        let rs2_24_20: i32 = ((processor.cod_machine as u32 & 0x01f00000 as u32) >> 20) as i32;

        let rs1_19_15: i32 = ((processor.cod_machine as u32 & 0x000f8000 as u32) >> 15) as i32;

        let funct_14_12: i32 = ((processor.cod_machine as u32 & 0x00007000 as u32) >> 12) as i32;

        let rd_11_7: usize = ((processor.cod_machine as u32 & 0x00000f80 as u32) >> 7) as usize;

        let opcode_6_0: i32 = (processor.cod_machine as u32 & 0x0000007f as u32) as i32;

        let imm_31_20: i32 = processor.cod_machine >> 20;

        let imm_31_25_11_7: i32 = (((funct_31_25 << 5) as u32) | rd_11_7 as u32) as i32;

        let imm_31_12: i32 = processor.cod_machine >> 12;

        //cada match intrepeta uma parte do codigo maquina o que leva ao local de execuçao de uma instruçao
        //caso o match falhe, o programa ira abortar de uma forma segura utilizando o comando "panic!" e indicando que nao é um codigo de maquina valido
        match opcode_6_0 {
            0b0110011 => match funct_31_25 {
                0b0000000 => match funct_14_12 {
                    0b000 => {
                        println!("add {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.add(rs1_19_15, rs2_24_20);
                    }

                    0b010 => {
                        println!("slt {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.slt(rs1_19_15, rs2_24_20);
                    }

                    0b011 => {
                        println!("sltu {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.sltu(rs1_19_15, rs2_24_20);
                    }

                    0b111 => {
                        println!("and {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.and(rs1_19_15, rs2_24_20);
                    }

                    0b110 => {
                        println!("or {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.or(rs1_19_15, rs2_24_20);
                    }

                    0b100 => {
                        println!("xor {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.xor(rs1_19_15, rs2_24_20);
                    }

                    0b001 => {
                        println!("sll {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.sll(rs1_19_15, rs2_24_20);
                    }

                    0b101 => {
                        println!("srl {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.srl(rs1_19_15, rs2_24_20);
                    }

                    _ => panic!("Not a valid machine code!"),
                },

                0b0100000 => match funct_14_12 {
                    0b000 => {
                        println!("sub {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.sub(rs1_19_15, rs2_24_20);
                    }

                    0b101 => {
                        println!("sra {}, {}, {}", rd_11_7, rs1_19_15, rs2_24_20);
                        processor.regs[rd_11_7] = processor.sra(rs1_19_15, rs2_24_20);
                    }

                    _ => panic!("Not a valid machine code!"),
                },

                _ => panic!("Not a valid machine code!"),
            },

            0b0010011 => match funct_14_12 {
                0b001 => match funct_31_25 {
                    0b0000000 => {
                        println!("slli {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                        processor.regs[rd_11_7] = processor.slli(rs1_19_15, imm_31_20);
                    }
                    _ => panic!("Not a valid machine code!"),
                },

                0b101 => match funct_31_25 {
                    0b0000000 => {
                        println!("srli {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                        processor.regs[rd_11_7] = processor.srli(rs1_19_15, imm_31_20);
                    }

                    0b0100000 => {
                        println!("srai {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                        processor.regs[rd_11_7] = processor.srai(rs1_19_15, imm_31_20);
                    }

                    _ => panic!("Not a valid machine code!"),
                },

                0b000 => {
                    println!("addi {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                    processor.regs[rd_11_7] = processor.addi(rs1_19_15, imm_31_20);
                }

                0b010 => {
                    println!("slti {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                    processor.regs[rd_11_7] = processor.slti(rs1_19_15, imm_31_20);
                }

                0b011 => {
                    println!("sltiu {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                    processor.regs[rd_11_7] = processor.sltiu(rs1_19_15, imm_31_20);
                }

                0b111 => {
                    println!("andi {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                    processor.regs[rd_11_7] = processor.andi(rs1_19_15, imm_31_20);
                }

                0b110 => {
                    println!("ori {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                    processor.regs[rd_11_7] = processor.ori(rs1_19_15, imm_31_20);
                }

                0b100 => {
                    println!("xori {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                    processor.regs[rd_11_7] = processor.xori(rs1_19_15, imm_31_20);
                }

                _ => panic!("Not a valid machine code!"),
            },
            0b0000011 => match funct_14_12 {
                0b010 => {
                    println!("lw {}, {}({})", rd_11_7, imm_31_20, rs1_19_15);
                    processor.regs[rd_11_7] = processor.lw(rs1_19_15, imm_31_20, mem);
                }

                0b001 => {
                    println!("lh {}, {}({})", rd_11_7, imm_31_20, rs1_19_15);
                    processor.regs[rd_11_7] = processor.lh(rs1_19_15, imm_31_20, mem);
                }

                0b101 => {
                    println!("lhu {}, {}({})", rd_11_7, imm_31_20, rs1_19_15);
                    processor.regs[rd_11_7] =
                        processor.lhu(rs1_19_15, imm_31_20, mem).try_into().unwrap();
                }

                0b000 => {
                    println!("lb {}, {}({})", rd_11_7, imm_31_20, rs1_19_15);

                    processor.regs[rd_11_7] = processor.lb(rs1_19_15, imm_31_20, mem);
                }

                0b100 => {
                    println!("lbu {}, {}({})", rd_11_7, imm_31_20, rs1_19_15);
                    processor.regs[rd_11_7] =
                        processor.lbu(rs1_19_15, imm_31_20, mem).try_into().unwrap();
                }

                _ => panic!("Not a valid machine code!"),
            },

            0b1100111 => match funct_14_12 {
                0b000 => {
                    println!("jalr {}, {}, {}", rd_11_7, rs1_19_15, imm_31_20);
                    processor.regs[rd_11_7] = processor.jalr(rs1_19_15, imm_31_20);
                }

                _ => panic!("Not a valid machine code!"),
            },

            0b0100011 => match funct_14_12 {
                0b010 => {
                    println!("sw {}, {}({})", rs2_24_20, imm_31_25_11_7, rs1_19_15);
                    processor.sw(rs1_19_15, rs2_24_20, imm_31_25_11_7, mem);
                }

                0b001 => {
                    println!("sh {}, {}({})", rs1_19_15, imm_31_25_11_7, rs2_24_20);
                    processor.sh(rs1_19_15, rs2_24_20, imm_31_25_11_7, mem);
                }

                0b000 => {
                    println!("sb {}, {}({})", rs1_19_15, imm_31_25_11_7, rs2_24_20);
                    processor.sb(rs1_19_15, rs2_24_20, imm_31_25_11_7, mem);
                }

                _ => panic!("Not a valid machine code!"),
            },

            0b1100011 => match funct_14_12 {
                0b000 => {
                    println!("beq {}, {}, {}", rs1_19_15, rs2_24_20, imm_31_25_11_7);
                    if processor.beq(rs1_19_15, rs2_24_20) {
                        processor.pc += imm_31_25_11_7;
                        flag_branch = true;
                    }
                }

                0b001 => {
                    println!("bne {}, {}, {}", rs1_19_15, rs2_24_20, imm_31_25_11_7);
                    if processor.bne(rs1_19_15, rs2_24_20) {
                        processor.pc += imm_31_25_11_7;
                        flag_branch = true;
                    }
                }

                0b100 => {
                    println!("blt {}, {}, {}", rs1_19_15, rs2_24_20, imm_31_25_11_7);
                    if processor.blt(rs1_19_15, rs2_24_20) {
                        processor.pc += imm_31_25_11_7;
                        flag_branch = true;
                    }
                }

                0b101 => {
                    println!("bge {}, {}, {}", rs1_19_15, rs2_24_20, imm_31_25_11_7);
                    if processor.bge(rs1_19_15, rs2_24_20) {
                        processor.pc += imm_31_25_11_7;
                        flag_branch = true;
                    }
                }

                0b110 => {
                    println!("bltu {}, {}, {}", rs1_19_15, rs2_24_20, imm_31_25_11_7);
                    if processor.bltu(rs1_19_15, rs2_24_20) {
                        if imm_31_25_11_7 < 0 {
                            let aux: u32 = imm_31_25_11_7 as u32 ^ 0xfffff800 as u32;
                            processor.pc += aux as i32;
                            flag_branch = true;
                        } else {
                            processor.pc += imm_31_25_11_7;
                            flag_branch = true;
                        }
                    }
                }

                0b111 => {
                    println!("bgeu {}, {}, {}", rs1_19_15, rs2_24_20, imm_31_25_11_7);
                    if processor.bgeu(rs1_19_15, rs2_24_20) {
                        if imm_31_25_11_7 < 0 {
                            let aux: u32 = imm_31_25_11_7 as u32 ^ 0xfffff800 as u32;
                            processor.pc += aux as i32;
                            flag_branch = true;
                        } else {
                            processor.pc += imm_31_25_11_7;
                            flag_branch = true;
                        }
                    }
                }

                _ => panic!("Not a valid machine code!"),
            },

            0b1101111 => {
                println!("jal {}", imm_31_12);
                processor.pc = processor.jal(imm_31_12);
                flag_branch = true;
            }

            0b0110111 => {
                println!("lui {}, {}", rd_11_7, imm_31_12);
                processor.regs[rd_11_7] = processor.lui(imm_31_12);
            }

            0b1110011 => match imm_31_20 {
                0b000000000000 => println!("ecall foi executada!"),

                0b000000000001 => println!("ebreak foi executada!"),

                _ => panic!("Not a valid machine code!"),
            },

            0b0010111 => {
                println!("auipc {}, {}", rd_11_7, imm_31_12);
                processor.regs[rd_11_7] = processor.auipc(imm_31_12);
            }

            _ => panic!("Not a valid machine code!"),
        };

        //caso durante a intrepetaçao e execucao tenha entrado numa instruçao de branch entao o progam counter ja foi atualizado
        //e nao precisa de avançar para a instruçao seguinte mas sim para a instruçao que o branch designou
        if flag_branch == false {
            processor.pc += 4;
        }
    }

    //no final mostra os registos e a memoria
    println!("\nREGISTOS: \n{:?}", processor.regs);
    println!("\nMEMORIA: \n{:?}", mem);
}
