use std::{cell::RefCell, fmt::Debug, rc::Rc};

use anyhow::Result;
use pest_consume::{match_nodes, Parser};

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

#[derive(Parser)]
#[grammar = "pegs/day07.pest"]
struct Day07Parser;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type PResult<T> = std::result::Result<T, pest_consume::Error<Rule>>;

#[pest_consume::parser]
impl Day07Parser {
    fn file(input: Node) -> PResult<Parsed> {
        Ok(match_nodes!(input.into_children();
            [prompts(prompts), EOI(_)] => prompts
        ))
    }

    fn prompts(input: Node) -> PResult<Parsed> {
        Ok(match_nodes!(input.into_children();
            [prompt(prompt)..] => prompt.collect()
        ))
    }

    fn prompt(input: Node) -> PResult<Prompt> {
        Ok(match_nodes!(input.into_children();
            [command(prompt)] => prompt,
            [entry(prompt)] => prompt
        ))
    }

    fn entry(input: Node) -> PResult<Prompt> {
        Ok(match_nodes!(input.into_children();
            [ent_dir(ent)] => Prompt::Entry(ent),
            [ent_file(ent)] => Prompt::Entry(ent)
        ))
    }

    fn ent_dir(input: Node) -> PResult<Entry> {
        Ok(match_nodes!(input.into_children();
            [dir_name(name)] => Entry::Directory(name)
        ))
    }

    fn ent_file(input: Node) -> PResult<Entry> {
        Ok(match_nodes!(input.into_children();
            [file_size(size), file_name(name)] => Entry::File(size, name)
        ))
    }

    fn command(input: Node) -> PResult<Prompt> {
        Ok(match_nodes!(input.into_children();
            [cmd_chg_dir(cmd)] => Prompt::Command(cmd),
            [cmd_ls_dir(cmd)] => Prompt::Command(cmd),
        ))
    }

    fn cmd_ls_dir(_input: Node) -> PResult<Command> {
        Ok(Command::ListDirectory)
    }

    fn cmd_chg_dir(input: Node) -> PResult<Command> {
        Ok(match_nodes!(input.into_children();
            [cmd_chg_dir_up(dir)] => Command::ChangeDirectory(dir),
            [cmd_chg_dir_root(dir)] => Command::ChangeDirectory(dir),
            [dir_name(dir)] => Command::ChangeDirectory(Directory::Specific(dir)),
        ))
    }

    fn cmd_chg_dir_root(_input: Node) -> PResult<Directory> {
        Ok(Directory::Root)
    }

    fn cmd_chg_dir_up(_input: Node) -> PResult<Directory> {
        Ok(Directory::Up)
    }

    fn dir_name(input: Node) -> PResult<&str> {
        Ok(input.as_str())
    }

    fn file_name(input: Node) -> PResult<&str> {
        Ok(input.as_str())
    }

    fn file_size(input: Node) -> PResult<usize> {
        input.as_str().parse().map_err(|e| input.error(e))
    }

    fn EOI(_input: Node) -> PResult<()> {
        Ok(())
    }
}

fn parse(input: &str) -> Result<Parsed> {
    let inputs = Day07Parser::parse(Rule::file, input)?;
    let input = inputs.single()?;
    Day07Parser::file(input).map_err(|e| e.into())
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
