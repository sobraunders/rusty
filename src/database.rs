use rusqlite::{Connection, params, OptionalExtension};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        
        // Create users_data table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users_data (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                data_key TEXT NOT NULL,
                data_value TEXT NOT NULL,
                UNIQUE(username, data_key)
            )",
            [],
        )?;

        // Create permissions table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS permissions (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                permission_level INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn set_user_data(&self, username: &str, key: &str, value: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT OR REPLACE INTO users_data (username, data_key, data_value)
             VALUES (?, ?, ?)",
            params![username, key, value],
        )?;
        Ok(())
    }

    pub fn get_user_data(&self, username: &str, key: &str) -> Result<Option<String>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT data_value FROM users_data WHERE username = ? AND data_key = ?"
        )?;
        
        let value = stmt.query_row(params![username, key], |row| {
            row.get(0)
        }).optional()?;
        
        Ok(value)
    }

    pub fn delete_user_data(&self, username: &str, key: &str) -> Result<bool, rusqlite::Error> {
        let rows = self.conn.execute(
            "DELETE FROM users_data WHERE username = ? AND data_key = ?",
            params![username, key],
        )?;
        Ok(rows > 0)
    }

    pub fn list_user_data(&self, username: &str) -> Result<Vec<(String, String)>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT data_key, data_value FROM users_data WHERE username = ?"
        )?;
        
        let data = stmt.query_map(params![username], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(data)
    }

    pub fn grant_permission(&self, username: &str, level: i32) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT OR REPLACE INTO permissions (username, permission_level) VALUES (?, ?)",
            params![username, level],
        )?;
        Ok(())
    }

    pub fn revoke_permission(&self, username: &str) -> Result<bool, rusqlite::Error> {
        let rows = self.conn.execute(
            "DELETE FROM permissions WHERE username = ?",
            params![username],
        )?;
        Ok(rows > 0)
    }

    pub fn get_permission_level(&self, username: &str) -> Result<i32, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT permission_level FROM permissions WHERE username = ?"
        )?;
        
        let level = stmt.query_row(params![username], |row| {
            row.get(0)
        }).optional()?;
        
        Ok(level.unwrap_or(0)) // Default to 0 (no permissions)
    }

    pub fn list_users_with_permissions(&self) -> Result<Vec<(String, i32)>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT username, permission_level FROM permissions ORDER BY username"
        )?;
        
        let users = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Database {
        Database::new(":memory:").expect("Failed to create in-memory database")
    }

    #[test]
    fn test_set_and_get_user_data() {
        let db = create_test_db();
        db.set_user_data("alice", "key1", "value1").expect("Failed to set data");
        
        let value = db.get_user_data("alice", "key1").expect("Failed to get data");
        assert_eq!(value, Some("value1".to_string()));
    }

    #[test]
    fn test_get_nonexistent_user_data() {
        let db = create_test_db();
        let value = db.get_user_data("nobody", "nonexistent").expect("Failed to query");
        assert_eq!(value, None);
    }

    #[test]
    fn test_get_nonexistent_key() {
        let db = create_test_db();
        db.set_user_data("alice", "key1", "value1").expect("Failed to set");
        
        let value = db.get_user_data("alice", "key2").expect("Failed to get");
        assert_eq!(value, None);
    }

    #[test]
    fn test_set_user_data_replace() {
        let db = create_test_db();
        db.set_user_data("bob", "color", "blue").expect("Failed to set");
        db.set_user_data("bob", "color", "red").expect("Failed to replace");
        
        let value = db.get_user_data("bob", "color").expect("Failed to get");
        assert_eq!(value, Some("red".to_string()));
    }

    #[test]
    fn test_delete_user_data() {
        let db = create_test_db();
        db.set_user_data("charlie", "item", "sword").expect("Failed to set");
        
        let deleted = db.delete_user_data("charlie", "item").expect("Failed to delete");
        assert!(deleted);
        
        let value = db.get_user_data("charlie", "item").expect("Failed to get");
        assert_eq!(value, None);
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let db = create_test_db();
        let deleted = db.delete_user_data("nobody", "key").expect("Failed to delete");
        assert!(!deleted);
    }

    #[test]
    fn test_list_user_data() {
        let db = create_test_db();
        db.set_user_data("diana", "key1", "value1").expect("Failed to set");
        db.set_user_data("diana", "key2", "value2").expect("Failed to set");
        db.set_user_data("diana", "key3", "value3").expect("Failed to set");
        
        let data = db.list_user_data("diana").expect("Failed to list");
        assert_eq!(data.len(), 3);
        assert!(data.contains(&("key1".to_string(), "value1".to_string())));
        assert!(data.contains(&("key2".to_string(), "value2".to_string())));
        assert!(data.contains(&("key3".to_string(), "value3".to_string())));
    }

    #[test]
    fn test_list_user_data_empty() {
        let db = create_test_db();
        let data = db.list_user_data("nonexistent").expect("Failed to list");
        assert!(data.is_empty());
    }

    #[test]
    fn test_grant_permission() {
        let db = create_test_db();
        db.grant_permission("admin", 10).expect("Failed to grant");
        
        let level = db.get_permission_level("admin").expect("Failed to get level");
        assert_eq!(level, 10);
    }

    #[test]
    fn test_get_permission_level_default() {
        let db = create_test_db();
        let level = db.get_permission_level("nobody").expect("Failed to get level");
        assert_eq!(level, 0);
    }

    #[test]
    fn test_grant_permission_update() {
        let db = create_test_db();
        db.grant_permission("user", 5).expect("Failed to grant");
        db.grant_permission("user", 8).expect("Failed to update");
        
        let level = db.get_permission_level("user").expect("Failed to get level");
        assert_eq!(level, 8);
    }

    #[test]
    fn test_revoke_permission() {
        let db = create_test_db();
        db.grant_permission("moderator", 7).expect("Failed to grant");
        
        let revoked = db.revoke_permission("moderator").expect("Failed to revoke");
        assert!(revoked);
        
        let level = db.get_permission_level("moderator").expect("Failed to get level");
        assert_eq!(level, 0);
    }

    #[test]
    fn test_revoke_nonexistent_permission() {
        let db = create_test_db();
        let revoked = db.revoke_permission("nobody").expect("Failed to revoke");
        assert!(!revoked);
    }

    #[test]
    fn test_list_users_with_permissions() {
        let db = create_test_db();
        db.grant_permission("alice", 10).expect("Failed to grant");
        db.grant_permission("bob", 5).expect("Failed to grant");
        db.grant_permission("charlie", 15).expect("Failed to grant");
        
        let users = db.list_users_with_permissions().expect("Failed to list");
        assert_eq!(users.len(), 3);
        assert!(users.contains(&("alice".to_string(), 10)));
        assert!(users.contains(&("bob".to_string(), 5)));
        assert!(users.contains(&("charlie".to_string(), 15)));
    }

    #[test]
    fn test_list_users_with_permissions_empty() {
        let db = create_test_db();
        let users = db.list_users_with_permissions().expect("Failed to list");
        assert!(users.is_empty());
    }

    #[test]
    fn test_separate_user_data_namespaces() {
        let db = create_test_db();
        db.set_user_data("user1", "key", "value1").expect("Failed to set");
        db.set_user_data("user2", "key", "value2").expect("Failed to set");
        
        let val1 = db.get_user_data("user1", "key").expect("Failed to get");
        let val2 = db.get_user_data("user2", "key").expect("Failed to get");
        
        assert_eq!(val1, Some("value1".to_string()));
        assert_eq!(val2, Some("value2".to_string()));
    }
}
