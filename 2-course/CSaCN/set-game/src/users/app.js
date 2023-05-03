import { Router } from 'express';

import { expectedBody } from '../core/middlewares.js'
import { login } from './auth.js';
import { Users } from './model.js';


const router = Router();

router.route('/')
    .get(async function (req, res) {
        const users = await Users.all();
        return res.status(200).json(users);
    })

    .post(expectedBody(['username', 'password']), async function (req, res) {
        const token = await Users.create(req.body.username, req.body.password);
        return res.status(201).json(token);
    });


router.route('/me')
    .get(login, async function (req, res) {
        return res.status(200).json(req.user);
    })
    .put(login, async function (req, res) {
        const updatedUser = await Users.update(req.user.id, {
            username: req.body.username ?? req.user.username,
            state: req.body.state ?? req.user.state
        })
        return res.status(200).json(updatedUser);
    })


router.get('/:user_id', async (req, res) => {
    const user = await Users.get(req.params.user_id);
    if (user === undefined) {
        return res.sendStatus(404);
    }
    return res.status(200).json(user)
});


export default router;