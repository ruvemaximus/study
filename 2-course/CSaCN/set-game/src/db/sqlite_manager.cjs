const sqlite3 = require('sqlite3')


class SQLiteManager {
    constructor(dbPath='./db.sqlite3') {
        this.db = new sqlite3.Database(dbPath);
    }

    createTables() {
        const sql = `
        CREATE TABLE IF NOT EXISTS Users(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );`
        this.db.run(sql);
    }

    get(...args) { this.db.get(...args); }
    run(...args) { this.db.run(...args); }
}


module.exports = SQLiteManager