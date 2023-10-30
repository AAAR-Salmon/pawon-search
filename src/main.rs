use itertools::Itertools;
use std::fs::File;
use vibrato::{tokenizer::worker::Worker, Dictionary, Tokenizer};

fn main() -> anyhow::Result<()> {
    let characters = ['ぱ', 'お', 'ん', 'こ', 'う', 'ち'];

    let dict = File::open("system.dic")?;
    let dict = Dictionary::read(dict)?;
    let tokenizer = Tokenizer::new(dict);
    let mut worker = tokenizer.new_worker();

    for length in 3..=10 {
        for word in (0..length)
            .map(|_| characters.iter())
            .multi_cartesian_product()
            .map(|v| v.into_iter().collect::<String>())
            // .filter(|s| !s.starts_with("ん"))
            .filter(|s| is_word(&mut worker, s))
        {
            println!("{}", word);
        }
    }

    Ok(())
}

fn is_word(worker: &mut Worker, s: &str) -> bool {
    worker.reset_sentence(s);
    worker.tokenize();

    worker.token_iter().count() == 1
}
