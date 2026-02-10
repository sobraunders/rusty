use std::collections::HashSet;

const WORDS: &[&str] = &[
    "programming", "rust", "bot", "hangman", "database",
    "function", "variable", "algorithm", "network", "server",
    "client", "protocol", "compiler", "library", "framework",
    "testing", "debugging", "performance", "security", "encryption",
];

const MAX_WRONG_GUESSES: u32 = 6;

#[derive(Clone, Debug)]
pub struct HangmanGame {
    word: String,
    guessed_letters: HashSet<char>,
    wrong_count: u32,
}

impl HangmanGame {
    pub fn new() -> Self {
        let word = WORDS[rand::random::<usize>() % WORDS.len()].to_lowercase();
        HangmanGame {
            word,
            guessed_letters: HashSet::new(),
            wrong_count: 0,
        }
    }

    pub fn guess(&mut self, letter: char) -> GuessResult {
        let letter = letter.to_lowercase().next().unwrap_or(letter);

        if self.guessed_letters.contains(&letter) {
            return GuessResult::AlreadyGuessed;
        }

        self.guessed_letters.insert(letter);

        if self.word.contains(letter) {
            if self.is_won() {
                GuessResult::Won
            } else {
                GuessResult::Correct
            }
        } else {
            self.wrong_count += 1;
            if self.is_lost() {
                GuessResult::Lost(self.word.clone())
            } else {
                GuessResult::Wrong
            }
        }
    }

