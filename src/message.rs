use std::collections::VecDeque;

const MAX_MESSAGES: usize = 3;
const MAX_MESSAGE_LENGTH: usize = 30;

#[derive(Debug)]
pub struct Message {
    pub messages: VecDeque<String>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
        }
    }

    pub fn push(&mut self, text: String) -> Result<(), String> {
        if text.len() > MAX_MESSAGE_LENGTH {
            return Err(format!("The message string is too long."));
        }
        self.messages.push_back(text);
        if self.messages.len() > MAX_MESSAGES {
            self.messages.pop_front();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push_message_less_than_max_message_length() -> Result<(), String> {
        let mut messages = Message::new();
        messages.push(String::from("a").repeat(MAX_MESSAGE_LENGTH))
    }
}
