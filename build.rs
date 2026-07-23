fn main() {
    let _ = std::fs::write("./src/lib.rs", "fn main() { panic!(\"broken\"); }");
}
