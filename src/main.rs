// Title: Overlap Compression
// Author: Heikki Juva
// Date: 2023-08-27
// Purpose: Compress a list of words by overlapping suffixes and prefixes


use core::ops::Range;
use std::collections::HashMap;

const INPUT: [&str; 5] = ["pleasure", "apple", "rendered", "clap", "suren"];

fn main() {
    println!("{:?}", INPUT);
    let input = INPUT.to_vec();
    compress(&input);
}

fn compress(word_list: &Vec<&str>) {
    let mut output = Vec::new();
    let mut used_words = Vec::new();

    // find all permutations of word1+word2
    let mut permutations = Vec::new();
    permutations = score_overlaps(word_list);

    // find permutation with maximum overlapping range, where the word1 does not exist in any other permutation word2
    // remove permutation from permutations
    // add permutation (word1+word2) to wordlist
    permutations.sort_by(|a, b| b.2.len().cmp(&a.2.len()));
    
    // Count the occurance of each suffix word, to select the rarest suffix word which has best overlapping score
    // TODO: Fix this, as the best score is not always best starting word.
    let mut next_word_scores = HashMap::new();
    for permutation in permutations.iter() {
        let next_word_score = next_word_scores.entry(permutation.0).or_insert(0);
        *next_word_score += permutation.2.len();
    }
    println!("{:?}", next_word_scores);
    let mut best_starting_word = next_word_scores.iter().max_by_key(|x| x.1).unwrap(); // TODO: What?? Maybe this could be rewritten to be readable
    let mut next_word = best_starting_word.0.clone().clone();
    used_words.push(next_word);

    //let mut rm_range = best_combination..clone().clone();
    //let tmp= &(output.to_owned() + next_word[rm_range.start]); // TODO: this is ugly, there must be a better way
    //output = &tmp;

    // find permutation with maximum overlapping range, where the word1 is the the word2 of previous permutation
    // remove permutation from permutations
    // add word2 to wordlist
    while used_words.len() < word_list.len() {
        let mut cannot_find_next_word = true;
        for permutation in permutations.iter() {
            if(permutation.2.len() == 0) {
                println!("Cannot find next word");
                output.push(next_word);
                break;
            }
            if(permutation.0.eq(&next_word) && !used_words.contains(&permutation.1)) {
                println!("next: {:?}", permutation);
                let truncated_word = &permutation.0[..permutation.2.start];
                next_word = permutation.1.clone().clone();
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

    println!("Output: {:?}", output)
    
}

fn score_overlaps<'a>(word_list: &'a Vec<&'a str>) -> Vec<(&'a &'a str, &'a &'a str, Range<usize>)> {
    let mut permutations = Vec::new();
    for prefix in word_list.iter() {
        for suffix in word_list.iter() {
            let mut overlap_range = find_overlap(prefix, suffix);
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
