pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message)
}

fn report(line: usize, r#where: String, message: String) {
    println!("[line {}] Error {}: {}", line, r#where, message)
}
