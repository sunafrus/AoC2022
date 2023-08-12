use camino::Utf8PathBuf;
use nom::{
    bytes::complete::{tag, take_while1},
    branch::alt,
    combinator::{all_consuming, map},
    IResult,
    sequence::{separated_pair, preceded}, Finish
};
use id_tree::{Node, InsertBehavior, Tree};

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let lines = include_str!("input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut tree = Tree::<FsEntry>::new();
    let root = tree.insert(
        Node::new(FsEntry {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // No action
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // No action: only occurs at start of input
                    }
                    ".." => {
                        curr = tree.get(&curr)?.parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // No action
                }
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry { size, path });
                    tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            },
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{s}");

    let sum_of_small = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|node| !node.children().is_empty())
        .map(|node| total_size(&tree, node).unwrap())
        .filter(|&size| size <= 100_000)
        .inspect(|size| { dbg!(size); })
        .sum::<u64>();

    dbg!(sum_of_small);

    let total_space = 70_000_000_u64;
    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap())?)?;
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_space = 30_000_000_u64;
    let space_to_free = needed_space.checked_sub(free_space).unwrap();

    let remove_size = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|node| total_size(&tree, node).unwrap())
        .filter(|&size| size >= space_to_free)
        .inspect(|size| { dbg!(size); })
        .min();

    dbg!(remove_size);

    Ok(())
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}


#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;

    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }

    Ok(total)
}

