use std::thread;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::sync::{Arc, Mutex};

const MAX_IDX: u64 = 1000;
const CHUNK_SIZE: u64 = 1000;

fn main() {
    let mut handles = vec![];

    // Para evitar prints bagunçados na saída, podemos usar Mutex para sincronizar
    let stdout_mutex = Arc::new(Mutex::new(()));

    for chunk_start in (0..=MAX_IDX).step_by(CHUNK_SIZE as usize) {
        let chunk_end = (chunk_start + CHUNK_SIZE - 1).min(MAX_IDX);
        let stdout_mutex = Arc::clone(&stdout_mutex);

        let handle = thread::spawn(move || {
            for i in chunk_start..=chunk_end {
                let fib = fibonacci_big(i);
                // Para imprimir sem bagunçar a saída entre threads
                let _lock = stdout_mutex.lock().unwrap();
                println!("fibonacci({}) = {}", i, fib);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// Fibonacci iterativo com BigUint para índices grandes
fn fibonacci_big(n: u64) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }
    if n == 1 {
        return BigUint::one();
    }

    let mut a = BigUint::zero();
    let mut b = BigUint::one();

    for _ in 2..=n {
        let c = &a + &b;
        a = b;
        b = c;
    }

    b
}
