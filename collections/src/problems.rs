// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn mean_median_mode(numbers: Vec<i32>) -> (f64, i32, i32) {
    let mut sum = 0;
    let mut map = HashMap::new();
    for &num in &numbers {
        sum += num;
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mean = sum as f64 / numbers.len() as f64;
    let mut median = 0;
    let mut mode = 0;
    let mut max = 0;
    for (&num, &count) in &map {
        if count > max {
            mode = num;
            max = count;
        }
    }
    let mut sorted = numbers.clone();
    sorted.sort();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        median = (sorted[mid - 1] + sorted[mid]) / 2;
    } else {
        median = sorted[mid];
    }
    return (mean, median, mode);
}

// Convert strings to pig latin
// The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
pub fn to_pig_latin(s: &str) -> String {
    let mut result = String::new();
    for word in s.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if "aeiou".contains(first_char) {
            result.push_str(word);
            result.push_str("-hay ");
        } else {
            let rest: String = chars.collect();
            result.push_str(&rest);
            result.push('-');
            result.push(first_char);
            result.push_str("ay ");
        }
    }
    return result.trim().to_string();
}
