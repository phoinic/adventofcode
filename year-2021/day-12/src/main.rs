static INPUT_DATA: &str = include_str!("input.txt");

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum VisitLogic {
    SmallOnce,
    OneSmallTwice,
}

#[derive(Debug)]
struct CavesMap<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
    pathes: Vec<Vec<&'a str>>,
    visit_logic: VisitLogic,
}

impl<'a> CavesMap<'a> {
    fn new() -> Self {
        CavesMap {
            map: HashMap::<&str, Vec<&str>>::new(),
            pathes: vec![],
            visit_logic: VisitLogic::SmallOnce,
        }
    }

    fn add_tunnel(&mut self, from_cave: &'a str, to_cave: &'a str) {
        if !self.map.contains_key(from_cave) {
            self.map.insert(from_cave, vec![]);
        }
        if !self.map.contains_key(to_cave) {
            self.map.insert(to_cave, vec![]);
        }
        self.map.get_mut(from_cave).unwrap().push(to_cave);
        self.map.get_mut(to_cave).unwrap().push(from_cave);
    }

    fn cave_is_small(cave: &str) -> bool {
        let re = Regex::new("^[a-z]+$").unwrap();
        re.is_match(cave)
    }

    fn build_pathes(&mut self, visit_logic: VisitLogic) {
        self.pathes = vec![];
        self.visit_logic = visit_logic;
        self.find_path("start", vec![], false);
    }

    fn find_path(&mut self, cave: &'a str, visited: Vec<&'a str>, has_small_visited: bool) {
        let mut visited = visited;
        if cave == "end" {
            visited.push(cave);
            self.pathes.push(visited);
        } else {
            let tunnels = self.map.get(cave).unwrap().clone();
            for to_cave in tunnels.iter() {
                let is_small = CavesMap::cave_is_small(to_cave);
                let is_visisted = visited.contains(&&to_cave);
                if to_cave != &"start"
                    && (!is_small
                        || !is_visisted
                        || (self.visit_logic == VisitLogic::OneSmallTwice && !has_small_visited))
                {
                    visited.push(cave);
                    self.find_path(
                        to_cave,
                        visited.clone(),
                        has_small_visited || (is_small && is_visisted),
                    );
                }
            }
        }
    }

    fn get_pathes(&self) -> &Vec<Vec<&str>> {
        &self.pathes
    }
}

fn main() {
    let mut caves_map = CavesMap::new();

    for line in INPUT_DATA.split("\n").into_iter() {
        let mut tmp = line.split("-");
        let from_cave = tmp.next().unwrap();
        let to_cave = tmp.next().unwrap();
        caves_map.add_tunnel(from_cave, to_cave);
    }

    caves_map.build_pathes(VisitLogic::SmallOnce);
    println!("Round 1: {}", caves_map.get_pathes().len());

    caves_map.build_pathes(VisitLogic::OneSmallTwice);
    println!("Round 2: {}", caves_map.get_pathes().len());
}
