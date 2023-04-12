import SQLiteManager from "../db/sqlite_manager.cjs";

export function getUser(user_id, fn) {
    const manager = new SQLiteManager();
    manager.get("SELECT `id`, `username` FROM `Users` WHERE `id`=?", [user_id], (err, row) => {
        if (err) throw err;
        return fn(row);
    });
}
export function createUser(user, fn) {
    const manager = new SQLiteManager();
    manager.run('INSERT INTO Users(`username`,`password`) VALUES(?,?)',
        [user.username, user.password], (err) => {
            if (err) throw err;
            manager.get("SELECT last_insert_rowid() as user_id", (err, user) => fn(user));
        }
    );
}