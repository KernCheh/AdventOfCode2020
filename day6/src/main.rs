use std::collections::HashMap;

fn main() {
    let lines = read_input::read_input_file_as_lines("input.txt");
    let mut groups: Vec<HashMap<char, i32>> = Vec::new();
    let mut group_sizes: Vec<i32> = Vec::new();

    let mut group_answers: HashMap<char, i32> = HashMap::new();
    let mut ans_count = 0;
    let mut group_size = 0;

    for line in lines {
        let lstr = line.unwrap();
        if lstr != "" {
            group_size += 1;
            for character in lstr.chars() {
                *group_answers.entry(character).or_insert(0) += 1;
            }
        } else {
            ans_count += group_answers.len();
            groups.push(group_answers);
            group_sizes.push(group_size);
            group_answers = HashMap::new();
            group_size = 0;
        }
    }
    ans_count += group_answers.len();
    groups.push(group_answers);
    group_sizes.push(group_size);

    // println!("{:?}", groups);
    println!("Answer count {}", ans_count);

    // println!("{:?}", group_sizes);

    ans_count = 0;
    for x in 0..groups.len() {
        for item in &groups[x] {
            // println!("{:?}", item);

            // If everyone in the group answered yes to this question,
            // in other words, if number of "yes" for this question matches the size of the group
            if *item.1 == group_sizes[x] {
                ans_count += 1;
            }
        }
    }
    println!("Answer count part 2: {}", ans_count);
}
