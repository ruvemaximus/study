<?php include_once '../dbproxy.php' ?>
<?php
$dbProxy = new DBProxy();
$conn = $dbProxy->getDBConnection();

$task_id = $_GET['task_id'];
$action = $_GET['action'] ?? 'save';

if ($_SERVER['REQUEST_METHOD'] === 'GET') { 
    $stmt = $conn->prepare('SELECT theme, type, date_time, location,  duration, comment, completed FROM tasks WHERE id=:task_id');
    $stmt->bindParam(':task_id', $task_id);
    $result = $stmt->execute();
    if ($row = $result->fetchArray()) {
        $theme = $row['theme'];
        $type = $row['type'];
        $location = $row['location'];
        $date_time = $row['date_time'];
        $duration = $row['duration'];
        $comment = $row['comment'];
        $completed = $row['completed'];
    } else { 
        echo 'Ошибка выполнения запроса';
    }
}

if ($_SERVER['REQUEST_METHOD'] === 'POST' && $action === 'save') {
    $theme = $_POST['theme'];
    $type = $_POST['type'];
    $location = $_POST['location'];
    $date_time = $_POST['date_time'];
    $duration = $_POST['duration'];
    $comment = $_POST['comment'];
    $completed = $_POST['completed'] ?? 'off';

    $stmt = $conn->prepare('UPDATE tasks SET theme=:theme, type=:type, location=:location, date_time=:date_time, duration=:duration, comment=:comment, completed=:completed WHERE id=:task_id');

    $stmt->bindParam(':theme', $theme);
    $stmt->bindParam(':type', $type);
    $stmt->bindParam(':location', $location);
    $stmt->bindParam(':date_time', $date_time);
    $stmt->bindParam(':duration', $duration);
    $stmt->bindParam(':comment', $comment);
    $stmt->bindParam(':completed', $completed);
    $stmt->bindParam(':task_id', $task_id);

    if ($stmt->execute()) { 
       echo 'Задача успешно обновленна';
    } else { 
        echo 'Ошибка при обновлении данных задачи';
    }
}

if ($_SERVER['REQUEST_METHOD'] === 'POST' && $action === 'delete') {
    $stmt = $conn->prepare('DELETE FROM tasks WHERE id=:task_id');
    $stmt->bindParam(':task_id', $task_id);
    if ($stmt->execute()) {
        header('Location: /tasks/me.php');
        exit();
    } else { 
        echo 'Произошла ошибка при удалении задачи';
    }
}

?>

<head>
    <title>Редактирование задачи</title>
    <link rel="stylesheet" href="/static/style.css">
</head>

<body>
    <h2>Редактирование задачи</h2>
    <a href="/tasks/me.php">Назад</a>

    <form method="POST" action="<?php echo '/tasks/edit.php?action=save&task_id=' . $task_id ?>"> 
        <?php include './templates/task_form.php' ?> 
        <input type="submit" value="Сохранить">
    </form>

    <form method="POST" action="<?php echo '/tasks/edit.php?action=delete&task_id=' . $task_id ?>">
        <input type="submit" value="Удалить">
    </form>
</body>