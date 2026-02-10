#[derive(Debug, Clone)]
pub struct ParsedMessage {
    pub command: String,
    pub args: Vec<String>,
    pub author: Option<String>,
    pub channel: String,
}

impl ParsedMessage {
    pub fn parse(text: &str, author: Option<String>, channel: String) -> Self {
        let parts: Vec<&str> = text.split_whitespace().collect();
        
        let (command, args) = if !parts.is_empty() && parts[0].starts_with('!') {
            let cmd = parts[0][1..].to_lowercase();
            let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
            (cmd, args)
        } else {
            (String::new(), Vec::new())
        };

        ParsedMessage {
            command,
            args,
            author,
            channel,
        }
    }

    pub fn is_command(&self) -> bool {
        !self.command.is_empty()
    }
}
