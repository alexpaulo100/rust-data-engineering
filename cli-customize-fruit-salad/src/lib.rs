use rand::prelude::*; 



/// Função que cria uma salada de frutas e escolhe um molho aleatório
pub fn create_fruit_salad(mut fruits: Vec<String>) -> (Vec<String>, String) {
    let mut rng = rand::rng();

    // Embaralha a lista de frutas
    fruits.shuffle(&mut rng);

    // Lista de molhos para escolher aleatoriamente
    let dressings = vec![
        "Mel e limão",
        "Calda de chocolate",
        "Iogurte natural",
        "Xarope de bordo",
        "Leite condensado",
        "Suco de laranja",
        "Chantilly",
    ];

    // Escolhe um molho aleatoriamente
    let dressing = dressings
        .choose(&mut rng)
        .unwrap_or(&"Molho padrão")
        .to_string();

    (fruits, dressing)
}
