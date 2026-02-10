use crate::database::Database;

pub fn has_permission(db: &Database, username: &str) -> bool {
    match db.get_permission_level(username) {
        Ok(level) => level > 0,
        Err(_) => false,
    }
}

pub fn is_admin(db: &Database, username: &str) -> bool {
    match db.get_permission_level(username) {
        Ok(level) => level >= 10,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Database {
        Database::new(":memory:").expect("Failed to create database")
    }

    #[test]
    fn test_has_permission_true() {
        let db = create_test_db();
        db.grant_permission("user", 5).expect("Failed to grant");
        
        assert!(has_permission(&db, "user"));
    }

    #[test]
    fn test_has_permission_false() {
        let db = create_test_db();
        db.grant_permission("user", 0).expect("Failed to grant");
        
        assert!(!has_permission(&db, "user"));
    }

    #[test]
    fn test_has_permission_no_permission() {
        let db = create_test_db();
        
        assert!(!has_permission(&db, "nobody"));
    }

    #[test]
    fn test_is_admin_true() {
        let db = create_test_db();
        db.grant_permission("admin", 10).expect("Failed to grant");
        
        assert!(is_admin(&db, "admin"));
    }

    #[test]
    fn test_is_admin_false() {
        let db = create_test_db();
        db.grant_permission("user", 5).expect("Failed to grant");
        
        assert!(!is_admin(&db, "user"));
    }

    #[test]
    fn test_is_admin_no_permission() {
        let db = create_test_db();
        
        assert!(!is_admin(&db, "nobody"));
    }

    #[test]
    fn test_is_admin_high_level() {
        let db = create_test_db();
        db.grant_permission("superadmin", 15).expect("Failed to grant");
        
        assert!(is_admin(&db, "superadmin"));
    }

    #[test]
    fn test_permission_boundary() {
        let db = create_test_db();
        db.grant_permission("user1", 9).expect("Failed to grant");
        db.grant_permission("user2", 10).expect("Failed to grant");
        
        assert!(!is_admin(&db, "user1"));
        assert!(is_admin(&db, "user2"));
    }
}
