static INPUT_DATA: &str = include_str!("input.txt");

use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, PartialEq)]
enum VisitLogic {
    SmallOnce,
    OneSmallTwice,
}

#[derive(Debug)]
struct CavesMap<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
    paths: Vec<Vec<&'a str>>,
    visit_logic: VisitLogic,
}

impl<'a> CavesMap<'a> {
    fn new() -> Self {
        CavesMap {
            map: HashMap::<&str, Vec<&str>>::new(),
            paths: vec![],
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
        if from_cave != "start" && to_cave != "end" {
            self.map.get_mut(to_cave).unwrap().push(from_cave);
        }
    }

    fn cave_is_small(cave: &str) -> bool {
        for ch in cave.as_bytes() {
            if !ch.is_ascii_lowercase() {
                return false;
            }
        }
        true
    }

    fn build_paths(&mut self, visit_logic: VisitLogic) {
        self.visit_logic = visit_logic;
        self.paths = self.find_path("start", vec![], false);
    }

    fn find_path(
        &self,
        cave: &'a str,
        visited: Vec<&'a str>,
        has_small_visited: bool,
    ) -> Vec<Vec<&'a str>> {
        let mut visited = visited.clone();
        visited.push(cave);
        if cave == "end" {
            vec![visited]
        } else {
            let tunnels = self.map.get(cave).unwrap().clone();
            let new_pathes = tunnels
                .par_iter()
                .map(|to_cave| {
                    let is_small = Self::cave_is_small(to_cave);
                    let is_visited = visited.contains(&to_cave);
                    if to_cave != &"start"
                        && (!is_small
                            || !is_visited
                            || (self.visit_logic == VisitLogic::OneSmallTwice
                                && !has_small_visited))
                    {
                        self.find_path(
                            to_cave,
                            visited.clone(),
                            has_small_visited || (is_small && is_visited),
                        )
                    } else {
                        vec![]
                    }
                })
                .flatten()
                .collect::<Vec<_>>();
            new_pathes
        }
    }

    fn get_pathes(&self) -> &Vec<Vec<&str>> {
        &self.paths
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

    let tstamp = Instant::now();
    caves_map.build_paths(VisitLogic::SmallOnce);
    println!(
        "Round 1: {} ({}ms)",
        caves_map.get_pathes().len(),
        tstamp.elapsed().as_millis()
    );

    let tstamp = Instant::now();
    caves_map.build_paths(VisitLogic::OneSmallTwice);
    println!(
        "Round 2: {} ({}ms)",
        caves_map.get_pathes().len(),
        tstamp.elapsed().as_millis()
    );
}
