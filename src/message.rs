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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_with_args() {
        let msg = ParsedMessage::parse(
            "!ping arg1 arg2",
            Some("user".to_string()),
            "#channel".to_string(),
        );
        assert_eq!(msg.command, "ping");
        assert_eq!(msg.args, vec!["arg1", "arg2"]);
        assert_eq!(msg.author, Some("user".to_string()));
        assert_eq!(msg.channel, "#channel");
    }

    #[test]
    fn test_parse_command_without_args() {
        let msg = ParsedMessage::parse(
            "!hello",
            Some("alice".to_string()),
            "#test".to_string(),
        );
        assert_eq!(msg.command, "hello");
        assert_eq!(msg.args.len(), 0);
        assert!(msg.is_command());
    }

    #[test]
    fn test_parse_non_command() {
        let msg = ParsedMessage::parse(
            "just a regular message",
            Some("bob".to_string()),
            "#channel".to_string(),
        );
        assert_eq!(msg.command, "");
        assert!(!msg.is_command());
    }

    #[test]
    fn test_parse_command_case_insensitive() {
        let msg = ParsedMessage::parse(
            "!PING",
            None,
            "#test".to_string(),
        );
        assert_eq!(msg.command, "ping");
    }

    #[test]
    fn test_parse_command_with_spaces() {
        let msg = ParsedMessage::parse(
            "!echo   multiple   spaces",
            Some("user".to_string()),
            "#ch".to_string(),
        );
        assert_eq!(msg.command, "echo");
        assert_eq!(msg.args, vec!["multiple", "spaces"]);
    }

    #[test]
    fn test_is_command_true() {
        let msg = ParsedMessage::parse("!test", None, "#ch".to_string());
        assert!(msg.is_command());
    }

    #[test]
    fn test_is_command_false() {
        let msg = ParsedMessage::parse("not a command", None, "#ch".to_string());
        assert!(!msg.is_command());
    }

    #[test]
    fn test_parse_author_none() {
        let msg = ParsedMessage::parse("!ping", None, "#ch".to_string());
        assert_eq!(msg.author, None);
    }
}
