import express from 'express';

import users from './users/app.js';
import DBManager from './db/db_manager.cjs';


const app = express();

app.use(express.json());

// Конфигурация сервера
const IP_ADDR = '127.0.0.1';
const PORT = 3000;


app.get('/', (req, res) => {
    return res.status(200).json(
        {message: 'Server working'}
    );
})


// Подключаем приложения 
app.use('/users', users);


app.listen(PORT, () => {
    const manager = new DBManager();

    manager.generateSchemas();
    console.log(`Start server at: http://${IP_ADDR}:${PORT}`);

})