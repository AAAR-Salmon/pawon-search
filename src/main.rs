use itertools::Itertools;

fn main() {
    let characters = ['ぱ', 'お', 'ん', 'こ', 'う', 'ち'];

    for length in 3..=10 {
        for word in (0..length)
            .map(|_| characters.iter())
            .multi_cartesian_product()
            .map(|v| v.into_iter().collect::<String>())
        {
            println!("{}", word);
        }
    }
}
