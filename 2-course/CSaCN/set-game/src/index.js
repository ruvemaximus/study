import express, { json } from 'express';

import users from './users/app.js';
import SQLiteManager from './db/sqlite_manager.cjs';

const app = express();

app.use(json());

// Конфигурация сервера
const ipAddr = '0.0.0.0';
const port = 3000

app.get('/', (req, res) => {
    res.send('Привет! Это сервер для игры в SET');
})

// Подключаем приложения 
app.use('/users', users);


app.listen(port, () => {
    const manager = new SQLiteManager();
    try {
        manager.init();
    } catch (error) {
        console.log('Ошибка инициализации базы данных: ', error)
    }

    console.log(`Игровой сервер доступен по адресу: http://${ipAddr}:${port}`)
})