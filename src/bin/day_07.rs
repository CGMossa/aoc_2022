use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct DirSizes {
    inner: HashMap<String, usize>,
}

impl DirSizes {
    fn from_input(input: &str) -> Self {
        let input = input.lines();
        // skip "$ cd /" and "$ ls" lines
        let input = input.skip(2);
        let mut dir_sizes = HashMap::new();
        dir_sizes.insert("/".to_string(), 0);

        let mut current_directories = vec!["/".to_string()];
        for line in input {
            let elements = line.split(' ').collect_vec();
            match elements.as_slice() {
                ["dir", _directory_name] => {
                    // skip for now
                }
                ["$", "ls"] => {
                    //skip for now
                }
                ["$", "cd", ".."] => {
                    current_directories.pop();
                }
                ["$", "cd", new_dir] => {
                    let full_directory_name = current_directories[1..].join("");
                    let full_directory_name = format!("{}/{}", full_directory_name, new_dir);
                    dir_sizes.insert(full_directory_name.clone(), 0);
                    current_directories.push(full_directory_name);
                }
                [file_size, _file_name] => {
                    let file_size: usize = file_size.parse().unwrap();
                    current_directories.iter().for_each(|dir| {
                        *dir_sizes.get_mut(dir).unwrap() += file_size;
                    });
                }
                _ => {
                    unreachable!()
                }
            }
        }
        Self { inner: dir_sizes }
    }
}

fn main() {
    let input1 = "$ cd /
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
7214296 k";
    let example_dir_sizes = DirSizes::from_input(input1);
    assert_eq!(one_star(&example_dir_sizes), 95437);

    let input = include_str!("../../input/day_07.txt");
    let dir_sizes = DirSizes::from_input(input);
    println!("Answer: {}", one_star(&dir_sizes));
    println!("Answer: {}", two_star(&dir_sizes));
}

fn one_star(dir_sizes: &DirSizes) -> usize {
    dir_sizes.inner.values().filter(|x| **x <= 100_000).sum()
}

fn two_star(dir_sizes: &DirSizes) -> usize {
    let total_disk_size = 70000000;
    let necessary_disk_space = 30000000;
    let currently_available_disk_space = total_disk_size - dir_sizes.inner.get("/").unwrap();

    let clear_atleast = necessary_disk_space - currently_available_disk_space;
    // dbg!(currently_available_disk_space, clear_atleast);

    dir_sizes
        .inner
        .values()
        .filter(|x| **x >= clear_atleast)
        .min()
        .copied()
        .unwrap()
}
