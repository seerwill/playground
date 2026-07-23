fn main() {
    let _ = std::fs::write(
        "./src/lib.rs",
        r#"
    fn main() { panic!("broken"); }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_thing() {
            assert!(false);
        }
    }
"#,
    );
}
