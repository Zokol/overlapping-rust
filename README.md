## Overlap Compression

## Description
This program compresses a list of words by finding the overlapping suffixes and prefixes between them. The program iterates through all possible combinations of word pairs and identifies the overlaps. It then selects the pair with the maximum overlapping range, where the suffix word does not exist as a prefix in any other pair. This pair is then added to the compressed list, and the process is repeated until all words are used or no more overlaps are found.

## Usage
Run the program with the Rust compiler by using the `cargo run` command. The program will output the original list of words, the number of times each word appears as a suffix in the overlaps, and the compressed list of words.

## Code Structure
The `main` function initializes the list of words and calls the `compress` function.

The `compress` function finds all the permutations of word pairs and their overlapping ranges. It then iterates through the permutations to find the best starting word (the word with the maximum overlapping score and the rarest occurrence as a suffix). It then finds the next word by looking for a pair where the current word is a prefix, and the suffix is not already used. The prefix of the current word is then added to the output list, and the process is repeated until all words are used or no more overlaps are found.

The `score_overlaps` function iterates through all possible combinations of word pairs and finds their overlapping ranges. It returns a vector of tuples containing the prefix word, the suffix word, and the overlapping range.

The `find_overlap` function finds the overlapping range between two words. It returns a range of indices indicating the start and end of the overlap in the prefix word.

The `print_overlap` function prints the prefix word, the overlapping part, and the suffix word in different colors to visualize the overlap.

## Example
Input: ["pleasure", "apple", "rendered", "clap", "suren"]  
Output: ["plea", "apple", "rendered", "clap", "suren"]

## Dependencies
- `colored`: This crate is used in the `print_overlap` function to color the overlapping part of the words.

## Notes
- The program does not handle cases where there are multiple words with the same maximum overlapping score and occurrence as a suffix. It simply selects the first one found.
- The program does not handle cases where there are multiple optimal solutions for the compressed list of words. It simply selects the first one found.
- The program assumes that the list of words does not contain duplicates.

## Future Improvements
- Handle cases where there are multiple words with the same maximum overlapping score and occurrence as a suffix.
- Handle cases where there are multiple optimal solutions for the compressed list of words.
- Optimize the `compress` function to reduce the time complexity.
- Improve the readability of the `compress` function by refactoring the code and adding comments.

## License
This project is licensed under the MIT License.