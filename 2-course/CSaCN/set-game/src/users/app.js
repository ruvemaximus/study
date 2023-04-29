import { Router } from 'express';
import auth from './auth.js';

import DBManager from "../db/db_manager.cjs";
import {getUser } from './model.js';


const router = Router();
const manager = new DBManager();


router.get('/', (req, res) => {
    manager.db.all('SELECT id, username FROM Users', (err, users) => {
        return res.status(200).json(users);
    });
});


router.get('/me', (req, res) => {
    const user = auth(req.headers);
    return res.json({a: 1});
});


router.get('/:user_id', (req, res) => {
    getUser({id: req.params.user_id}, (user) => {
        if (user === undefined){
            return res.status(404).json({
                detail: 'User not found'
            });
        }

        return res.status(200).json({
            'id':  user.id,
            'username': user.username
        });
    });
});


router.post('/', (req, res) => {
    if (!req.body.username || !req.body.password) {
        let undefinedFields = [];

        for (const expectedField of ['username', 'password']) {
            if (req.body[expectedField] === undefined) {
                undefinedFields.push(expectedField);
            }
        }
        const requiredFields = undefinedFields.join(', ');

        return res
            .status(400)
            .send(`Fields expected: [${requiredFields}]`);
    }

    const sql = 'INSERT INTO Users(username, password) VALUES(?,?)';
    const params = [req.body.username, req.body.password];

    manager.db.run(sql, params, (err) => {
        if (err) throw err;
    });

    manager.db.get('SELECT last_insert_rowid() as user_id', (err, row) => {
        return res.status(201).json({'id': row.user_id});
    });
});


router.delete('/:user_id', (req, res) => {
    const sql = 'DELETE FROM Users WHERE id=?';
    const params = req.params.user_id;

    manager.db.run(sql, params, (err) => {
        if (err) throw err;
        return res.status(200).json({message: `Deleted ${req.params.user_id}`});
    });
});

export default router;