pub fn info(message: &str) {
    println!("💬\x1B[38;5;39mINFO:\x1B[0m {}", message);
}
pub fn success(message: &str) {
    println!("✅\x1B[38;5;2mSUCCESS:\x1B[0m {}", message);
}

pub fn warn(message: &str) {
    println!("⚠️\x1B[1;33mWARNING:\x1B[0m {}", message);
}

pub fn debug(message: &str) {
    println!("\x1B[38;5;5mDEBUG:\x1B[0m {}", message);
}