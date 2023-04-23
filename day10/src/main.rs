use std::{fs, str::FromStr};

type Parsed = Vec<Instruction>;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }
        let parts: Vec<_> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err("Invalid instruction".to_string());
        }
        match (parts[0], parts[1]) {
            ("addx", value) => Ok(Instruction::AddX(i32::from_str(value).map_err(|_| "Invlid param".to_string())?)),
            _ => Err("Invalid instruction".to_string()),
        }
    }
}

#[derive(Debug)]
struct State {
    x: i32,
    cycle: i32,
}

type Sceen = Vec<Vec<char>>;

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }

    fn apply(&self, state: State) -> State {
        let cycle = state.cycle + self.cycles();
        match self {
            Instruction::Noop => State{x: state.x, cycle},
            Instruction::AddX(value) => State { x: state.x + value, cycle},
        }
    }
}

impl State {
    fn strength(&self) -> i32 {
        self.x * self.cycle
    }
}

fn get_state_after_cycles(instructions: &Parsed, cycles: i32) -> State {
    let mut state = State{x: 1, cycle: 1};

    for instruction in instructions {
        let i_cycles = instruction.cycles();
        if state.cycle + i_cycles > cycles {
            return State{x: state.x, cycle: cycles};
        }
        state = instruction.apply(state);
    }

    return state;
}

fn print_screen(screen: &Sceen) {
    for line in screen {
        let s: String = line.into_iter().collect();
        println!("{}", s);
    }
}

fn summarize_cycles(instructions: &Parsed, cycles: Vec<i32>) -> i32 {
    let mut sum = 0;
    for cycle in cycles {
        let strength = get_state_after_cycles(instructions, cycle).strength();
        println!("{}: {}", cycle, strength);
        sum += strength;
    }
    return sum;
}

fn display(instructions: &Parsed) {
    let mut screen = vec![vec!['.'; 40]; 6];

    let mut cycle = 1;
    for row in 0..6 {
        for col in 0..40u16 {
            let state = get_state_after_cycles(instructions, cycle);
            let icol = i32::from(col);
            if icol >= state.x-1 && icol <= state.x+1 {
                screen[row][usize::from(col)] = '#';
            }

            cycle += 1;
        }
    }

    print_screen(&screen);
}

fn parse(content: &String) -> Parsed {
    let mut instructions: Parsed = Vec::new();
    for line in content.split("\n") {
        instructions.push(Instruction::from_str(line).expect("Failed to parse line"))
    }
    return instructions;
}

fn part1(root: &Parsed) {
    //println!("{:?}", root);
    println!("Part 1: {}", summarize_cycles(root, vec![20, 60, 100, 140, 180, 220]));
}

fn part2(root: &Parsed) {
    println!("Part 2: {}", "TODO");
    display(root);
}

fn main() {
    let files = vec!["sample.txt", "input.txt" ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}