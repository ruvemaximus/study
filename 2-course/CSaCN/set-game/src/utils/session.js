import SQLiteManager from "../db/sqlite_manager.cjs";

export function tokenExpired(token) {
    const manager = new SQLiteManager();
    const res = manager.get('SELECT * FROM Sessions WHERE token=?', [token]);

    console.log(res);
}

export function generateToken(length=15) {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let token = '';

    for (let i = 0; i < chars.length; i++) {
        const randomIndex = Math.floor(Math.random() * chars.length);
        token += chars[randomIndex];
    }

    return token.slice(0, length);
}

export function createSession(user_id) {
    const token = generateToken();
    const manager = new SQLiteManager();

    manager.run('INSERT INTO Sessions(user_id, token) VALUES(?, ?)', [user_id, token]);
    return token;
}