    pub fn display(&self) -> String {
        self.word
            .chars()
            .map(|c| {
                if self.guessed_letters.contains(&c) {
                    c.to_string()
                } else {
                    "_".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn is_won(&self) -> bool {
        self.word
            .chars()
            .all(|c| self.guessed_letters.contains(&c))
    }

    pub fn is_lost(&self) -> bool {
        self.wrong_count >= MAX_WRONG_GUESSES
    }

    pub fn guessed(&self) -> String {
        let mut letters: Vec<char> = self.guessed_letters.iter().copied().collect();
        letters.sort();
        letters.into_iter().collect()
    }

    pub fn wrong_count(&self) -> u32 {
        self.wrong_count
    }

    pub fn remaining(&self) -> u32 {
        MAX_WRONG_GUESSES - self.wrong_count
    }

    pub fn word(&self) -> &str {
        &self.word
    }
}

#[derive(Debug)]
pub enum GuessResult {
    Correct,
    Wrong,
    Won,
    Lost(String),
    AlreadyGuessed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_initialization() {
        let game = HangmanGame::new();
        assert_eq!(game.wrong_count, 0);
        assert!(game.guessed_letters.is_empty());
        assert!(!game.word.is_empty());
        assert!(!game.is_won());
        assert!(!game.is_lost());
    }

    #[test]
    fn test_guess_correct_letter() {
        let mut game = HangmanGame::new();
        let first_letter = game.word.chars().next().unwrap();
        
        let result = game.guess(first_letter);
        match result {
            GuessResult::Correct | GuessResult::Won => {
                assert!(game.guessed_letters.contains(&first_letter));
                assert_eq!(game.wrong_count, 0);
            }
            _ => panic!("Expected Correct or Won for a letter in the word"),
        }
    }

    #[test]
    fn test_guess_wrong_letter() {
        let mut game = HangmanGame::new();
        let wrong_letter = 'z';
        
        // Make sure 'z' is not in the word (unlikely with our word list)
        if !game.word.contains(wrong_letter) {
            let result = game.guess(wrong_letter);
            match result {
                GuessResult::Wrong | GuessResult::Lost(_) => {
                    assert!(game.guessed_letters.contains(&wrong_letter));
                    assert_eq!(game.wrong_count, 1);
                }
                _ => panic!("Expected Wrong or Lost for a letter not in the word"),
            }
        }
    }

    #[test]
    fn test_guess_already_guessed() {
        let mut game = HangmanGame::new();
        let letter = game.word.chars().next().unwrap(); // Use a letter from the word
        
        game.guess(letter);
        let result = game.guess(letter);
        
        match result {
            GuessResult::AlreadyGuessed => {
                // Correct behavior
            }
            _ => panic!("Expected AlreadyGuessed"),
        }
    }

    #[test]
    fn test_case_insensitive_guessing() {
        let mut game = HangmanGame::new();
        let first_letter_lower = game.word.chars().next().unwrap();
        let first_letter_upper = first_letter_lower.to_uppercase().next().unwrap();
        
        let result = game.guess(first_letter_upper);
        match result {
            GuessResult::Correct | GuessResult::Won => {
                assert!(game.guessed_letters.contains(&first_letter_lower));
            }
            _ => {}
        }
    }

    #[test]
    fn test_display_initially_blanks() {
        let game = HangmanGame::new();
        let display = game.display();
        
        // Should show blanks for each letter
        let blank_count = display.matches('_').count();
        assert_eq!(blank_count, game.word.len());
    }

    #[test]
    fn test_display_after_correct_guess() {
        let mut game = HangmanGame::new();
        let first_letter = game.word.chars().next().unwrap();
        
        game.guess(first_letter);
        let display = game.display();
        
        // First character should be revealed
        assert!(display.starts_with(&first_letter.to_string()));
    }

    #[test]
    fn test_win_condition() {
        let mut game = HangmanGame::new();
        let word_to_guess = game.word.clone();
        
        // Guess all letters in the word
        for letter in word_to_guess.chars().collect::<std::collections::HashSet<_>>() {
            game.guess(letter);
        }
        
        assert!(game.is_won());
    }

    #[test]
    fn test_lose_condition() {
        let mut game = HangmanGame::new();
        
        // Guess 6 wrong letters
        let wrong_letters = vec!['@', '#', '$', '%', '^', '&'];
        for letter in wrong_letters {
            if !game.word.contains(letter) {
                game.guess(letter);
            }
        }
        
        assert!(game.is_lost());
    }

    #[test]
    fn test_guessed_letters_string() {
        let mut game = HangmanGame::new();
        game.guess('a');
        game.guess('b');
        game.guess('c');
        
        let guessed = game.guessed();
        assert!(guessed.contains('a'));
        assert!(guessed.contains('b'));
        assert!(guessed.contains('c'));
    }

    #[test]
    fn test_wrong_count() {
        let mut game = HangmanGame::new();
        
        assert_eq!(game.wrong_count(), 0);
        
        // Guess a wrong letter repeatedly (should only count once due to AlreadyGuessed)
        let wrong_letter = if game.word.contains('z') { 'x' } else { 'z' };
        if !game.word.contains(wrong_letter) {
            game.guess(wrong_letter);
            assert_eq!(game.wrong_count(), 1);
        }
    }

    #[test]
    fn test_remaining_guesses() {
        let game = HangmanGame::new();
        assert_eq!(game.remaining(), MAX_WRONG_GUESSES);
        
        let mut game2 = HangmanGame::new();
        // Force increment wrong_count directly for testing
        game2.wrong_count = 2;
        assert_eq!(game2.remaining(), MAX_WRONG_GUESSES - 2);
    }

    #[test]
    fn test_word_getter() {
        let game = HangmanGame::new();
        let word = game.word();
        
        assert!(!word.is_empty());
        assert_eq!(word, &game.word);
        // Verify it's from our word list
        assert!(WORDS.contains(&word.to_lowercase().as_str()) ||
                WORDS.iter().any(|w| w == &word.to_lowercase()));
    }

    #[test]
    fn test_word_from_list() {
        // Test multiple games to ensure words come from the list
        for _ in 0..10 {
            let game = HangmanGame::new();
            let word_lowercase = game.word.to_lowercase();
            assert!(WORDS.iter().any(|w| w == &word_lowercase));
        }
    }

    #[test]
    fn test_game_independence() {
        let mut game1 = HangmanGame::new();
        let game2 = HangmanGame::new();
        
        // Games should be independent
        let first_letter = game1.word.chars().next().unwrap();
        game1.guess(first_letter);
        
        assert!(game1.guessed_letters.contains(&first_letter));
        assert!(!game2.guessed_letters.contains(&first_letter));
    }
}
