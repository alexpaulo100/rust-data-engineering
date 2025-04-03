# CLI Customize Fruit Salad

Este é um aplicativo de linha de comando (CLI) para criar uma salada de frutas personalizada com um molho aleatório. Você pode fornecer as frutas via entrada direta ou a partir de um arquivo CSV e salvar a salada gerada em um novo arquivo CSV.

## 📌 Como Usar

### 1. Rodar o programa sem salvar em CSV
```sh
cargo run -- --fruits "banana, maçã, morango, uva"
```

### 2. Rodar e salvar a saída em um arquivo CSV
```sh
cargo run -- --fruits "banana, maçã, morango, uva" -o minha_salada.csv
```

### 3. Ler frutas de um arquivo CSV e salvar a saída
```sh
cargo run -- fruits.csv -o resultado.csv
```

## 🛠 Estrutura do Projeto

- `src/main.rs`: Código principal do CLI
- `src/lib.rs`: Implementação das funções auxiliares
- `Cargo.toml`: Gerenciamento de dependências
- `README.md`: Este arquivo, com as instruções de uso

## 🚀 Funcionalidades
- Aceita entrada de frutas via CLI ou CSV
- Escolhe um molho aleatório para a salada
- Exibe a salada no terminal
- Salva a salada gerada em um arquivo CSV opcionalmente

## 📦 Dependências
Este projeto utiliza as seguintes bibliotecas Rust:
- `clap` → Para o parsing de argumentos da linha de comando
- `rand` → Para a escolha aleatória do molho

Instale as dependências rodando:
```sh
cargo build
```

## 📄 Exemplo de Arquivo CSV
Se quiser usar um arquivo CSV como entrada, crie um arquivo `fruits.csv` com o seguinte conteúdo:
```csv
banana, maçã, morango, uva
```
Depois, rode o programa com:
```sh
cargo run -- fruits.csv -o resultado.csv
```

Isso criará um arquivo `resultado.csv` com a lista de frutas e o molho escolhido.

---

Criado por: **Alex Silva**

