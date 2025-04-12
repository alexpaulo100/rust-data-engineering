fn main() {
    let s = String::from("hello");  // s entra no escopo

    assume_responsabilidade(s);             // O valor de s é movido para a função...
                                    // ...e por isso não é mais válido aqui

    let x = 5;                      // x entra em escopo

    faz_copia(x);                  //porque i32 implementa a característica Copy,
                                    // x NÃO se move para a função,
    println!("{}", x);              // então está tudo bem usar x depois
} // // Aqui, x sai do escopo, depois s. Mas como o valor de s foi movido, nada 
// de especial acontece.

fn assume_responsabilidade(some_string: String) { // some_string entra no escopo
    println!("{some_string}");
} // Aqui, some_string sai do escopo e `drop` é chamado. O suporte
  // a memória é liberada.

fn faz_copia(some_integer: i32) { // some_integer comes entra no escopo
    println!("{some_integer}");
} // Aqui, some_integer sai do escopo. Nada de especial acontece.