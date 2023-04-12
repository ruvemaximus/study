import { Router } from 'express';
import { getUser, createUser } from './model.js';

const router = Router();

router.get('/', (req, res) => {
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
        res.status(201).json(user);
    });
});

export default router