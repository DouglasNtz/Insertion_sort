use std::io::Take;
use rand;
use std::time::{Duration, Instant};
use my_insertion_sort::{my_insertion_sort, my_swap_sort, insertion_sort, swap_sort};
use my_insertion_sort::{my_insertion_sort_it, insertion_sort_it, swap_sort_it, my_swap_sort_it};

enum My_insertion_sort<T> {
    Times(for<'a> fn(&'a mut Vec<T>)),
    Iterations(for<'a> fn(&'a mut Vec<T>) -> usize)
}

enum Insertion_sort<T> {
    Times(for<'a> fn(&'a mut Vec<T>)),
    Iterations(for<'a> fn(&'a mut Vec<T>) -> usize)
}

enum My_swap_sort<T> {
    Times(for<'a> fn(&'a mut Vec<T>)),
    Iterations(for<'a> fn(&'a mut Vec<T>) -> usize)
}

enum Swap_sort<T> {
    Times(for<'a> fn(&'a mut Vec<T>)),
    Iterations(for<'a> fn(&'a mut Vec<T>) -> usize)
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let numero_experimentos = args[1].parse::<usize>().unwrap();

    let tamanho_lista = args[2].parse::<usize>().unwrap();

    let func_name = args[3].as_str();

    let kind_algorithm = args[4].as_str();

    let algorithms;

    match kind_algorithm {
        "Times" => {
            algorithms = (My_insertion_sort::Times(my_insertion_sort),
                          Insertion_sort::Times(insertion_sort),
                          My_swap_sort::Times(my_swap_sort),
                          Swap_sort::Times(swap_sort))
        },
        "Iterations" => {
            algorithms = (My_insertion_sort::Iterations(my_insertion_sort_it),
                          Insertion_sort::Iterations(insertion_sort_it),
                          My_swap_sort::Iterations(my_swap_sort_it),
                          Swap_sort::Iterations(swap_sort_it))
        },
        _ => {
            println!("Tipo de algoritmo não existe.");
            return;
        }

    }

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    let mut vec_iterations = Vec::with_capacity(numero_experimentos);

    let mut iterations: usize;

    for _exp in 0..numero_experimentos {

        let mut v0 = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v0.push(rand::random::<i32>());
        }

        let mut v = v0.clone();

        match func_name {
            "my_insertion_sort" => {
                start_time = Instant::now();
                iterations = match algorithms.0 {
                    My_insertion_sort::Times(func) => {
                        func(&mut v);
                        0
                    },
                    My_insertion_sort::Iterations(func) => {
                        func(&mut v)
                    }
                };
                //my_insertion_sort(&mut v);
                duration = start_time.elapsed();
            },
            "insertion_sort" => {
                start_time = Instant::now();
                iterations = match algorithms.1 {
                    Insertion_sort::Times(func) => {
                        func(&mut v);
                        0
                    },
                    Insertion_sort::Iterations(func) => {
                        func(&mut v)
                    }
                };
                //insertion_sort(&mut v);
                duration = start_time.elapsed();
            },
            "my_swap_sort" => {
                start_time = Instant::now();
                iterations = match algorithms.2 {
                    My_swap_sort::Times(func) => {
                        func(&mut v);
                        0
                    },
                    My_swap_sort::Iterations(func) => {
                        func(&mut v)
                    }
                };
                //my_swap_sort(&mut v);
                duration = start_time.elapsed();
            },
            "swap_sort" => {
                start_time = Instant::now();
                iterations = match algorithms.3 {
                    Swap_sort::Times(func) => {
                        func(&mut v);
                        0
                    },
                    Swap_sort::Iterations(func) => {
                        func(&mut v)
                    }
                };
                //swap_sort(&mut v);
                duration = start_time.elapsed();
            },
            _ => {
                println!("Nome da função não existe.");
                return;
            }
        };

        times.push(duration);
        vec_iterations.push(iterations);

    }


    if kind_algorithm == "Times" {
        println!(r###"Function: {func_name}
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());
    } else if kind_algorithm == "Iterations" {
        println!(r###"Function: {func_name}
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Itarações por execução: {:?}
"###, (vec_iterations.iter().sum::<usize>() as f64) / (numero_experimentos as f64));
    }

}

