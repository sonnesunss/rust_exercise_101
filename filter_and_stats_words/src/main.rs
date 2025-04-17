use itertools::Itertools;

fn main() {
    let words = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("pear"),
        String::from("watermelon"),
        String::from("kiwi"),
        String::from("strawberry"),
    ];

    let new_words: Vec<(String, usize)> = words
        .iter()
        .filter(|word| word.len() >= 5)
        .map(|word| {
            let upper = word.to_uppercase();
            (upper.clone(), upper.len())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .sorted_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))
        .collect();

    println!("{:?}", new_words);
}
