use clap::Parser;
use fruit_salad_maker::create_fruit_salad;
use std::fs::File;
use std::io::Write;

fn display_fruit_salad(fruits: Vec<String>, dressing: String) {
    println!("Sua salada de frutas contém:");
    for fruit in &fruits {
        println!("{}", fruit);
    }
    println!("\nMolho escolhido: {}", dressing);
}

/// Salva a salada de frutas em um arquivo CSV
fn save_to_csv(fruits: &Vec<String>, dressing: &str, filename: &str) {
    let mut file = File::create(filename).expect("Erro ao criar arquivo CSV");
    writeln!(file, "Frutas, Molho").expect("Erro ao escrever cabeçalho");
    
    for fruit in fruits {
        writeln!(file, "{}, {}", fruit, dressing).expect("Erro ao escrever no arquivo");
    }

    println!("\nSalada salva em: {}", filename);
}

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Make a Fruit Salad"
)]
struct Opts {
    /// Frutas fornecidas como uma string de valores separados por vírgula
    #[clap(short, long)]
    fruits: Option<String>,
    
    /// Arquivo CSV de entrada
    csvfile: Option<String>,

    /// Arquivo CSV de saída (opcional)
    #[clap(short = 'o', long = "output")]
    output_file: Option<String>,
}

// Converte um CSV em um vetor de strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn main() {
    let opts: Opts = Opts::parse();

    // Pegar as frutas da CLI ou de um arquivo CSV
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename)
                .expect("Erro ao ler o arquivo CSV");
            csv_to_vec(&fruits)
        },
        None => {
            opts.fruits.unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        },
    };

    // Criar e exibir a salada de frutas com molho
    let (fruit_salad, dressing) = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad.clone(), dressing.clone());

    // Se um nome de arquivo de saída for fornecido, salvar a salada de frutas
    if let Some(output_filename) = opts.output_file {
        save_to_csv(&fruit_salad, &dressing, &output_filename);
    }
}
