use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::str::FromStr;

type DirectoryRef = Rc<RefCell<Directory>>;

pub trait Display {
    fn display(&self, indent: usize) -> ();
}

pub trait CaluclateSize {
    fn calc_size(&self) -> usize;
    fn calc_size_max(&self, max: usize) -> usize;
    fn get_closest_to(&self, current_closest: usize, needed: usize) -> usize;
}

struct EntryParseError {}

struct Directory {
    name: String,
    entries: HashMap<String, Entry>,
    parent: Option<Rc<RefCell<Directory>>>,
}

fn resolve_closest(this: usize, current_closest: usize, needed: usize) -> usize {
    if current_closest == 0 {
        return this;
    }

    if this < needed || this > current_closest {
        return current_closest;
    }
    
    return this;
}

impl Display for Directory {
    fn display(&self, indent: usize) -> () {
        println!("{:indent$}{} {} {}", "", self.name, "<dir>", self.calc_size(), indent=indent);
        for entry in self.entries.values() {
            entry.display(indent + 2);
        }
    }
}

impl CaluclateSize for Directory {
    fn calc_size(&self) -> usize {
        let mut dir_size = 0;
        for entry in self.entries.values() {
            dir_size += entry.calc_size();
        }
        return dir_size;
    }

    fn calc_size_max(&self, max: usize) -> usize {
        let mut size = 0;
        let this_size = self.calc_size();
        if this_size <= max {
            size += this_size;
        }

        for entry in self.entries.values() {
            size += entry.calc_size_max(max);
        }

        return size;
    }

    fn get_closest_to(&self, current_closest: usize, needed: usize) -> usize {
        let size = self.calc_size();
        let mut new_closest = resolve_closest(size, current_closest, needed);

        for entry in self.entries.values() {
            new_closest = entry.get_closest_to(new_closest, needed);
        }

        return new_closest;
    }
}

#[derive(Clone)]
struct File {
    name: String,
    size: usize,
}

impl Display for File {
    fn display(&self, indent: usize) -> () {
        println!("{:indent$}{} {}", "", self.name, self.size, indent=indent);
    }
}

impl CaluclateSize for File {
    fn calc_size(&self) -> usize {
        return self.size
    }

    fn calc_size_max(&self, _max: usize) -> usize {
        return 0;
    }

    fn get_closest_to(&self, current_closest: usize, _needed: usize) -> usize {
        return current_closest;
    }
}

enum Entry {
    Directory(DirectoryRef),
    File(File),
}

impl Display for Entry {
    fn display(&self, indent: usize) -> () {
        match self {
            Entry::Directory(dir) => dir.borrow_mut().display(indent),
            Entry::File(file) => file.display(indent),
        }
    }
}

impl CaluclateSize for Entry {
    fn calc_size(&self) -> usize {
        return match self {
            Entry::Directory(dir) => dir.borrow_mut().calc_size(),
            Entry::File(file) => file.calc_size(),
        }        
    }

    fn calc_size_max(&self, max: usize) -> usize {
        return match self {
            Entry::Directory(dir) => dir.borrow_mut().calc_size_max(max),
            Entry::File(_) => 0,
        }                
    }

    fn get_closest_to(&self, current_closest: usize, needed: usize) -> usize {
        return match self {
            Entry::Directory(dir) => dir.borrow().get_closest_to(current_closest, needed),
            Entry::File(_) => current_closest,
        }                        
    }
}

impl FromStr for Entry {
    type Err = EntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(EntryParseError {});
        }
        if parts[0] == "dir" {
            return Ok(Entry::Directory(Rc::new(RefCell::new(Directory {
                name: parts[1].to_string(),
                entries: HashMap::new(),
                parent: Option::None,
            }))));
        }
        return usize::from_str(parts[0]).map_or_else(
            |_e| Err(EntryParseError {}),
            |size| {
                Ok(Entry::File(File {
                    name: parts[1].to_string(),
                    size,
                }))
            },
        );
    }
}

enum Command {
    GoRoot,
    Ls,
    GoDown,
    GoUp(String),
}

impl FromStr for Command {
    type Err = EntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        let count = parts.len();
        if count < 2 || count > 3 || parts[0] != "$" {
            return Err(EntryParseError {});
        }

        // println!("{:?}", parts);

        return match parts[1] {
            "cd" => match parts[2] {
                "/" => Ok(Command::GoRoot),
                ".." => Ok(Command::GoDown),
                _ => Ok(Command::GoUp(parts[2].to_string())),
            },
            "ls" => Ok(Command::Ls),
            _ => Err(EntryParseError {}),
        };
    }
}

#[derive(PartialEq)]
enum State {
    Command,
    DirList,
}

fn part1(hd: &DirectoryRef) {
    println!("Part 1: {}", hd.borrow_mut().calc_size_max(100000));
    //hd.borrow_mut().display(0);
}

fn part2(hd: &DirectoryRef) {
    let free_space = 70_000_000 - hd.borrow().calc_size();
    let needed = 30_000_000 - free_space;
    println!("Part 2: {} {} {}", free_space, needed, hd.borrow().get_closest_to(0, needed));
}

fn parse(content: &String) -> DirectoryRef {
    let root = Rc::new(RefCell::new(Directory {
        name: "/".to_string(),
        entries: HashMap::new(),
        parent: Option::None,
    }));
    let mut current = Rc::clone(&root);
    let mut state = State::Command;

    for line in content.split("\n") {
        if state == State::DirList {
            match Entry::from_str(line) {
                Ok(entry) => {
                    match entry {
                        Entry::Directory(dir) => {
                            let mut current_mut = current.borrow_mut();
                            dir.borrow_mut().parent = Some(current.clone());
                            current_mut.entries.insert(
                                dir.borrow().name.clone(),
                                Entry::Directory(Rc::clone(&dir)),
                            );
                        }
                        Entry::File(file) => {
                            current
                                .borrow_mut()
                                .entries
                                .insert(file.name.clone(), Entry::File(file));
                        }
                    }
                    continue;
                }
                _ => state = State::Command,
            }
        }
        match Command::from_str(line) {
            Ok(command) => match command {
                Command::GoRoot => continue,
                Command::Ls => state = State::DirList,
                Command::GoDown => {
                    let mut current_temp: Option<DirectoryRef> = None;
                    {
                        if let Some(parent) = &current.borrow().parent {
                            current_temp = Some(Rc::clone(parent));
                        }
                    }
                    match current_temp {
                        Some(new_current) => current = new_current,
                        _ => todo!("Missing Partent"),
                    }
                }
                Command::GoUp(to) => {
                    let current_temp: Option<DirectoryRef>;
                    {
                        let borrowed = current.borrow();
                        let child = borrowed.entries.get(&to);
                        match child {
                            Some(entry) => match entry {
                                Entry::Directory(dir) => {
                                    current_temp = Some(Rc::clone(dir));
                                }
                                _ => todo!("Trying to cd into file"),
                            },
                            _ => todo!("Missing folder {}", to),
                        }
                    }
                    match current_temp {
                        Some(new_current) => current = new_current,
                        _ => (),
                    }
                }
            },
            _ => todo!("Unexpected line: {}", line),
        }
    }

    return root;
}

fn main() {
    let files = vec!["sample.txt" , "input.txt"];
    for file in files {
        println!("Reading {}", file);
        let content = fs::read_to_string(file).expect("Cannot read file");
        let root = parse(&content);
        part1(&root);
        part2(&root);
    }
}
