use rand::Rng;
use std::time::Instant;

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1]{
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {

    let mut arr: Vec<i32> = Vec::with_capacity(1_000_000);
    let mut rng = rand::thread_rng();
    for _ in 0..1_000_000 {
        arr.push(rng.gen_range(0..100)); // Generar números aleatorios en el rango de 0 a 99 (excluyendo 100)
    }

    let start_time = Instant::now();

    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Tiempo transcurrido: {} segundos y {} milisegundos", elapsed_time.as_secs(), elapsed_time.subsec_millis());
}