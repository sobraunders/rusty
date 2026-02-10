#!/bin/bash

set -e

echo "🤖 IRC Bot Installation Script"
echo "=============================="
echo ""

# Check if sqlite3 is available
if ! command -v sqlite3 &> /dev/null; then
    echo "❌ Error: sqlite3 is not installed. Please install it first."
    exit 1
fi

# Database file path
DB_FILE="bot_data.db"

# Create or initialize database
echo "📦 Initializing database..."
sqlite3 "$DB_FILE" << EOF
CREATE TABLE IF NOT EXISTS users_data (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    data_key TEXT NOT NULL,
    data_value TEXT NOT NULL,
    UNIQUE(username, data_key)
);

CREATE TABLE IF NOT EXISTS permissions (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    permission_level INTEGER NOT NULL
);
EOF

echo "✅ Database initialized successfully"
echo ""

# Ask for IRC username
read -p "📝 Enter your IRC username (for admin access): " USERNAME

if [ -z "$USERNAME" ]; then
    echo "❌ Error: Username cannot be empty"
    exit 1
fi

# Add user with admin permissions
echo "🔐 Adding admin permissions for user: $USERNAME"
sqlite3 "$DB_FILE" "INSERT OR REPLACE INTO permissions (username, permission_level) VALUES ('$USERNAME', 10);"

# Verify the insertion
RESULT=$(sqlite3 "$DB_FILE" "SELECT permission_level FROM permissions WHERE username = '$USERNAME';")

if [ "$RESULT" = "10" ]; then
    echo "✅ Admin permissions granted to $USERNAME"
    echo ""
    echo "🎉 Installation complete!"
    echo ""
    echo "Next steps:"
    echo "1. Edit src/main.rs to set your IRC server and starting channels"
    echo "2. Run: cargo run"
    echo ""
    echo "Available commands:"
    echo "  - Public: !ping, !hello, !echo <msg>, !help"
    echo "  - Restricted: !join <#ch>, !leave <#ch>, !set <k> <v>, !get <k>, !del <k>, !list"
    echo "  - Admin: !grant <user> <level>, !revoke <user>, !perms"
else
    echo "❌ Error: Failed to grant admin permissions"
    exit 1
fi
