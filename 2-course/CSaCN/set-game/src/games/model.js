import DBManager from "../db/db_manager.cjs";

const manager = new DBManager();


export class Games {
    constructor(id, finished) {
        this.id = id;
        this.finished = finished;
    }

    static async get(id) {
        return manager.get('SELECT id, finished FROM Games WHERE id=?', [id])
            .then(game => new Games(game.id, game.finished))
            .catch(error => { throw error });
    }

    static async delete(id) {
        return manager.run('DELETE FROM Games WHERE id=?', [id])
            .then(manager.run('DELETE FROM GameUsers WHERE game_id=?', [id]));
    }

    static async create() {
        return manager.run('INSERT INTO Games(finished) VALUES(0)')
            .then(game => game)
            .catch(error => { throw error });
    }

    static async filter({finished}) {
        return manager.all('SELECT id, finished FROM Games WHERE finished=?', [finished])
            .then(games => games);
    }

    async finish() {
        return manager.run('UPDATE Games SET finished=1 WHERE id=?', [this.id])
            .then(game => game)
            .catch(err => { throw err });
    }
}


export async function joinGame(user, game) {
    manager.run('INSERT INTO GameUsers(user_id, game_id) VALUES(?,?)', [user.id, game.id])
        .then(game => game)
        .catch(error => { throw error });
}