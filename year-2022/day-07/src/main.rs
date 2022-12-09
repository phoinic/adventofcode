use regex::Regex;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

static INPUT_DATA: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum FsNodeType {
    Dir,
    File,
}

#[derive(Debug, Clone)]
struct FsNode {
    node_type: FsNodeType,
    name: String,
    size: Option<usize>,
    children_idx: Vec<usize>,
    parent_idx: Option<usize>,
}

impl FsNode {
    fn new(
        node_type: FsNodeType,
        name: &str,
        size: Option<usize>,
        parent_idx: Option<usize>,
    ) -> Self {
        Self {
            node_type,
            name: name.to_owned(),
            size,
            parent_idx,
            children_idx: vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct FsTree {
    nodes: BTreeMap<usize, FsNode>,
    next_idx: usize,
    current_idx: usize,
}

impl FsTree {
    fn init() -> Self {
        FsTree {
            nodes: BTreeMap::from([(0, FsNode::new(FsNodeType::Dir, "/", None, None))]),
            next_idx: 1,
            current_idx: 0,
        }
    }

    fn cd(&mut self, name: &str) {
        match name {
            "/" => self.current_idx = 0,
            ".." => {
                self.current_idx = self
                    .nodes
                    .get(&self.current_idx)
                    .unwrap()
                    .parent_idx
                    .unwrap()
            }
            name => {
                self.current_idx = self
                    .nodes
                    .get(&self.current_idx)
                    .unwrap()
                    .children_idx
                    .iter()
                    .find_map(|&idx| {
                        if self.nodes.get(&idx).unwrap().name == name {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .unwrap();
            }
        }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.nodes.insert(
            self.next_idx,
            FsNode::new(FsNodeType::File, name, Some(size), Some(self.current_idx)),
        );
        self.nodes
            .get_mut(&self.current_idx)
            .unwrap()
            .children_idx
            .push(self.next_idx);
        self.next_idx += 1;
    }

    fn add_dir(&mut self, name: &str) {
        self.nodes.insert(
            self.next_idx,
            FsNode::new(FsNodeType::Dir, name, None, Some(self.current_idx)),
        );
        self.nodes
            .get_mut(&self.current_idx)
            .unwrap()
            .children_idx
            .push(self.next_idx);
        self.next_idx += 1;
    }

    fn get_dir_size(&self) -> usize {
        self.inner_get_dir_size(self.current_idx)
    }

    fn inner_get_dir_size(&self, idx: usize) -> usize {
        let mut res = 0;

        for &child_idx in self.nodes.get(&idx).unwrap().children_idx.iter() {
            let node = self.nodes.get(&child_idx).unwrap();
            match node.node_type {
                FsNodeType::File => res += node.size.unwrap(),
                FsNodeType::Dir => res += self.inner_get_dir_size(child_idx),
            }
        }

        res
    }

    fn find_dir_size_under(&self, under_size: usize) -> Vec<(String, usize)> {
        self.inner_find_dir_size_limit(under_size, -1, self.current_idx)
    }

    fn find_dir_size_above(&self, above_size: usize) -> Vec<(String, usize)> {
        self.inner_find_dir_size_limit(above_size, 1, self.current_idx)
    }

    fn inner_find_dir_size_limit(
        &self,
        limit_size: usize,
        limit_type: i8,
        idx: usize,
    ) -> Vec<(String, usize)> {
        let mut res = vec![];

        let node = self.nodes.get(&idx).unwrap();
        let self_size = self.inner_get_dir_size(idx);

        if (limit_type < 0 && self_size <= limit_size)
            || (limit_type > 0 && self_size >= limit_size)
        {
            res.push((node.name.clone(), self_size));
        }

        for &child_idx in node.children_idx.iter() {
            let child_node = self.nodes.get(&child_idx).unwrap();
            match child_node.node_type {
                FsNodeType::Dir => res
                    .append(&mut self.inner_find_dir_size_limit(limit_size, limit_type, child_idx)),
                _ => {}
            }
        }

        res
    }

    fn fmt_lvl(&self, f: &mut Formatter<'_>, idx: usize, lvl: usize) -> std::fmt::Result {
        let node = self.nodes.get(&idx).unwrap();
        for _ in 0..lvl {
            write!(f, "  ").unwrap();
        }
        match node.node_type {
            FsNodeType::Dir => writeln!(f, "- {} (dir)", node.name).unwrap(),
            FsNodeType::File => {
                writeln!(f, "- {} (file, size={})", node.name, node.size.unwrap()).unwrap()
            }
        };
        for &child_idx in node.children_idx.iter() {
            self.fmt_lvl(f, child_idx, lvl + 1).unwrap();
        }
        Ok(())
    }
}

impl Display for FsTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_lvl(f, 0, 0)
    }
}

fn read_input() -> FsTree {
    let mut tree = FsTree::init();

    for cmd_line in INPUT_DATA.split("\n") {
        let cmd_re = Regex::new(r"^\$ ").unwrap();

        if cmd_re.is_match(cmd_line) {
            let cmd_tokens = cmd_line.split(" ").collect::<Vec<_>>();

            match cmd_tokens[1] {
                "ls" => {}
                "cd" => tree.cd(cmd_tokens[2]),
                _ => {}
            }
        } else {
            let node_tokens = cmd_line.split(" ").collect::<Vec<_>>();

            match node_tokens[0] {
                "dir" => tree.add_dir(node_tokens[1]),
                size_str => tree.add_file(node_tokens[1], size_str.parse::<usize>().unwrap()),
            }
        }
    }

    tree.cd("/");

    tree
}

fn main() {
    let mut fstree = read_input();

    println!("{}", fstree);

    fstree.cd("/");
    let round1 = fstree
        .find_dir_size_under(100000)
        .iter()
        .map(|&(_, size)| size)
        .sum::<usize>();

    println!("Round 1: {}", round1);

    fstree.cd("/");
    let root_size = fstree.get_dir_size();
    let free_space = 70000000 - root_size;
    let need_delete = 30000000 - free_space;

    println!("Total size: {}", root_size);
    println!("Free space: {}", free_space);
    println!("Need delete: {}", need_delete);

    let round2 = fstree
        .find_dir_size_above(need_delete)
        .iter()
        .map(|&(_, size)| size)
        .min()
        .unwrap();

    println!("Round 2: {}", round2);
}
