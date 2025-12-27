pub fn send_message_to_santa() -> String {
    String::from("I've been nice")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_message_to_santa() {
        assert_eq!(send_message_to_santa(), "I've been nice");
    }
}
