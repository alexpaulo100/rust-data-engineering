fn main() {
    let caixa = String::from("Ferramentas");

    {
        let alicate = String::from("Alicate");
        println!("Usando o {}", alicate);
    } // 🔐 A porta se fecha: o alicate é descartado (drop é chamado)

    println!("Ainda tenho a {}", caixa);
} // 🏁 A casa é fechada: a caixa é descartada aqui
