pub mod hangman;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use hangman::HangmanGame;

pub struct GameManager {
    hangman_games: Arc<Mutex<HashMap<String, HangmanGame>>>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            hangman_games: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn new_hangman(&self, channel: &str) {
        let mut games = self.hangman_games.lock().await;
        games.insert(channel.to_string(), HangmanGame::new());
    }

    pub async fn hangman_guess(&self, channel: &str, letter: char) -> Option<hangman::GuessResult> {
        let mut games = self.hangman_games.lock().await;
        games.get_mut(channel).map(|game| game.guess(letter))
    }

    pub async fn hangman_game(&self, channel: &str) -> Option<HangmanGame> {
        let games = self.hangman_games.lock().await;
        games.get(channel).cloned()
    }

    pub async fn hangman_quit(&self, channel: &str) -> bool {
        let mut games = self.hangman_games.lock().await;
        games.remove(channel).is_some()
    }

    pub fn clone_arc(&self) -> Arc<Mutex<HashMap<String, HangmanGame>>> {
        Arc::clone(&self.hangman_games)
    }
}

impl Clone for GameManager {
    fn clone(&self) -> Self {
        GameManager {
            hangman_games: Arc::clone(&self.hangman_games),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_hangman_game() {
        let manager = GameManager::new();
        manager.new_hangman("#channel1").await;
        
        let game = manager.hangman_game("#channel1").await;
        assert!(game.is_some());
    }

    #[tokio::test]
    async fn test_hangman_guess() {
        let manager = GameManager::new();
        manager.new_hangman("#channel1").await;
        
        let game = manager.hangman_game("#channel1").await.unwrap();
        let first_letter = game.word().chars().next().unwrap();
        drop(game);
        
        let result = manager.hangman_guess("#channel1", first_letter).await;
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_hangman_quit() {
        let manager = GameManager::new();
        manager.new_hangman("#channel1").await;
        
        let quit_result = manager.hangman_quit("#channel1").await;
        assert!(quit_result);
        
        let game = manager.hangman_game("#channel1").await;
        assert!(game.is_none());
    }

    #[tokio::test]
    async fn test_channel_isolation() {
        let manager = GameManager::new();
        manager.new_hangman("#channel1").await;
        manager.new_hangman("#channel2").await;
        
        let game1 = manager.hangman_game("#channel1").await;
        let game2 = manager.hangman_game("#channel2").await;
        
        // Both should exist and be different games
        assert!(game1.is_some());
        assert!(game2.is_some());
        assert_ne!(game1.unwrap().word(), game2.unwrap().word());
    }

    #[tokio::test]
    async fn test_nonexistent_game() {
        let manager = GameManager::new();
        let game = manager.hangman_game("#channel1").await;
        assert!(game.is_none());
    }

    #[tokio::test]
    async fn test_guess_nonexistent_game() {
        let manager = GameManager::new();
        let result = manager.hangman_guess("#channel1", 'a').await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_quit_nonexistent_game() {
        let manager = GameManager::new();
        let result = manager.hangman_quit("#channel1").await;
        assert!(!result);
    }

    #[tokio::test]
    async fn test_shared_game_state() {
        let manager = GameManager::new();
        manager.new_hangman("#channel1").await;
        
        // Get initial game state
        let game1 = manager.hangman_game("#channel1").await.unwrap();
        let word = game1.word().to_string();
        let first_letter = word.chars().next().unwrap();
        drop(game1);
        
        // Make a guess
        manager.hangman_guess("#channel1", first_letter).await;
        
        // Verify the same game is modified
        let game2 = manager.hangman_game("#channel1").await.unwrap();
        assert!(game2.guessed().contains(first_letter));
    }
}
