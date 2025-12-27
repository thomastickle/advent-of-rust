use std::io::Write;

// TODO: Update the function argument type
pub fn print_message(message: &String) {
    internal_print_message(message, &mut std::io::stdout());
}

fn internal_print_message<W: Write>(message: &String, mut writer: W) {
    writeln!(writer, "Message: {}", message).expect("Failed to write to writer");
}

// Example Usage
pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    print_message(&gift_message);

    println!("{}", gift_message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_message_content() {
        let mut buffer = Vec::new();
        let message = String::from("Hello, Santa!");
        internal_print_message(&message, &mut buffer);
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "Message: Hello, Santa!\n");
    }

    #[test]
    fn test_print_message_execution() {
        let message = String::from("test");
        print_message(&message);
    }
}
