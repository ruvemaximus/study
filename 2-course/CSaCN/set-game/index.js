const express = require('express');

const users = require('./users/app');
const db = require('./db');

const app = express();

app.use(express.json());

// Конфигурация сервера
const ipAddr = '127.0.0.1';
const port = 3000

app.get('/', (req, res) => {
    res.send('Hello, world');
})

// Подключаем приложения 
app.use('/users', users);


app.listen(port, () => {
    console.log('Настройка базы данных');
    db.init();
    console.log(`Игровой сервер доступен по адресу: http://${ipAddr}:${port}`)
})