use std::collections::HashMap;
use std::io;

fn count_word_frequencies(text: &str) -> Vec<(String, u32)> {
    let mut frequencies = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase(); // Para contar sem diferenciar maiúsculas e minúsculas
        *frequencies.entry(word).or_insert(0) += 1;
    }

    frequencies.into_iter().collect()
}

fn main() {
    let mut input = String::new();
    println!("Digite uma frase:");
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");

    let result = count_word_frequencies(&input);
    println!("A frequência de cada palavra é: {:?}", result);
}
