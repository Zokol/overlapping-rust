// Title: Overlap Compression
// Author: Heikki Juva
// Date: 2023-08-27
// Purpose: Compress a list of words by overlapping suffixes and prefixes


use core::ops::Range;
use std::{collections::{HashMap, HashSet}, ops::RangeBounds};

const INPUT: [&str; 5] = ["pleasure", "apple", "rendered", "clap", "suren"];

fn main() {
    println!("{:?}", INPUT);
    let input = INPUT.to_vec();
    let output = compress(&input);
    println!("{:?}", output);
    print_compressed(output);
}

fn print_compressed(permutations: Vec<(&str, &str, Range<usize>)>) {
    let mut indentation = 0;
    for permutation in permutations.into_iter() {
        println!("{}{}", " ".repeat(indentation), permutation.0);
        indentation += permutation.2.start;
    }
}

fn compress<'a>(word_list: &'a Vec<&'a str>) -> Vec<(&'a str, &'a str, std::ops::Range<usize>)> {
    let mut output = Vec::new();
    let mut used_words = Vec::new();

    // find all permutations of word1+word2
    let mut permutations = score_overlaps(word_list);

    // find permutation with maximum overlapping range, where the word1 does not exist in any other permutation word2
    // remove permutation from permutations
    // add permutation (word1+word2) to wordlist
    permutations.sort_by(|a, b| b.2.len().cmp(&a.2.len()));

    println!();
    println!("Sorted permutations: {:?}", permutations);

    let first_words : HashSet<_> = permutations.iter().filter(|(_,_,r)| r.len() != 0).map(|x| x.0).collect();
    let last_words : HashSet<_> = permutations.iter().filter(|(_,_,r)| r.len() != 0).map(|x| x.1).collect();

    println!("First: {:?}", first_words);
    println!("Last: {:?}", last_words);

    let possible_first_words : HashSet<_> = first_words.difference(&last_words).collect();

    println!("Possible first words: {:?}", possible_first_words);
    
    // Count the occurance of each suffix word, to select the rarest suffix word which has best overlapping score
    // TODO: Fix this, as the best score is not always best starting word.
    let mut next_word_scores: HashMap<&str, usize> = HashMap::new();
    for permutation in permutations.iter() {
        if possible_first_words.contains(&permutation.0) {
            let next_word_score = next_word_scores.entry(permutation.0).or_insert(0);
            *next_word_score += permutation.2.len();
        }
    }

    println!();
    println!("CHOOSING FIRST WORD: {:?}", next_word_scores);
    let best_starting_word = next_word_scores.iter().max_by_key(|x| x.1).unwrap(); // TODO: What?? Maybe this could be rewritten to be readable
    let mut next_word = best_starting_word.0.clone().clone();
    used_words.push(next_word);

    //let mut rm_range = best_combination..clone().clone();
    //let tmp= &(output.to_owned() + next_word[rm_range.start]); // TODO: this is ugly, there must be a better way
    //output = &tmp;

    // find permutation with maximum overlapping range, where the word1 is the the word2 of previous permutation
    // remove permutation from permutations
    // add word2 to wordlist
    let mut combination = Vec::new();
    let mut cannot_find_next_word = true;
    while used_words.len() < word_list.len() {
        cannot_find_next_word = true;
        for permutation in permutations.clone().into_iter() {
            if(permutation.2.len() == 0) {
                println!("Cannot find next word");
                output.push(next_word);
                break;
            }
            if(permutation.0.eq(next_word) && !used_words.contains(&permutation.1)) {
                println!("next: {:?}", permutation);
                combination.push(permutation.clone());
                let truncated_word = &permutation.0[..permutation.2.start];
                next_word = permutation.1;
                used_words.push(next_word);
                println!("truncated: {}, next: {}, score: {}", truncated_word, next_word, permutation.2.len());
                output.push(truncated_word);
                cannot_find_next_word = false;
                break;
            }
        }
        if(cannot_find_next_word) {
            break;
        }
    }
    if(!cannot_find_next_word) {
        output.push(next_word);
    }

    println!("{}", output.join(""));

    return combination;
    
}

fn score_overlaps<'a>(word_list: &'a Vec<&'a str>) -> Vec<(&'a str, &'a str, Range<usize>)> {
    let mut permutations = Vec::new();
    for &prefix in word_list.into_iter() {
        for &suffix in word_list.into_iter() {
            let overlap_range = find_overlap(prefix, suffix);
            permutations.push((prefix, suffix, overlap_range.clone()));
            print_overlap(&prefix, &suffix, overlap_range)
        }
    }
    println!("{:?}", permutations);
    return(permutations);
}

fn find_overlap(word1: &str, word2: &str) -> Range<usize> {
    let mut overlap = 0;
    let mut overlap_range = 0..0;
    let mut suffix;
    let max_overlap = std::cmp::min(word1.len(), word2.len());

    //print!("{} and {}, max overlap: {}\n", word1, word2, max_overlap);

    for i in 0..max_overlap {
        suffix = &word2[..i];
        if word1.ends_with(suffix) && word1 != word2 {
            overlap = word2.len();
            overlap_range = (word1.len() - i)..word1.len();
        }
    }

    if overlap == 0 || overlap_range.start == overlap_range.end {
        return(0..0);
    } else {
        //print!("{} and {}, overlap: {:?}\n", word1, word2, overlap_range);
        //print_overlap(word1, word2, overlap_range);
        return(overlap_range);
    }
}

fn print_overlap(word1: &str, word2: &str, overlap_range: Range<usize>) {
    use colored::*;
    let suffix_start = overlap_range.end - overlap_range.start;
    let overlap_str = &word1[overlap_range.clone()].red().to_string();
    let prefix_str = &word1[..overlap_range.start];
    let suffix_str = &word2[suffix_start..];
    println!("{}{}{}\n", prefix_str, overlap_str, suffix_str);
}
