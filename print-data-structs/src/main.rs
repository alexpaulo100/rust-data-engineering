use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, BTreeSet, BinaryHeap}; // Adicionando BTreeSet e BinaryHeap
use std::io::Write;

fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Coleções iniciais
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    println!("\n\tVec após inserção:{:?}", vec);
    vec.pop();
    println!("\tVec após remoção:{:?}", vec);
    vec.insert(0, 1914);
    println!("\tVec após inserção em posição especifica:{:?}", vec);

    let mut vec_deque = VecDeque::new();
    vec_deque.push_back(1);
    vec_deque.push_front(51);
    vec_deque.push_front(85);
    println!("\n\tVecDeque após inserção: {:?}", vec_deque);
    vec_deque.pop_front();
    println!("\tVecDeque após remoção do front: {:?}", vec_deque);
    vec_deque.pop_back(); // Remove o último elemento
    println!("\tVecDeque após remoção do back: {:?}", vec_deque);

    // Coleções interativas
    println!("\nEscolha uma coleção para manipular:");
    println!("1. Vec");
    println!("2. VecDeque");
    println!("3. LinkedList");
    println!("4. HashMap");
    println!("5. BTreeMap");
    println!("6. BTreeSet");
    println!("7. BinaryHeap");

    let mut escolha = String::new();
    print!("Digite o número da sua escolha: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut escolha).unwrap();
    let escolha: u32 = escolha.trim().parse().unwrap();

    match escolha {
        1 => manipular_vec(),
        2 => manipular_vec_deque(),
        3 => manipular_linked_list(),
        4 => manipular_hash_map(),
        5 => manipular_btree_map(),
        6 => manipular_btree_set(),
        7 => manipular_binary_heap(),
        _ => println!("Escolha inválida!"),
    }
}

