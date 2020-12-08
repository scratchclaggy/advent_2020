fn main() {
    // get instructions
    // seperate into lines
    // split each line around the space
    //
}

enum instruction {
    NOP(fn(ic: &mut u32, acc: &mut i32, var: &mut i32)),
    ACC(fn(ic: &mut u32, acc: &mut i32, var: &mut i32)),
    JMP(fn(ic: &mut u32, acc: &mut i32, var: &mut i32)),
}

impl instruction {
    fn new(instruction_string: &str) -> instruction {
        match instruction_string {
            "acc" => instruction::ACC(acc),
            "jmp" => instruction::JMP(jmp),
            "nop" => instruction::NOP(nop),
            _ => panic!(),
        }
    }
}

fn acc(ic: &mut u32, acc: &mut i32, var: &mut i32) {
    *acc += *var;
    *ic += 1;
}

fn jmp(ic: &mut u32, acc: &mut i32, var: &mut i32) {}

fn nop(ic: &mut u32, acc: &mut i32, var: &mut i32) {}
