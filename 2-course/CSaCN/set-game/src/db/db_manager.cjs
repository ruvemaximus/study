const sqlite3 = require('sqlite3');


class DBManager {
    constructor(dbPath='./db.sqlite3') {
        this.db = new sqlite3.Database(dbPath);
    }

    generateSchemas() {
        const sql = `
        CREATE TABLE IF NOT EXISTS Users(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            token TEXT NOT NULL UNIQUE,
            state TEXT DEFAULT NULL
        ); CREATE TABLE IF NOT EXISTS Games(
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            finished INTEGER DEFAULT 0
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

    get(sql, params) {
        return new Promise((resolve, reject) => {
            this.db.get(sql, params, (err, row) => {
                if (err) { reject(err) }
                resolve(row);
            });
        });
    }

    all(sql, params) {
        return new Promise((resolve, reject) => {
            this.db.all(sql, params, (err, rows) => {
                if (err) { reject(err) }
                resolve(rows);
            });
        });
    }

    run(sql, params) {
        return new Promise((resolve, reject) => {
            this.db.run(sql, params, function (err) {
                if (err) { reject(err) }
                resolve({ id: this.lastID })
            });
        });
    }

}


module.exports = DBManager