<?php
class DBProxy {
    static private $schemaGenerated = false;
    private $db;

    function __construct() {
        $this->db = new SQLite3('/var/www/html/calendar.db'); 
        if (!DBProxy::$schemaGenerated) { 
            $this->generateSchemas();
        }
    }

    function getDBConnection() {
        return $this->db;
    }

    private function generateSchemas() { 
        // Создание таблицы пользователей
        $this->db->exec('CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )');

        // Создание таблицы задач
        $this->db->exec('CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            theme TEXT NOT NULL,
            type TEXT NOT NULL,
            location TEXT,
            date_time DATETIME NOT NULL,
            duration INTEGER,
            comment TEXT,
            completed BOOLEAN DEFAULT FALSE, 
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )');
        DBProxy::$schemaGenerated = true;
    }
}

?>