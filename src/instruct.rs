use std::io::{Read, stdin};
use std::ops::{DerefMut};
use std::process::exit;
use crate::init::Machine;

type Umi = u32;

pub struct Field {
    width: u32,
    lsb: u32,
}
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

enum Opcode { CMov, Load, Store, Add, Mul, Div, NAND, HALT, MapSeg, UMapSeg, Out, In, LP, LV }

fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn perform(inst: Umi, mut machine: &mut Machine){
    //contents of registers
    let a = machine.registers[get(&RA, inst) as usize];
    let b = machine.registers[get(&RB, inst) as usize];
    let c = machine.registers[get(&RC, inst) as usize];

    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {

            if c != 0{
                machine.deref_mut().registers[get(&RA, inst) as usize] = b as u32;
            }

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::Load as u32 => {

            machine.registers[get(&RA, inst) as usize] = machine.memory[b as usize][c as usize];

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::Store as u32 => {

            machine.deref_mut().memory[a as usize][b as usize] = c;

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::Add as u32 => {

            machine.deref_mut().registers[get(&RA, inst) as usize] = b.wrapping_add(c);

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::Mul as u32 => {

            machine.deref_mut().registers[get(&RA, inst) as usize] = b.wrapping_mul(c);

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::Div as u32 => {

            machine.deref_mut().registers[get(&RA, inst) as usize] = b / c;

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::NAND as u32 => {

            machine.deref_mut().registers[get(&RA, inst) as usize] = !(b & c);

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::HALT as u32 => {

            exit(0)

        }
        o if o == Opcode::MapSeg as u32 => {

            let new_seg = vec![0_u32; c as usize];
            let new_idx;

            //checks for any reusable memory segments
            if machine.unmapped.len() > 0{
                let reused = machine.unmapped.pop().unwrap() as usize;
                machine.deref_mut().memory[reused] = new_seg;
                new_idx = reused;
            }
            else {
                machine.deref_mut().memory.push(new_seg);
                new_idx = machine.memory.len() - 1;
            }

            //places index of new memory segment in register B
            machine.deref_mut().registers[get(&RB, inst) as usize] = new_idx as u32;

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::UMapSeg as u32 => {

            machine.deref_mut().unmapped.push(c as u32);
            machine.deref_mut().memory[c as usize] = vec![];

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::Out as u32 => {

            print!("{}", char::from_u32(c).unwrap());

            machine.deref_mut().program_counter += 1;

        }
        o if o == Opcode::In as u32 => {

            //let mut line = String::new();
            let mut buffer = [0_u8];
            let res = stdin().read(&mut buffer).unwrap();

            if res == 1{
                machine.deref_mut().registers[get(&RC, inst) as usize] = buffer[0] as u32
            }
            else if res == 0 {
                machine.deref_mut().registers[get(&RC, inst) as usize] = u32::MAX
            }
            //match to 1 or 0
            //machine.deref_mut().registers[get(&RC, inst) as usize] = std::io::stdin().read_exact(buffer).unwrap() as u32;

            machine.deref_mut().program_counter += 1;
        }
        o if o == Opcode::LP as u32 => {

            let dupe_seg = machine.memory[b as usize].clone();
            machine.deref_mut().memory[0] = dupe_seg;

            //The program counter is set to point to $m[0][$r[C]]
            machine.deref_mut().program_counter = c as u32;

        }
        o if o == Opcode::LV as u32 => {

            machine.deref_mut().registers[get(&RL, inst) as usize] = get(&VL, inst);

            machine.deref_mut().program_counter += 1;

        }
        _ => {()}
    }
}