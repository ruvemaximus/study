import { Router } from 'express';
import { getUser, createUser } from './model.js';
import { createSession, tokenExpired } from '../utils/session.js';
import SQLiteManager from "../db/sqlite_manager.cjs";

const router = Router();

router.get('/', (req, res) => {
    const auth = req.headers.authorization;
    const manager = new SQLiteManager();

    if (!auth) {
        return res.status(401).send('Отсутствует токен авторизации');
    }
    manager.get("SELECT user_id FROM Sessions WHERE token=?", [auth], (err, user) =>{
        console.log(user);
    });

    return res.send('Welcome to User page!');
    
});

router.get('/:user_id', (req, res) => {
    return getUser(req.params.user_id, (user) => {
        res.status(200).json(user);
    });
});

router.post('/', (req, res) => {
    const user = req.body;

    if (!user.username || !user.password) {
        return res.status(400).send('Неверный запрос на регистрацию пользователя');
    }

    createUser(user, (user) => {
        const token = createSession(user.user_id);
        return res.status(201).json({'user_id': user.user_id, 'token': token});
    });
});

export default router
