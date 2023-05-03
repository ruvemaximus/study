import { generateToken } from '../utils/token.js';
import DBManager from "../db/db_manager.cjs";


export class Users {

    static manager = new DBManager();

    constructor({id, username, state}) {
        this.id = id;
        this.username = username;
        this.state = state;
    }

    static async get(id) {
        return this.manager.get('SELECT id, username, state FROM Users WHERE id=?', [id])
            .then(user => new Users({...user}))
    }

    static async getByToken(token) {
        return this.manager.get('SELECT id, username, state FROM Users WHERE token=?', [token])
            .then(user => new Users({...user}))
            .catch(err => { throw err; });
    }

    static async all() {
        return this.manager.all('SELECT id, username, state FROM Users')
            .then(users => users);
    }

    static async create(username, password) {
        const token = generateToken();
        this.manager.run('INSERT INTO Users (username, password, token) VALUES(?,?,?)', [username, password, token]);
        return {token: token};
    }

    static async update(id, {username, state}) {
        return this.manager.run('UPDATE Users SET username=?, state=? WHERE id=?', [username, state, id])
            .then(user => user)
    }

    async join(game) {
        return Users.manager.run('INSERT INTO GameUsers(game_id, user_id) VALUES(?,?)', [game.id, this.id])
            .then(user => user)
    }

    async leave() {
        return Users.manager.run('DELETE FROM GameUsers WHERE user_id=?', [this.id]);
    }
}