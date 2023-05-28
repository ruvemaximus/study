import express from 'express';

import users from './users/app.js';
import games from './games/app.js';
import DBManager from './db/db_manager.cjs';
import { SuccessResponse } from './core/responses.js';


const IP_ADDR = '127.0.0.1';
const PORT = 3000;

const app = express();

app.get('/', (req, res) => {
    return res.status(200).json(new SuccessResponse('Server working!'));
})

app.use(express.json());
app.use('/users', users);
app.use('/games', games);


app.listen(PORT, () => {
    const manager = new DBManager();

    manager.generateSchemas();
    console.log(`Server started at: http://${IP_ADDR}:${PORT}`);
})