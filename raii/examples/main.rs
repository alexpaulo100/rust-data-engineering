

//Aqui você define uma struct chamada Ferramenta.Ela possui um único campo: nome, que é do tipo String
struct Ferramenta {
    nome: String,
}
//A função drop é chamada automaticamente quando o objeto sai de escopo.
//O código dentro do drop mostra uma mensagem dizendo que o recurso (ferramenta) está sendo liberado.
impl Drop for Ferramenta {
    fn drop(&mut self) {
        println!("Liberando a ferramenta: {}", self.nome);
    }
}
//Função principal do programa

fn main() {
    // Aqui, você cria uma instância da struct Ferramenta chamada martelo
    // e outra chamada chave dentro de um bloco.
    // Quando o bloco termina, a instância chave é descartada automaticamente.
    // Finalmente, o programa imprime uma mensagem indicando que o programa chegou ao fim.
    // A instância martelo é descartada automaticamente quando o programa termina.
    // A instância chave é descartada automaticamente quando o bloco termina.
    let martelo = Ferramenta { nome: String::from("Martelo") };
    {
        let chave = Ferramenta { nome: String::from("Chave de fenda") };
    } // Aqui, "Chave de fenda" é descartada

    println!("Fim do programa");
} // Aqui, "Martelo" é descartado
