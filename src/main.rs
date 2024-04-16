use rand;
use std::time::{Duration, Instant};
use my_insertion_sort::{my_insertion_sort, my_swap_sort, insertion_sort, swap_sort};

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let numero_experimentos = args[1].parse::<usize>().unwrap();

    let tamanho_lista = args[2].parse::<usize>().unwrap();

    let func_name = args[3].as_str();

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    for _exp in 0..numero_experimentos {

        let mut v0 = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v0.push(rand::random::<i32>());
        }

        let mut v = v0.clone();

        match func_name {
            "my_insertion_sort" => {
                start_time = Instant::now();
                my_insertion_sort(&mut v);
                duration = start_time.elapsed();
            },
            "swap_sort" => {
                start_time = Instant::now();
                swap_sort(&mut v);
                duration = start_time.elapsed();
            },
            "insertion_sort" => {
                start_time = Instant::now();
                insertion_sort(&mut v);
                duration = start_time.elapsed();
            },
            "my_swap_sort" => {
                start_time = Instant::now();
                my_swap_sort(&mut v);
                duration = start_time.elapsed();
            },
            _ => {
                println!("Função digitado não existe...");
                return;
            }
        };

        times.push(duration);

    }

    println!(r###"Function: {func_name}
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());

}

