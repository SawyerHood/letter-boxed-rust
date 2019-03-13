use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str;

fn main() -> io::Result<()> {
    let lines = read_lines("words_alpha.txt");
    let sides: Vec<HashSet<_>> = [
        ['a', 's', 'c'],
        ['k', 't', 'o'],
        ['l', 'r', 'i'],
        ['u', 'n', 'h'],
    ]
    .iter()
    .cloned()
    .map(|x| -> HashSet<_> { x.iter().cloned().map(|string| string.to_string()).collect() })
    .collect();

    let all_chars = sides
        .iter()
        .cloned()
        .fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect());

    let words = get_valid_words(&lines, &sides);
    let map = words_by_first_letter(&words);

    words.par_iter().for_each(|word1| {
        let last = word1.chars().last().unwrap().to_string();
        if let Some(vec) = map.get(&last) {
            for word2 in vec {
                let word1_set: HashSet<_> = word1.chars().map(|c| c.to_string()).collect();
                let word2_set: HashSet<_> = word2.chars().map(|c| c.to_string()).collect();

                let set: HashSet<_> = all_chars.difference(&word1_set).cloned().collect();
                let out: HashSet<_> = set.difference(&word2_set).collect();
                if out.is_empty() {
                    println!("{} {}", word1, word2)
                }
            }
        }
    });

    Ok(())
}

fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|str| str.unwrap()).collect()
}

fn get_valid_words<'a>(lines: &Vec<String>, sides: &Vec<HashSet<String>>) -> Vec<String> {
    lines
        .par_iter()
        .filter_map(|string| {
            let mut last_side = 12;
            for char in string.chars() {
                let mut found = false;
                for (i, side) in sides.iter().enumerate() {
                    if side.contains(&(char.to_string())) && i != last_side {
                        found = true;
                        last_side = i;
                        break;
                    }
                }
                if !found {
                    return None;
                }
            }
            Some(string)
        })
        .cloned()
        .collect()
}

fn words_by_first_letter(words: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for word in words {
        let first = word.chars().next().unwrap().to_string();
        let vec = map.entry(first).or_insert(Vec::new());
        vec.push(word.clone());
    }
    map
}
