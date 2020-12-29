mod read_input;

#[derive(Debug)]
struct BiomeRow {
    biome: Vec<bool>,
}

impl BiomeRow {
    fn is_tree(&self, i: usize) -> bool {
        let index = i % self.biome.len();
        self.biome[index]
    }

    /// Loads data into the `BiomeRow`.
    /// Using a mutable struct is unnecessary practice, it would be better to define this functionally.
    /// Yet we are doing this just for the sake of experimenting on implementing methods tied to structs.
    fn load(&mut self, line: Vec<char>) {
        for character in line {
            match character {
                '.' => self.biome.push(false),
                '#' => self.biome.push(true),
                _ => panic!("unknown char"),
            }
        }
    }
}

fn get_trees_count(biome_rows: &Vec<BiomeRow>, x_inc: usize, y_inc: usize) -> i32 {
    let mut trees_count = 0;
    let (mut x_index, mut y_index) = (x_inc, y_inc);
    while y_index < biome_rows.len() {
        if biome_rows[y_index].is_tree(x_index) {
            trees_count += 1;
        }
        x_index += x_inc;
        y_index += y_inc;
    }

    trees_count
}

fn main() {
    let lines = read_input::read_input_file_as_lines("input.txt");
    let mut biome_rows = Vec::<BiomeRow>::new();
    for line in lines {
        let ll = line.unwrap();
        let mut biome_row = BiomeRow { biome: Vec::new() };
        biome_row.load(ll.chars().collect::<Vec<char>>());
        biome_rows.push(biome_row);
    }

    println!("Part 1: {}", get_trees_count(&biome_rows, 3, 1));
    println!(
        "Part 2: {}",
        get_trees_count(&biome_rows, 1, 1)
            * get_trees_count(&biome_rows, 3, 1)
            * get_trees_count(&biome_rows, 5, 1)
            * get_trees_count(&biome_rows, 7, 1)
            * get_trees_count(&biome_rows, 1, 2)
    );
}
