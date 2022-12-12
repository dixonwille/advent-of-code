use std::{cell::RefCell, fmt::Debug, rc::Rc};

use anyhow::{Ok, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{alpha1, char as character, digit1, line_ending},
    combinator::{map, map_res},
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

const INPUT: &str = include_str!("inputs/day07.txt");

pub fn run_part_a() -> Result<()> {
    let i = build(parse(INPUT)?);
    println!("{}", part_a(i)?);
    Ok(())
}

pub fn run_part_b() -> Result<()> {
    let i = build(parse(INPUT)?);
    println!("{}", part_b(i)?);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    ChangeDirectory(Directory<'a>),
    ListDirectory,
}

#[derive(Debug, PartialEq)]
enum Directory<'a> {
    Root,
    Up,
    Specific(&'a str),
}

#[derive(Debug, PartialEq)]
enum Entry<'a> {
    Directory(&'a str),
    File(usize, &'a str),
}

#[derive(Debug, PartialEq)]
enum Prompt<'a> {
    Command(Command<'a>),
    Entry(Entry<'a>),
}

#[derive(PartialEq, Clone)]
struct Tree {
    name: String,
    parent: Option<Rc<RefCell<Tree>>>,
    children: Vec<Rc<RefCell<Tree>>>,
    size: Option<usize>,
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size.is_some() {
            writeln!(f, "- {} (file, size={})", self.name, self.size.unwrap())?;
        } else {
            writeln!(f, "- {} (dir)", self.name)?;
        }
        for c in &self.children {
            let fmt = format!("{:?}", RefCell::borrow(c));
            for l in fmt.lines() {
                writeln!(f, "  {}", l)?;
            }
        }
        std::result::Result::Ok(())
    }
}

impl Tree {
    fn get_size(&self) -> usize {
        if let Some(s) = self.size {
            return s;
        }
        self.children
            .iter()
            .map(|c| {
                let bor = c.borrow();
                bor.get_size()
            })
            .sum()
    }

    fn list_all_directories(&self) -> Vec<Rc<RefCell<Tree>>> {
        self.children
            .iter()
            .filter_map(|c| {
                let bor = c.borrow();
                if bor.size.is_some() {
                    return None;
                }
                let mut v = vec![Rc::clone(c)];
                v.append(&mut bor.list_all_directories());
                Some(v)
            })
            .flatten()
            .collect()
    }
}

type Parsed<'a> = Vec<Prompt<'a>>;
type Built = Tree;

fn command(input: &str) -> IResult<&str, Command> {
    let prompt = terminated(character('$'), character(' '));
    let ls_command = map(tag("ls"), |_| Command::ListDirectory);
    let cd_command = map(
        separated_pair(
            tag("cd"),
            character(' '),
            alt((
                map(tag(".."), |_| Directory::Up),
                map(tag("/"), |_| Directory::Root),
                map(alpha1, |d| Directory::Specific(d)),
            )),
        ),
        |(_, p)| Command::ChangeDirectory(p),
    );
    terminated(preceded(prompt, alt((ls_command, cd_command))), line_ending)(input)
}

fn entry(input: &str) -> IResult<&str, Entry> {
    let dir_entry = map(
        separated_pair(tag("dir"), character(' '), alpha1),
        |(_, p)| Entry::Directory(p),
    );
    let file_entry = map(
        separated_pair(
            map_res(digit1, str::parse),
            character(' '),
            take_until1("\n"),
        ),
        |(size, p)| Entry::File(size, p),
    );
    terminated(alt((dir_entry, file_entry)), line_ending)(input)
}

fn parse(input: &str) -> Result<Parsed> {
    let (_, out) = many1(alt((
        map(command, |c| Prompt::Command(c)),
        map(entry, |e| Prompt::Entry(e)),
    )))(input)
    .map_err(|e| e.to_owned())?;
    Ok(out)
}

fn build(raw: Parsed) -> Built {
    let mut directory = Vec::new();
    for entry in raw {
        match entry {
            Prompt::Command(c) => match c {
                Command::ChangeDirectory(d) => match d {
                    Directory::Root => {
                        if directory.len() == 0 {
                            directory.push(Rc::new(RefCell::new(Tree {
                                name: "/".to_owned(),
                                parent: None,
                                children: Vec::new(),
                                size: None,
                            })));
                        } else {
                            directory = directory.splice(1.., Vec::new()).collect();
                        }
                    }
                    Directory::Up => {
                        directory.pop();
                    }
                    Directory::Specific(s) => {
                        // Grab an reference to the last item in stack
                        let last = directory.pop().unwrap();
                        let last_borrowed = last.borrow();
                        directory.push(Rc::clone(&last));

                        // Check if directory exists in children's array of last item
                        let existing = last_borrowed
                            .children
                            .iter()
                            .find(|e| e.borrow().name == s && e.borrow().size.is_none());
                        match existing {
                            Some(e) => directory.push(Rc::clone(e)), // Push existing item to stack
                            None => unimplemented!(),                // Directory doesn't exist
                        };
                    }
                },
                Command::ListDirectory => (),
            },
            Prompt::Entry(e) => match e {
                Entry::Directory(n) => {
                    // Get a mutable reference to last item in stack
                    let last = directory.pop().unwrap();
                    let mut last_borrowed = last.borrow_mut();
                    directory.push(Rc::clone(&last));

                    // See if directory already exists in children
                    let existing = last_borrowed
                        .children
                        .iter()
                        .find(|e| e.borrow().name == n && e.borrow().size.is_none());

                    match existing {
                        Some(_) => (), // Do nothing if it is already in the array
                        None => last_borrowed.children.push(Rc::new(RefCell::new(Tree {
                            name: n.to_owned(),
                            parent: Some(Rc::clone(&last)),
                            children: Vec::new(),
                            size: None,
                        }))),
                    }
                }
                Entry::File(s, n) => {
                    // Get a mutable reference to last item in stack
                    let last = directory.pop().unwrap();
                    let mut last_borrowed = last.borrow_mut();
                    directory.push(Rc::clone(&last));

                    // See if file already exists in children
                    let existing = last_borrowed
                        .children
                        .iter()
                        .find(|e| e.borrow().name == n && e.borrow().size.is_some());
                    match existing {
                        Some(_) => (), // Do nothing if it is already in the array
                        None => last_borrowed.children.push(Rc::new(RefCell::new(Tree {
                            name: n.to_owned(),
                            parent: Some(Rc::clone(&last)),
                            children: Vec::new(),
                            size: Some(s),
                        }))),
                    }
                }
            },
        }
    }
    directory.reverse();
    let foo = (*directory.pop().unwrap()).clone().into_inner();
    foo
}

fn part_a(tree: Built) -> Result<usize> {
    Ok(tree
        .list_all_directories()
        .iter()
        .filter_map(|d| {
            let s = d.borrow().get_size();
            if s > 100_000 {
                return None;
            }
            Some(s)
        })
        .sum())
}

const AVAILABLE: usize = 70_000_000;
const NEEDED_UNUSED: usize = 30_000_000;

fn part_b(tree: Built) -> Result<usize> {
    let free = AVAILABLE - tree.get_size();
    let need_to_free = NEEDED_UNUSED - free;
    Ok(tree
        .list_all_directories()
        .iter()
        .filter_map(|d| {
            let s = d.borrow().get_size();
            if s > need_to_free {
                return Some(s);
            }
            None
        })
        .min()
        .unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k

";

    #[test]
    fn test_command_parse() {
        assert_eq!(
            command("$ cd /\n").unwrap(),
            ("", Command::ChangeDirectory(Directory::Root))
        );
        assert_eq!(
            command("$ cd ..\n").unwrap(),
            ("", Command::ChangeDirectory(Directory::Up))
        );
        assert_eq!(
            command("$ cd bqm\n").unwrap(),
            ("", Command::ChangeDirectory(Directory::Specific("bqm")))
        );
        assert_eq!(command("$ ls\n").unwrap(), ("", Command::ListDirectory));
    }

    #[test]
    fn test_entry_parse() {
        assert_eq!(entry("dir bqm\n").unwrap(), ("", Entry::Directory("bqm")));
        assert_eq!(
            entry("1234 bqm.cjj\n").unwrap(),
            ("", Entry::File(1234, "bqm.cjj"))
        );
        assert_eq!(entry("5678 bqm\n").unwrap(), ("", Entry::File(5678, "bqm")));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_INPUT).unwrap(),
            vec![
                Prompt::Command(Command::ChangeDirectory(Directory::Root)),
                Prompt::Command(Command::ListDirectory),
                Prompt::Entry(Entry::Directory("a")),
                Prompt::Entry(Entry::File(14848514, "b.txt")),
                Prompt::Entry(Entry::File(8504156, "c.dat")),
                Prompt::Entry(Entry::Directory("d")),
                Prompt::Command(Command::ChangeDirectory(Directory::Specific("a"))),
                Prompt::Command(Command::ListDirectory),
                Prompt::Entry(Entry::Directory("e")),
                Prompt::Entry(Entry::File(29116, "f")),
                Prompt::Entry(Entry::File(2557, "g")),
                Prompt::Entry(Entry::File(62596, "h.lst")),
                Prompt::Command(Command::ChangeDirectory(Directory::Specific("e"))),
                Prompt::Command(Command::ListDirectory),
                Prompt::Entry(Entry::File(584, "i")),
                Prompt::Command(Command::ChangeDirectory(Directory::Up)),
                Prompt::Command(Command::ChangeDirectory(Directory::Up)),
                Prompt::Command(Command::ChangeDirectory(Directory::Specific("d"))),
                Prompt::Command(Command::ListDirectory),
                Prompt::Entry(Entry::File(4060174, "j")),
                Prompt::Entry(Entry::File(8033020, "d.log")),
                Prompt::Entry(Entry::File(5626152, "d.ext")),
                Prompt::Entry(Entry::File(7214296, "k")),
            ]
        );
    }

    const EXPECTED_FMT: &str = "- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
";

    #[test]
    fn test_build_tree() {
        let parsed = parse(TEST_INPUT).unwrap();
        let tree = build(parsed);
        assert_eq!(format!("{:?}", tree), EXPECTED_FMT);
    }

    #[test]
    fn test_part_a() {
        let built = build(parse(TEST_INPUT).unwrap());
        assert_eq!(part_a(built).unwrap(), 95437)
    }

    #[test]
    fn test_part_b() {
        let built = build(parse(TEST_INPUT).unwrap());
        assert_eq!(part_b(built).unwrap(), 24933642)
    }
}
