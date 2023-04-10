const express = require('express');
const sqlite3 = require('sqlite3');
const db = new sqlite3.Database('./db.sqlite3');

const router = express.Router();

router.get('/', (req, res) => {
    res.send('Welcome to User page!');
});

router.get('/:user_id', (req, res) => {
    const user_id = +req.params.user_id;
    db.get("SELECT username FROM Users WHERE id = ?", [user_id], (err, username) => {
        res.status(200).json({'username': username});
    });
});

router.post('/', (req, res) => {
    const user = req.body;

    if (!user.username || !user.password) {
        res.status(400).send('Неверный запрос на регистрацию пользователя');
    }

    db.run('INSERT INTO Users(`username`,`password`) VALUES(?,?)', 
        [req.body.username, req.body.password], 
        (err) => { if (err) return console.error(err.message); }
    );

    res.status(201).send('Пользователь успешно создан');
});

module.exports = router