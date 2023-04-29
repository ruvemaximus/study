import DBManager from "../db/db_manager.cjs";

function auth(headers) {
    const manager = new DBManager();

    if (headers.authorization === undefined) {
        return false;
    }

    const sql = 'SELECT user_id FROM Tokens WHERE token=?';
    const params = headers.authorization;

    manager.db.get(sql, params, (err, row) => {
        console.log(row);
    });
    return true;
}

function _generateToken(user_id) {
    const manager = new DBManager();

    manager.db.run('INSERT INTO Tokens(user_id, token) VALUES(?,?)' [user_id, token], (err) => {
        
    });
}

export default auth;