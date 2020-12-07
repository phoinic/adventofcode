use regex::Regex;
use std::collections::HashMap;

static INPUT_DATA: &str = include_str!("input.txt");

fn collect_parents(bags_map: &HashMap<String, Vec<String>>, bag: String) -> Vec<String> {
    let mut rv: Vec<String> = vec![];
    if bags_map.contains_key(&bag) {
        for parent in &bags_map[&bag] {
            rv.push((*parent).clone());
            for p_parent in collect_parents(bags_map, (*parent).clone()) {
                if !rv.contains(&p_parent) {
                    rv.push(p_parent);
                }
            }
        }    
    }
    rv
}

fn count_children(bags_tree: &HashMap<String, HashMap<String, u32>>, bag: String) -> u32 {
    let mut rv = 0;
    if bags_tree.contains_key(&bag) {
        for (child_bag, count) in bags_tree.get(&bag).unwrap().iter() {
            rv += count;
            rv += count_children(bags_tree, child_bag.clone()) * count;
        }
    }
    rv
}

fn main() {
    
    let mut bags_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut bags_tree: HashMap<String, HashMap<String, u32>> = HashMap::new();
    let bag_re = Regex::new(r"^\s*(?P<n>[0-9]+)?\s*(?P<c>[a-z]+\s[a-z]+)\sbag[s]{0,1}[\.]{0,1}\s*$").unwrap();

    for str in INPUT_DATA.split("\n") {
        let relations = str.split("contain").collect::<Vec<&str>>();
        let parent_bag = bag_re.replace_all(relations[0], "$c").into_owned();
        let bags = relations[1].split(", ").map(|v| bag_re.replace_all(v, "$c:$n").into_owned()).collect::<Vec<String>>();
        if !bags_tree.contains_key(&parent_bag) {
            bags_tree.insert(parent_bag.clone(), HashMap::new());
        }
        let children = bags_tree.get_mut(&parent_bag).unwrap();
        for bag_pair in bags.iter() {
            let mut bag_pair = bag_pair.split(":");
            let bag = bag_pair.next().unwrap().to_string();
            let count = match bag_pair.next().unwrap().parse::<u32>() {
                Ok(v) => v,
                Err(_) => 0
            };
            if bag != "no other" {
                if !children.contains_key(&bag) {
                    children.insert(bag.clone(), count);
                }
                if !bags_map.contains_key(&bag) {
                    bags_map.insert(bag.clone(), vec![]);
                }
                let parents = bags_map.get_mut(&bag).unwrap();
                if !parents.contains(&parent_bag) {
                    parents.push(parent_bag.clone());
                }
            }
        }
    }

    let bag = "shiny gold".to_string();
    let all_parents = collect_parents(&bags_map, bag.clone());
    let count_children = count_children(&bags_tree, bag.clone());

    println!("{}", all_parents.len());
    println!("{}", count_children);
}

