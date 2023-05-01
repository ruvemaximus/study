const sqlite3 = require('sqlite3')


class DBManager {
    constructor(dbPath='./db.sqlite3') {
        this.db = new sqlite3.Database(dbPath);
    }

    generateSchemas() {
        const sql = `
        CREATE TABLE IF NOT EXISTS Users(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        ); CREATE TABLE IF NOT EXISTS Tokens(
            token TEXT PRIMARY KEY,
            user_id INT
        ); CREATE TABLE IF NOT EXISTS Games(
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            finished INTEGER
        ); CREATE TABLE IF NOT EXISTS GameUsers(
            game_id INT NOT NULL,
            user_id INT NOT NULL
        )`;
        this.db.exec(sql, (err) => {
            if (err) {
                console.error('Creating tables failed: ' + err.message);
            }
        });
    }

    get(...args) { this.db.get(...args); }
    run(...args) { this.db.run(...args); }
}


module.exports = DBManager