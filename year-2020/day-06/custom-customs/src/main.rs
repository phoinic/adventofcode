use std::collections::HashMap;

static INPUT_DATA: &str = include_str!("input.txt");
struct Group {
    members_count: usize,
    answers: HashMap<char, usize>
}

fn main() {
    
    let mut groups: Vec<Group> = vec![];

    let mut group = Group{
        members_count: 0,
        answers: HashMap::new()
    };

    for str in INPUT_DATA.split("\n") {

        if str == "" {
            groups.push(group);
            group = Group{
                members_count: 0,
                answers: HashMap::new()
            };
        } else {
            for ch in str.chars() {
                match group.answers.get(&ch) {
                    Some(val) => { group.answers.insert(ch, val + 1); },
                    None      => { group.answers.insert(ch, 1); },
                }
            }
            group.members_count += 1;
        }
    }

    groups.push(group);

    let mut answers_count = 0;
    let mut answers_count_similar = 0;

    for group in groups {
        answers_count += group.answers.len();
        answers_count_similar += group.answers.values().filter(|&&v| { v == group.members_count }).collect::<Vec<&usize>>().len();
    }

    println!("{}", answers_count);
    println!("{}", answers_count_similar);
}
