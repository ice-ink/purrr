pub struct Game {
    turn: u32,
    over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self { turn: 0, over: false }
    }

    pub fn render(&self) -> String {
        format!(
            "\n=== Turn {} ===\n> ",
            self.turn
        )
    }

    pub fn handle(&mut self, input: &str) -> Option<String> {
        self.turn += 1;

        match input {
            "" => Some("You say nothing.".to_string()),
            "help" => Some("Try typing anything, or 'quit'.".to_string()),
            "win" => {
                self.over = true;
                Some("You win!".to_string())
            }
            other => Some(format!("You said: {}", other)),
        }
    }

    pub fn is_over(&self) -> bool {
        self.over
    }
}