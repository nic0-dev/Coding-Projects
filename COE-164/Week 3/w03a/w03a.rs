use std::io::*;
use std::collections::HashMap;

struct LetterFreq {
    dictionary: HashMap<char, u64>,
}

impl LetterFreq {
    fn new() -> Self {
        LetterFreq{
            dictionary: HashMap::new(),
        }
    }

    fn count(&mut self, input: char) {
        let c = input.to_lowercase().next().unwrap();
        *self.dictionary.entry(c).or_insert(0) += 1;
    }

    fn current_counter(&mut self, input: char) -> u64{
        let input = input.to_lowercase().next().unwrap();
        *self.dictionary.get(&input).unwrap_or(&0)
    }
}

fn main() {
    let mut str_in = String::new();
    stdin().read_line(&mut str_in).expect("Invalid input!");
    let test_case: usize = str_in.trim().parse().expect("Not an integer!"); 

    // Iterate each Test Case
    for i in 1..=test_case {
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Invalid input!");
        let mut freq = LetterFreq::new();
        for c in str_in.chars() {
            if c == '\\' { break; }
            freq.count(c);
        }

        let mut letters = Vec::new();
        for i in 'a'..='z' { 
            letters.push(i); 
        }
        letters.push(' ');

        println!("---LETTER FREQUENCY of CASE #{}---", i);
        for letter in letters {
            let count = freq.current_counter(letter);
            if count > 0 {
                println!("{}: {}", letter, count);
            }
        }
        println!("{}", "-".repeat(33));
    }
}

// cat in_w03a_pub_s01.txt | ./w03a.exe | Out-File -Encoding UTF8 output.txt
// Compare-Object (gc output.txt) (gc out_w03a_pub_s01.txt)
