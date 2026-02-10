# Test Suite Summary

This document provides an overview of all tests in the rustirc IRC bot project.

## Test Execution

Run all tests with:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run specific test:
```bash
cargo test test_name
```

## Test Coverage

### Message Parser (`src/message.rs`) - 8 tests

Tests for the `ParsedMessage` struct and command parsing functionality.

| Test | Purpose |
|------|---------|
| `test_parse_command_with_args` | Verify proper parsing of commands with multiple arguments |
| `test_parse_command_without_args` | Verify command parsing without arguments |
| `test_parse_non_command` | Verify non-command messages are handled correctly |
| `test_parse_command_case_insensitive` | Verify commands are converted to lowercase |
| `test_parse_command_with_spaces` | Verify command parsing with irregular whitespace |
| `test_is_command_true` | Verify is_command returns true for valid commands |
| `test_is_command_false` | Verify is_command returns false for non-commands |
| `test_parse_author_none` | Verify handling of messages with no author |

### Database Operations (`src/database.rs`) - 19 tests

Tests for SQLite database operations including user data and permissions.

| Test | Purpose |
|------|---------|
| `test_set_and_get_user_data` | Verify basic set/get operations for user data |
| `test_get_nonexistent_user_data` | Verify None is returned for missing user |
| `test_get_nonexistent_key` | Verify None is returned for missing key |
| `test_set_user_data_replace` | Verify INSERT OR REPLACE functionality |
| `test_delete_user_data` | Verify successful deletion returns true |
| `test_delete_nonexistent_key` | Verify deletion of missing data returns false |
| `test_list_user_data` | Verify listing all user data entries |
| `test_list_user_data_empty` | Verify empty list for non-existent user |
| `test_grant_permission` | Verify permission granting |
| `test_get_permission_level_default` | Verify default permission level is 0 |
| `test_grant_permission_update` | Verify permission level updates |
| `test_revoke_permission` | Verify successful permission revocation |
| `test_revoke_nonexistent_permission` | Verify revocation of non-existent permission returns false |
| `test_list_users_with_permissions` | Verify listing all users with permissions |
| `test_list_users_with_permissions_empty` | Verify empty list when no permissions granted |
| `test_separate_user_data_namespaces` | Verify different users have separate data |

### Hangman Game (`src/games/hangman.rs`) - 17 tests

Tests for the hangman game logic and state management.

| Test | Purpose |
|------|---------|
| `test_new_game_initialization` | Verify new game starts with correct state |
| `test_guess_correct_letter` | Verify correct guess is handled properly |
| `test_guess_wrong_letter` | Verify wrong guess increments wrong_count |
| `test_guess_already_guessed` | Verify duplicate guesses return AlreadyGuessed |
| `test_case_insensitive_guessing` | Verify uppercase letters work |
| `test_display_initially_blanks` | Verify all blanks shown initially |
| `test_display_after_correct_guess` | Verify letter revealed after correct guess |
| `test_win_condition` | Verify win detection when all letters guessed |
| `test_lose_condition` | Verify loss detection at max wrong guesses |
| `test_guessed_letters_string` | Verify guessed letters are tracked |
| `test_wrong_count` | Verify wrong guess counting |
| `test_remaining_guesses` | Verify remaining guess calculation |
| `test_word_getter` | Verify word getter returns correct word |
| `test_word_from_list` | Verify selected words are from the word list |
| `test_game_independence` | Verify separate games don't interfere |

### Permission Utilities (`src/commands/utils.rs`) - 7 tests

Tests for permission checking helper functions.

| Test | Purpose |
|------|---------|
| `test_has_permission_true` | Verify has_permission returns true for permission > 0 |
| `test_has_permission_false` | Verify has_permission returns false for permission 0 |
| `test_has_permission_no_permission` | Verify has_permission returns false for non-existent user |
| `test_is_admin_true` | Verify is_admin returns true for level >= 10 |
| `test_is_admin_false` | Verify is_admin returns false for level < 10 |
| `test_is_admin_no_permission` | Verify is_admin returns false for non-existent user |
| `test_permission_boundary` | Verify level 9 is not admin, level 10 is |

## Test Statistics

- **Total Tests**: 47
- **All Passing**: ✅
- **Coverage Areas**:
  - Message parsing and command extraction
  - Database operations (CRUD)
  - Permission system
  - Permission helpers
  - Hangman game logic

## Database Schema Test Coverage

The database schema was corrected during testing:
- Removed incorrect `UNIQUE` constraint on username in users_data table
- This allows multiple key-value pairs per user
- Current schema uses `UNIQUE(username, data_key)` for proper data isolation

## Notes

- All database tests use in-memory SQLite (`:memory:`) to avoid interference
- Each test creates its own isolated database instance
- Hangman tests verify both positive cases (wins) and negative cases (losses)
- Permission tests verify boundary conditions (exactly level 10 threshold)
