use std::{collections::HashMap, fs, num::ParseIntError, str::FromStr};

type ParseError = String;
type TailMap = HashMap<Position, bool>;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Position(i32, i32);

#[derive(Debug)]
enum Direction {
    L,
    U,
    R,
    D,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "L" => Ok(Direction::L),
            "U" => Ok(Direction::U),
            "R" => Ok(Direction::R),
            "D" => Ok(Direction::D),
            _ => Err("Invalid Direction".to_string()),
        };
    }
}

#[derive(Debug)]
enum InstructionParseError {
    Dir(ParseError),
    Amount(ParseIntError),
}

impl From<ParseError> for InstructionParseError {
    fn from(value: ParseError) -> Self {
        InstructionParseError::Dir(value)
    }
}

impl From<ParseIntError> for InstructionParseError {
    fn from(value: ParseIntError) -> Self {
        InstructionParseError::Amount(value)
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(InstructionParseError::Dir(
                "Invalid Instruction".to_string(),
            ));
        }

        let direction = Direction::from_str(parts[0])?;
        let amount = i32::from_str(parts[1])?;

        return Ok(Instruction { direction, amount });
    }
}

fn parse(content: &String) -> Vec<Instruction> {
    let mut result = Vec::new();

    for line in content.split("\n") {
        result.push(Instruction::from_str(line).expect("Failed to parse line"))
    }

    return result;
}

fn move_head(head: &mut Position, direction: &Direction) {
    match direction {
        Direction::L => head.0 -= 1,
        Direction::U => head.1 += 1,
        Direction::R => head.0 += 1,
        Direction::D => head.1 -= 1,
    }
}

fn move_cord(visited: &mut TailMap, head: &Position, tail: &mut Position, track: bool) {
    let mut changed = false;

    if (head.0 - tail.0).abs() >= 2 {
        if head.0 > tail.0 {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }

        if head.1 != tail.1 {
            tail.1 = head.1
        }

        changed = true;
    }

    if (head.1 - tail.1).abs() >= 2 {
        if head.1 > tail.1 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }

        if head.0 != tail.0 {
            tail.0 = head.0
        }

        changed = true;
    }

    if changed && track {
        visited.insert(tail.clone(), true);
    }
}

fn find_rope_positions(instructions: &Vec<Instruction>) -> TailMap {
    let mut rope = vec![Position(0, 0); 10];
    // println!("{:?}", rope);
    let mut tail_visited: TailMap = HashMap::new();
    tail_visited.insert(Position(0, 0), true);

    // let mut i = 0;
    for instruction in instructions {
        println!("{:?}", instruction);
        for _ in 0..instruction.amount {
            move_head(&mut rope[0], &instruction.direction);
            for pos in 1..rope.len() {
                let prev_pos = rope[pos - 1].clone();
                move_cord(&mut tail_visited, &prev_pos, &mut rope[pos], pos == 9);
            }
            // println!("{:?}", rope);
        }
        // if i > 2 {
            // break;
        // }
        // i += 1;
    }

    return tail_visited;
}

fn find_tail_positions(instructions: &Vec<Instruction>) -> TailMap {
    let mut head = Position(0, 0);
    let mut tail = Position(0, 0);
    let mut tail_visited: TailMap = HashMap::new();
    tail_visited.insert(Position(0, 0), true);

    for instruction in instructions {
        for _ in 0..instruction.amount {
            move_head(&mut head, &instruction.direction);
            move_cord(&mut tail_visited, &head, &mut tail, true);
        }
    }

    return tail_visited;
}

fn part1(instructions: &Vec<Instruction>) {
    //println!("{:?}", instructions);
    let visited = find_tail_positions(instructions);
    println!("Part 1: {}", visited.len());
}

fn part2(instructions: &Vec<Instruction>) {
    let visited = find_rope_positions(instructions);
    println!("Part 2: {} (Warning, may be wrong)", visited.len());
}

fn main() {
    let files = vec!["sample.txt", "sample2.txt" , "input.txt" ];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
