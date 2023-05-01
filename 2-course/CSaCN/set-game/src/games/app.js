import express from 'express';

import { SuccessResponse, ErrorResponse } from '../core/response_models.js';
import { login } from '../users/auth.js';
import { Games, joinGame } from './model.js';


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


router.get('/:game_id', async (req, res) => {
    const game = await Games.get(req.params.game_id);
    if (game === undefined) {
        return res.status(404).json(new ErrorResponse('Game not found!'));
    }

    return res.status(200).json(game);
});


router.delete('/:game_id', async (req, res) => {
    const game_id = req.params.game_id;
    await Games.delete(game_id);
    return res.status(200).json(new SuccessResponse(`Game deleted: ${game_id}`));
});


router.put('/:game_id/finish', async (req, res) => {
    const game = await Games.get(req.params.game_id);
    await game.finish();

    return res.status(200).json(new SuccessResponse(`Game finished: ${game.id}`));
});


router.post('/:game_id/join', async (req, res) => {
    const user = await login(req, res);
    const game = await Games.get(req.params.game_id);

    await joinGame(user, game);

    return res.status(200).json(new SuccessResponse('Game joined successfully'));
});


export default router;