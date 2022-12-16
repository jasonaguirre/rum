use std::env;

mod rumload;
mod instruct;
mod init;

fn main() {
    let input = env::args().nth(1);
    let mut machine = init::initialize(input.clone());
    while machine.program_counter < machine.memory[0].len() as u32 {
        let instruction = machine.memory[0][machine.program_counter as usize];
        instruct::perform(instruction, &mut machine)
    }
}
