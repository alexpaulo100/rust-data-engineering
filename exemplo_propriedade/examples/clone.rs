fn main(){

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");


    // o valor de s1 é copiado para s2
    // inteiros são copiados
    // e não clonados
    // o valor de x é copiado para y
    // e não o endereço de memória
    // o valor de x é copiado para y
    // e não o endereço de memória

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");


}