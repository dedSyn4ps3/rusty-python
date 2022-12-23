use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod logger;
mod run_requests;

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

/// Makes async requests to query Github usernames
#[pyfunction]
fn begin_request_test() {
    logger::info("Running async requests...");

    print!("\n");
    match run_requests::start() {
        // If function returned OK...
        Ok(_) => {
            print!("\n\n");
            logger::debug("Process finished successfully!");
        }
        // Otherwise...
        Err(_) => {
            print!("\n\n");
            logger::warn("begin_request_test returned errors...");
        },
    }
}


/// Our Python module implemented using Rust
#[pymodule]
fn rusty_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(run_loops, m)?)?;
    m.add_function(wrap_pyfunction!(begin_request_test, m)?)?;
    Ok(())
}
