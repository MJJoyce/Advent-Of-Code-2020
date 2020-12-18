use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Cmd {
    NOP(i64),
    JMP(i64),
    ACC(i64)
}

struct CPU {
    pc: i64,
    acc: i64
}

impl CPU {
    fn new() -> Self {
        CPU {pc: 0, acc: 0}
    }

    fn exec(&mut self, cmd: &Cmd) {
        match cmd {
            Cmd::NOP(_) => self.nop(),
            Cmd::JMP(offset) => self.jmp(*offset),
            Cmd::ACC(add) => self.acc(*add),
            _ => panic!("Invalid CPU Command")
        }
    }

    fn nop(&mut self) {
        self.pc += 1;
    }

    fn jmp(&mut self, offset: i64) {
        self.pc += offset;
    }

    fn acc(&mut self, add: i64) {
        self.pc += 1;
        self.acc += add;
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }
}

fn main() {
    let cmds = load_data();

    part1(&cmds);
    part2(&cmds);
}

fn load_data() -> Vec<Cmd> {
    let br = BufReader::new(File::open("./input/input.txt").unwrap());
    let mut cmds = Vec::new();

    for l in br.lines() {
        let line = l.unwrap();
        let line = line.trim();
        let mut split = line.split(" ");
        let cmd = split.next().unwrap();
        let arg = i64::from_str(split.next().unwrap()).unwrap();

        match cmd {
            "nop" => cmds.push(Cmd::NOP(arg)),
            "jmp" => cmds.push(Cmd::JMP(arg)),
            "acc" => cmds.push(Cmd::ACC(arg)),
            _     => panic!("Invalid command parsed from command stream")
        };
    }

    cmds
}

fn part1(cmds: &Vec<Cmd>) {
    let mut executed_pc: HashSet<i64> = HashSet::new();
    let mut cpu = CPU::new();

    while ! executed_pc.contains(&cpu.pc) {
        executed_pc.insert(cpu.pc);
        cpu.exec(&cmds[cpu.pc as usize]);
    }

    println!("\nPart 1: Acc prior to infinite loop: {}", cpu.acc);
}

// This is a straight forward brute force solution to part 2. Swap JMP / NOP
// commands in order until the program successfully completes
fn part2(cmds: &Vec<Cmd>) {
    let mut executed_pc: HashSet<i64> = HashSet::with_capacity(cmds.len());
    let mut cpu = CPU::new();

    for swap_index in 0..cmds.len() {
        if let Cmd::ACC(_) = cmds[swap_index] {
            continue;
        }

        cpu.reset();
        executed_pc.clear();

        // Stop execution if:
        //   We previously visited a PC (i.e., infinite loop)
        //   We try to run the command immediately after the last cmd
        //      (i.e., successful termination)
        while !executed_pc.contains(&cpu.pc) && cpu.pc as usize != cmds.len() {
            executed_pc.insert(cpu.pc);

            let cmd = &cmds[cpu.pc as usize];
            if cpu.pc as usize == swap_index {
                cpu.exec(&swap(cmd));
            } else {
                cpu.exec(cmd);
            }

        }

        if cpu.pc as usize == cmds.len() {
            println!("\nPart 2: Program terminated with cmd {} swapped. Acc: {}", swap_index, cpu.acc);
            break;
        }
    }
}

fn swap(cmd: &Cmd) -> Cmd {
    match cmd {
        Cmd::JMP(offset) => Cmd::NOP(*offset),
        Cmd::NOP(value) => Cmd::JMP(*value),
        _ => panic!("Swap called on NOP")
    }
}
