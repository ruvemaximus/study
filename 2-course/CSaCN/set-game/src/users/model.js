import SQLiteManager from "../db/db_manager.cjs";


export function getUser({id}, response_func) {
    const manager = new SQLiteManager();

    const sql = 'SELECT id, username FROM Users WHERE id=?';
    const params = [id];

    manager.get(sql, params, (err, row) => {
        if (err) throw err;
        return response_func(row);
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