use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();

    words
        .split(|c: char| c.is_whitespace() || (c.is_ascii_punctuation() && c != '\''))
        .for_each(|w| {
            let key = w
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_string()
                .to_lowercase();
            if !key.is_empty() {
                res.entry(key).and_modify(|e| *e += 1).or_insert(1);
            }
        });

    println!("{res:?}");

    res
}
