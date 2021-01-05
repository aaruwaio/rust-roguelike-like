use std::collections::VecDeque;

const MAX_MESSAGES: usize = 3;

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

    pub fn push(&mut self, text: String) {
        self.messages.push_back(text);
        if self.messages.len() > MAX_MESSAGES {
            self.messages.pop_front();
        }
    }
}
