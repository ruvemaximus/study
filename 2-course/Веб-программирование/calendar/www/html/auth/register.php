<?php include_once('../dbproxy.php'); ?>
<?php
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $username = $_POST['username'];
    $password = $_POST['password'];
    $confirmPassword = $_POST['confirm_password'];

    if (isset($_SESSION['user_id'])) {
        header('Location: ' . '/tasks/me.php');
        exit();
    }
    
    // Проверка на совпадение паролей
    if ($password !== $confirmPassword) {
        die('Пароли не совпадают');
    }
    
    // Хеширование пароля
    $hashedPassword = password_hash($password, PASSWORD_DEFAULT);
    
    // Сохранение пользователя в базе данных
    $dbProxy = new DBProxy();
    $db = $dbProxy->getDBConnection();
    $stmt = $db->prepare('INSERT INTO users (username, password) VALUES (:username, :password)');
    $stmt->bindParam(':username', $username);
    $stmt->bindParam(':password', $hashedPassword);
    
    if ($stmt->execute()) {
        header('Location: ./login.php');
        exit();
    } else {
        echo 'Ошибка при регистрации пользователя';
    }
}
?>
<!DOCTYPE html>
<html>
<head>
    <title>Регистрация</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <h2>Регистрация</h2>
    <form method="POST" action="register.php">
        <label for="username">Имя пользователя:</label>
        <input type="text" id="username" name="username" required><br><br>
        
        <label for="password">Пароль:</label>
        <input type="password" id="password" name="password" required><br><br>
        
        <label for="confirm_password">Подтвердите пароль:</label>
        <input type="password" id="confirm_password" name="confirm_password" required><br><br>
        
        <input type="submit" value="Зарегистрироваться">
        <a href="./login.php">Войти</a>
    </form>
</body>
</html>
