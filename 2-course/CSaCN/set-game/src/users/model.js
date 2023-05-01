import { generateToken } from '../utils/token.js';
import DBManager from "../db/db_manager.cjs";

const manager = new DBManager();


export class Users {
    constructor(id, username, state) {
        this.id = id;
        this.username = username;
        this.state = state;
    }

    static async get(id) {
        return manager.get('SELECT id, username, state FROM Users WHERE id=?', [id])
            .then(user => new Users(user.id, user.username, user.state))
    }

    async join() {
        return;
    }

    async leave() {
        return;
    }
}

export async function getUser(id) {
    return manager.get('SELECT id, username, state FROM Users WHERE id=?', [id])
        .then(user => user)
        .catch(error => { throw error; });
}


export async function getAllUsers() {
    return manager.all('SELECT id, username, state FROM Users')
        .then(users => users);
}

export async function createUser(username, password) {
    const token = generateToken();
    manager.run('INSERT INTO Users (username, password, token) VALUES(?,?,?)', [username, password, token]);
    return {token: token};
}