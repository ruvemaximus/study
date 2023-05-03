import DBManager from "../db/db_manager.cjs";


export class Games {

    static manager = new DBManager();

    constructor(id, finished) {
        this.id = id;
        this.finished = finished;
    }

    static async get(id) {
        return this.manager.get('SELECT id, finished FROM Games WHERE id=?', [id])
            .then(game => new Games(game.id, game.finished))
            .catch(error => { throw error });
    }

    static async delete(id) {
        return this.manager.run('DELETE FROM Games WHERE id=?', [id])
            .then(manager.run('DELETE FROM GameUsers WHERE game_id=?', [id]));
    }

    static async create() {
        return this.manager.run('INSERT INTO Games(finished) VALUES(0)')
            .then(game => game)
            .catch(error => { throw error });
    }

    static async filter({finished}) {
        return this.manager.all('SELECT id, finished FROM Games WHERE finished=?', [finished])
            .then(games => games);
    }

    async finish() {
        return this.manager.run('UPDATE Games SET finished=1 WHERE id=?', [this.id])
            .then(game => game)
            .catch(err => { throw err });
    }
}
