import express from 'express';

import { SuccessResponse, ErrorResponse } from '../core/responses.js';
import { login } from '../users/auth.js';
import { Users } from '../users/model.js';
import { Games } from './model.js';


const router = express.Router();


router.get('/', async (req, res) => {
    const isFinished = req.query.finished ?? 0;
    const games = await Games.filter({finished: isFinished});
    return res.status(200).json(games);
});


router.post('/', async (req, res) => {
    const game = await Games.create();
    return res.status(201).json(game);
});


router.route('/:game_id')
    .get(async function (req, res) {
        const game = await Games.get(req.params.game_id);
        if (game === undefined) {
            return res.sendStatus(404);
        }

        return res.status(200).json(game);
    })
    .delete(async function (req, res) {
        const game_id = req.params.game_id;
        await Games.delete(game_id);
        return res.status(200).json(new SuccessResponse(`Game deleted: ${game_id}`));
    })


router.put('/:game_id/finish', async (req, res) => {
    const game = await Games.get(req.params.game_id);
    await game.finish();

    return res.status(200).json(new SuccessResponse(`Game finished: ${game.id}`));
});


router.post('/:game_id/join', login, async (req, res) => {
    const game = await Games.get(req.params.game_id);
    const user = req.user.id;

    await user.join(game);
    return res.sendStatus(200);
});


export default router;