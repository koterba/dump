use pyo3::prelude::*;
use pyo3::types::PyList;
use rand::prelude::*;

// def calculate_pi() -> int:
//     in_square = in_circle = pi = 0
//     while True:
//         x = random.random()
//         y = random.random()

//         dist = (x*x + y*y) ** 0.5
//         in_square += 1

//         if dist <= 1:
//             in_circle += 1
        
//         if in_square % 1000 == 0: ## in square is used only not to waste variable names
//             pi = 4 * in_circle / in_square
//             yield pi

/// Formats the sum of two numbers as string.
#[pyfunction]
fn get_next_pi(iterations: i32, mut in_square: f64, mut in_circle: f64, mut pi: f64) -> Vec<f64> {
    for _ in 0..iterations {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        let dist = (x*x + y*y).powf(0.5);
        in_square += 1.0;

        if dist <= 1.0 {
            in_circle += 1.0;
        }
    }

    pi = 4.0 * in_circle / in_square;
    
    vec![in_square, in_circle, pi]
}

/// A Python module implemented in Rust.
#[pymodule]
fn pi_aprox(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_next_pi, m)?)?;
    Ok(())
}