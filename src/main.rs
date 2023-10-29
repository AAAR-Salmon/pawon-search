use itertools::Itertools;

fn main() {
    let characters = ['ぱ', 'お', 'ん', 'こ', 'う', 'ち'];

    for length in 3..=10 {
        for maybe_word in characters
            .iter()
            .cycle()
            .take(characters.len() * length)
            .permutations(length)
        {
            let maybe_word = maybe_word.into_iter().collect::<String>();
            println!("{}", maybe_word);
        }
    }
}
