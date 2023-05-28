<?php include_once('../dbproxy.php'); ?>
<?php
session_start();
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $username = $_POST['username'];
    $password = $_POST['password'];
    
    // Проверка учетных данных пользователя
    $dbProxy = new DBProxy();
    $db = $dbProxy->getDBConnection();
    $stmt = $db->prepare('SELECT id, password FROM users WHERE username = :username');
    $stmt->bindParam(':username', $username);
    $result = $stmt->execute();
    
    if ($row = $result->fetchArray(SQLITE3_ASSOC)) {
        $hashedPassword = $row['password'];
        
        // Проверка пароля
        if (password_verify($password, $hashedPassword)) {
            $_SESSION['user_id'] = $row['id'];
            $_SESSION['username'] = $username;
            header('Location: /tasks/me.php');
            echo 'Успешный вход. Добро пожаловать, ' . $username;
            exit();
        } else {
            echo 'Неправильное имя пользователя или пароль';
        }
    } else {
        echo 'Неправильное имя пользователя или пароль';
    }
}
?>


<!DOCTYPE html>
<html>
<head>
    <title>Вход</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <h2>Вход</h2>
    <form method="POST" action="login.php">
        <label for="username">Имя пользователя:</label>
        <input type="text" id="username" name="username" required><br><br>
        
        <label for="password">Пароль:</label>
        <input type="password" id="password" name="password" required><br><br>
        
        <input type="submit" value="Войти">
        <a href="./register.php">Зарегистрироваться</a>
    </form>
</body>
</html>
