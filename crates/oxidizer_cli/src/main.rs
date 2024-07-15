fn main() {
    if let Err(e) = oxidizer::run() {
        eprintln!("Error: {}", e);
    }
}
