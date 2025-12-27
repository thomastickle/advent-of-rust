use std::io::Write;

pub fn print_message(message: String) {
    internal_print_message(message, &mut std::io::stdout());
}

fn internal_print_message<W: Write>(message: String, mut writer: W) {
    writeln!(writer, "Message: {message}").expect("Failed to write to writer");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_message_content() {
        let mut buffer = Vec::new();
        internal_print_message("Hello, Rust!".to_string(), &mut buffer);
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "Message: Hello, Rust!\n");
    }

    #[test]
    fn test_print_message_execution() {
        // This test ensures the public API still works as expected (doesn't panic)
        print_message("test".to_string());
    }
}
