import { Router } from 'express';

import { SuccessResponse, ErrorResponse } from '../core/response_models.js';
import { login } from './auth.js';
import { getUser, getAllUsers, createUser } from './model.js';
import { Users } from './model.js';


const router = Router();


router.get('/', async (req, res) => {
    const users = await getAllUsers();
    return res.status(200).json(users);
});


router.get('/me', async (req, res) => {
    const userData = await login(req, res);
    const user = await getUser(userData.id);

    return res.status(200).json(user);
});


router.put('/me', async (req, res) => {
    
});


router.get('/:user_id', async (req, res) => {
    const user = await getUser(req.params.user_id);
    if (user === undefined) {
        return res.status(404).json({detail: 'User not found'});
    }
    return res.status(200).json(user)
});


router.post('/', async (req, res) => {
    if (!req.body.username || !req.body.password) {
        const expectedFields = ['username', 'password'].filter(field => req.body[field] === undefined);
        return res.status(422).json(new ErrorResponse(`Fields expected: ${expectedFields.join(', ')}`));
    }

    const token = await createUser(req.body.username, req.body.password);
    return res.status(201).json(token);
});

export default router;