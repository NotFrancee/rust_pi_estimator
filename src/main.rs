extern crate rayon;

use std::time::Instant; 
use rayon::prelude::*;

fn parallel_monte_carlo_pi(points: usize) -> f64 {
    let within_circle = (0..points)
        .into_par_iter()
        .filter_map(|_| {
            let x = rand::random::<f64>(); 
            let y: f64 = rand::random::<f64>(); 
            if x * x + y * y <= 1f64 { Some(1) } else { None }
        })
        .count();
    4f64 * within_circle as f64 / points as f64
}


fn main() { 
    let n_iters = 100;

    let times: Vec<u128> = (0..n_iters).map(|_| {
        let now = Instant::now();
        _ = parallel_monte_carlo_pi(100_000_000);
        now.elapsed().as_millis()
    }).collect(); 

    let mean_time: f64 = times.iter().sum::<u128>() as f64 / n_iters as f64;
    println!("The mean running time is {}ms", mean_time); 
}