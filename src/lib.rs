use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod logger;
mod write_request;
mod writer;

/// Says Hello
#[pyfunction]
fn say_hello(name: String) {
    println!("Hello there {name}!");
    println!();
}

/// Runs several test loops
#[pyfunction]
fn run_loops() {
    logger::info("Running test loops...");
    let mut _count: u32 = 0;

    for _ in 0..1000 {
        for _ in 0..100 {
            _count += 1;
        }
    }

    print!("\n");
    logger::debug("Process Finished");
}

/// Makes async requests and writes static text to files
#[pyfunction]
fn make_write_request(filepath: String) {
    logger::info("Running async requests and file writing...");

    print!("\n");
    match write_request::write_and_request(&filepath) {
        // If function returned OK...
        Ok(_) => {
            print!("\n\n");
            logger::debug("Process finished successfully!");
        }
        // Otherwise...
        Err(_) => {
            print!("\n\n");
            logger::warn("write_and_request returned errors...");
        },
    }
}


/// Our Python module implemented using Rust
#[pymodule]
fn rusty_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(run_loops, m)?)?;
    m.add_function(wrap_pyfunction!(make_write_request, m)?)?;
    Ok(())
}
