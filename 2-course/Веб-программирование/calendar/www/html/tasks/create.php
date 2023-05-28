<?php include_once '../dbproxy.php' ?>
<?php
session_start();
$type = '' ;
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $user_id = $_SESSION['user_id'];
    $theme = $_POST['theme'];
    $type = $_POST['type'];
    $location = $_POST['location'];
    $date_time = $_POST['date_time'];
    $duration = $_POST['duration'];
    $comment = $_POST['comment'];
    
    // Сохранение задачи в базе данных
    $dbProxy = new DBProxy();
    $conn = $dbProxy->getDBConnection();
    $stmt = $conn->prepare('INSERT INTO tasks (theme, type, location, date_time, duration, comment, user_id) VALUES (:theme, :type, :location, :date_time, :duration, :comment, :user_id)');
    $stmt->bindParam(':theme', $theme);
    $stmt->bindParam(':type', $type);
    $stmt->bindParam(':location', $location);
    $stmt->bindParam(':date_time', $date_time);
    $stmt->bindParam(':duration', $duration);
    $stmt->bindParam(':comment', $comment);
    $stmt->bindParam(':user_id', $user_id);
    
    if ($stmt->execute()) {
        echo 'Задача успешно создана';
    } else {
        echo 'Ошибка при создании задачи';
    }
}
?>
<!DOCTYPE html>
<html>
<head>
    <title>Создать задачу</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <h2>Создать задачу</h2>
    <a href="./me.php">Назад</a>
    <form method="POST" action="create.php">
        <?php include './templates/task_form.php' ?>
        <input type="submit" value="Создать">
    </form>
</body>
</html>