// Função para manipular um Vec
fn manipular_vec() {
    let mut vec = Vec::new();
    loop {
        println!("\nOperações no Vec:");
        println!("1. Adicionar um elemento");
        println!("2. Remover um elemento");
        println!("3. Imprimir Vec");
        println!("4. Sair");

        let mut escolha = String::new();
        print!("Escolha uma operação: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut escolha).unwrap();
        let escolha: u32 = escolha.trim().parse().unwrap();

        match escolha {
            1 => {
                let mut elem = String::new();
                print!("Digite um valor para adicionar: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut elem).unwrap();
                let elem: i32 = elem.trim().parse().unwrap();
                vec.push(elem);
            }
            2 => {
                if let Some(removed) = vec.pop() {
                    println!("Elemento removido: {}", removed);
                } else {
                    println!("Vec vazio, nada para remover!");
                }
            }
            3 => println!("{:?}", vec),
            4 => break,
            _ => println!("Escolha inválida!"),
        }
    }
}

// Função para manipular um VecDeque
fn manipular_vec_deque() {
    let mut vec_deque = VecDeque::new();
    loop {
        println!("\nOperações no VecDeque:");
        println!("1. Adicionar um elemento");
        println!("2. Remover um elemento do início");
        println!("3. Remover um elemento do final");
        println!("4. Imprimir VecDeque");
        println!("5. Sair");

        let mut escolha = String::new();
        print!("Escolha uma operação: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut escolha).unwrap();
        let escolha: u32 = escolha.trim().parse().unwrap();

        match escolha {
            1 => {
                let mut elem = String::new();
                print!("Digite um valor para adicionar: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut elem).unwrap();
                let elem: i32 = elem.trim().parse().unwrap();
                vec_deque.push_back(elem);
            }
            2 => {
                if let Some(removed) = vec_deque.pop_front() {
                    println!("Elemento removido do início: {}", removed);
                } else {
                    println!("VecDeque vazio, nada para remover!");
                }
            }
            3 => {
                if let Some(removed) = vec_deque.pop_back() {
                    println!("Elemento removido do final: {}", removed);
                } else {
                    println!("VecDeque vazio, nada para remover!");
                }
            }
            4 => println!("{:?}", vec_deque),
            5 => break,
            _ => println!("Escolha inválida!"),
        }
    }
}

// Função para manipular um LinkedList
fn manipular_linked_list() {
    let mut linked_list = LinkedList::new();
    loop {
        println!("\nOperações no LinkedList:");
        println!("1. Adicionar um elemento");
        println!("2. Remover um elemento");
        println!("3. Imprimir LinkedList");
        println!("4. Sair");

        let mut escolha = String::new();
        print!("Escolha uma operação: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut escolha).unwrap();
        let escolha: u32 = escolha.trim().parse().unwrap();

        match escolha {
            1 => {
                let mut elem = String::new();
                print!("Digite um valor para adicionar: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut elem).unwrap();
                let elem: i32 = elem.trim().parse().unwrap();
                linked_list.push_back(elem);
            }
            2 => {
                if let Some(removed) = linked_list.pop_back() {
                    println!("Elemento removido: {}", removed);
                } else {
                    println!("LinkedList vazio, nada para remover!");
                }
            }
            3 => println!("{:?}", linked_list),
            4 => break,
            _ => println!("Escolha inválida!"),
        }
    }
}

// Função para manipular um HashMap
fn manipular_hash_map() {
    let mut hash_map = HashMap::new();
    loop {
        println!("\nOperações no HashMap:");
        println!("1. Adicionar um valor");
        println!("2. Remover um valor");
        println!("3. Imprimir HashMap");
        println!("4. Sair");

        let mut escolha = String::new();
        print!("Escolha uma operação: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut escolha).unwrap();
        let escolha: u32 = escolha.trim().parse().unwrap();

        match escolha {
            1 => {
                let mut key = String::new();
                print!("Digite a chave: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut key).unwrap();
                let key: String = key.trim().to_string();

                let mut value = String::new();
                print!("Digite o valor: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut value).unwrap();
                let value: i32 = value.trim().parse().unwrap();

                hash_map.insert(key, value);
            }
            2 => {
                let mut key = String::new();
                print!("Digite a chave a ser removida: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut key).unwrap();
                let key: String = key.trim().to_string();

                if hash_map.remove(&key).is_some() {
                    println!("Chave removida com sucesso!");
                } else {
                    println!("Chave não encontrada!");
                }
            }
            3 => println!("{:?}", hash_map),
            4 => break,
            _ => println!("Escolha inválida!"),
        }
    }
}

// Função para manipular um BTreeMap
fn manipular_btree_map() {
    let mut btree_map = BTreeMap::new();
    loop {
        println!("\nOperações no BTreeMap:");
        println!("1. Adicionar um valor");
        println!("2. Remover um valor");
        println!("3. Imprimir BTreeMap");
        println!("4. Sair");

        let mut escolha = String::new();
        print!("Escolha uma operação: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut escolha).unwrap();
        let escolha: u32 = escolha.trim().parse().unwrap();

        match escolha {
            1 => {
                let mut key = String::new();
                print!("Digite a chave: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut key).unwrap();
                let key: i32 = key.trim().parse().unwrap();

                let mut value = String::new();
                print!("Digite o valor: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut value).unwrap();
                let value: String = value.trim().to_string();

                btree_map.insert(key, value);
            }
            2 => {
                let mut key = String::new();
                print!("Digite a chave a ser removida: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut key).unwrap();
                let key: i32 = key.trim().parse().unwrap();

                if btree_map.remove(&key).is_some() {
                    println!("Chave removida com sucesso!");
                } else {
                    println!("Chave não encontrada!");
                }
            }
            3 => println!("{:?}", btree_map),
            4 => break,
            _ => println!("Escolha inválida!"),
        }
    }
}

// Função para manipular um BTreeSet
fn manipular_btree_set() {
    let mut btree_set = BTreeSet::new();
    loop {
        println!("\nOperações no BTreeSet:");
        println!("1. Adicionar um valor");
        println!("2. Remover um valor");
        println!("3. Imprimir BTreeSet");
        println!("4. Sair");

        let mut escolha = String::new();
        print!("Escolha uma operação: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut escolha).unwrap();
        let escolha: u32 = escolha.trim().parse().unwrap();

        match escolha {
            1 => {
                let mut value = String::new();
                print!("Digite um valor para adicionar: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut value).unwrap();
                let value: i32 = value.trim().parse().unwrap();
                btree_set.insert(value);
            }
            2 => {
                let mut value = String::new();
                print!("Digite um valor para remover: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut value).unwrap();
                let value: i32 = value.trim().parse().unwrap();
                if btree_set.remove(&value) {
                    println!("Valor removido: {}", value);
                } else {
                    println!("Valor não encontrado!");
                }
            }
            3 => println!("{:?}", btree_set),
            4 => break,
            _ => println!("Escolha inválida!"),
        }
    }
}

// Função para manipular um BinaryHeap
fn manipular_binary_heap() {
    let mut binary_heap = BinaryHeap::new();
    loop {
        println!("\nOperações no BinaryHeap:");
        println!("1. Adicionar um valor");
        println!("2. Remover o valor de maior prioridade");
        println!("3. Imprimir BinaryHeap");
        println!("4. Sair");

        let mut escolha = String::new();
        print!("Escolha uma operação: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut escolha).unwrap();
        let escolha: u32 = escolha.trim().parse().unwrap();

        match escolha {
            1 => {
                let mut value = String::new();
                print!("Digite um valor para adicionar: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut value).unwrap();
                let value: i32 = value.trim().parse().unwrap();
                binary_heap.push(value);
            }
            2 => {
                if let Some(removed) = binary_heap.pop() {
                    println!("Valor removido: {}", removed);
                } else {
                    println!("BinaryHeap vazio!");
                }
            }
            3 => println!("{:?}", binary_heap),
            4 => break,
            _ => println!("Escolha inválida!"),
        }
    }
}
