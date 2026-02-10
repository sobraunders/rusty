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
                username TEXT NOT NULL UNIQUE,
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
