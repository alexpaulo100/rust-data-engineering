
/*Por que esse código dá erro?
Você está tentando capturar data[i] += 1 em várias threads, mas o Rust não permite múltiplas referências mutáveis ao mesmo tempo, especialmente entre threads diferentes. 
Isso gera risco de data race, e o Rust protege contra isso em tempo de compilação.*/
/*



use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    // No data race can occur, this will not compile.
}
*/


use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            vec[i] += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Resultado final: {:?}", data.lock().unwrap());
}
// Explicação:
// 1. `Arc` (Atomic Reference Counted) é usado para compartilhar a propriedade de `data` entre várias threads.
// 2. `Mutex` é usado para garantir que apenas uma thread possa acessar `data` ao mesmo tempo.
// 3. `lock()` é chamado para obter acesso exclusivo ao vetor dentro do mutex.
// 4. `unwrap()` é usado para lidar com o resultado do lock, que pode falhar se o mutex estiver em um estado de pânico.
// 5. `join()` é chamado para esperar que todas as threads terminem antes de continuar.
// 6. O resultado final é impresso após todas as threads terem terminado, mostrando o vetor atualizado.     

