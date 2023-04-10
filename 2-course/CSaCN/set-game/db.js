const sqlite3 = require('sqlite3');
const db = new sqlite3.Database('./db.sqlite3');

module.exports = {
    init: function() {
        db.serialize(() => {
            db.run("CREATE TABLE IF NOT EXISTS Users( \
                id INTEGER PRIMARY KEY AUTOINCREMENT, \
                username TEXT, \
                password TEXT \
            )");
        });
        db.close();
    }
}