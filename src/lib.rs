#[cfg(test)]
mod tests;
mod algorithms;
mod proof_algorithms;
mod performance_algorithms;

pub use algorithms::{my_insertion_sort, insertion_sort, swap_sort, my_swap_sort};

pub use performance_algorithms::{my_insertion_sort_it, insertion_sort_it, swap_sort_it, my_swap_sort_it};

