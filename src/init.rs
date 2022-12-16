use crate::rumload;

//#[derive(Clone)]
pub struct Machine {
    pub(crate) memory: Vec<Vec<u32>>,
    pub(crate) registers: Vec<u32>,
    pub(crate) unmapped: Vec<u32>,
    pub(crate) program_counter: u32
}

pub fn initialize(input: Option<String>) -> Machine{
    let mut machine: Machine = Machine { memory: vec![vec![]], registers: vec![0; 8], unmapped: vec![], program_counter: 0 };
    let instructions = rumload::load(input.as_deref());
    for instruction in instructions {
        machine.memory[0].push(instruction)
    }
    //println!("{:?}", machine.memory);
    machine
}