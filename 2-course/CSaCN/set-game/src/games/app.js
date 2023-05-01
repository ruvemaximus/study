import express from 'express';
import DBManager from '../db/db_manager.cjs';

const router = express.Router();
const manager = new DBManager();

router.get('/', (req, res) => {
    manager.db.all('SELECT id FROM Games', (err, rows) => {
        if (err) throw err;
        return res.status(200).json({
            games: rows
        });
    });
});

router.post('/', (req, res) => {
    manager.db.all('INSERT INTO Games(finished) VALUES (1)', (err, rows) => {
        if (err) throw err;
        return res.status(200).json(rows);
    });
});


export default router;