Создать докер-образ 
```bash
docker build -t set-game-server .
```

Запустить контейнер
```bash
docker run -p 3000:3000 set-game-server
```