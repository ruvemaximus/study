import DBManager from "../db/db_manager.cjs";


export async function login(req, res) {
    const manager = new DBManager();
    const bearerToken = req.headers.authorization;

    if (bearerToken === undefined) {
        return res.status(401).json({detail: 'Expected token'});
    }

    const [prefix, token] = bearerToken.split(' ');

    if (prefix !== 'Bearer') {
        return res.status(401).json({detail: 'Wrong bearer token format'});
    }

    const user = await manager.get('SELECT id FROM Users WHERE token=?', [token]).then(user => user);

    if (user === undefined) {
        return res.status(401).json({detail: 'Wrong token'});
    }

    return user;
}