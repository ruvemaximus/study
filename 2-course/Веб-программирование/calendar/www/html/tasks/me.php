<?php include_once '../dbproxy.php' ?>
<?php session_start() ?>


<?php
$filter = $_GET['filter'] ?? 'current';
if (!isset($_SESSION['user_id'])) {
    header('Location: /auth/login.php');
    exit();
}

$dbProxy = new DBProxy();
$user_id = $_SESSION['user_id'];
$conn = $dbProxy->getDBConnection();
$currentDateTime = date('Y-m-d H:i:s');

if ($filter === 'expired') { 
    $stmt = $conn->prepare('SELECT id, type, theme, location, date_time FROM tasks WHERE user_id=:user_id AND date_time < :currentDateTime');
    $stmt->bindParam(':currentDateTime',$currentDateTime);
}

else if ($filter === 'current') { 
    $stmt = $conn->prepare('SELECT id, type, theme, location, date_time FROM tasks WHERE user_id=:user_id AND date_time >= :currentDateTime');
    $stmt->bindParam(':currentDateTime',$currentDateTime);
}

else if ($filter === 'completed') { 
    $stmt = $conn->prepare('SELECT id, type, theme, location, date_time FROM tasks WHERE user_id=:user_id AND completed="on"');
}

else if ($filter === 'date') { 
    $date = $_POST['date'];

    // Определение начала и конца дня
    $startOfDay = $date . 'T00:00:00';
    $endOfDay = $date . 'T23:59:59';
    
    $stmt = $conn->prepare('SELECT id, type, theme, location, date_time FROM tasks WHERE user_id=:user_id AND date_time>=:startOfDay AND date_time<=:endOfDay');
    $stmt->bindParam(':startOfDay', $startOfDay);
    $stmt->bindParam(':endOfDay', $endOfDay);
}

$stmt->bindParam(':user_id', $user_id);
$userTasks = $stmt->execute();
?>

<head>
    <title>Мои задачи</title>
    <link rel="stylesheet" href="/static/style.css">
</head>

<body>
    <h1> С возвращением, <?php echo $_SESSION['username'] ?>!</h1>
    <a href="/auth/logout.php">Выйти</a>
    <div class="tag-panel">
        <a class="tag <?php echo $filter==='current'?'active':''?>" href="/tasks/me.php?filter=current">Текущие задачи</a>
        <a class="tag <?php echo $filter==='expired'?'active':''?>" href="/tasks/me.php?filter=expired">Просроченные задачи</a>
        <a class="tag <?php echo $filter==='completed'?'active':''?>" href="/tasks/me.php?filter=completed">Выполненные</a>
    </div>
    <form method="post" action="/tasks/me.php?filter=date">
        <input type="date" name="date">
        <input class="btn" type="submit" value="Показать задачи"> 
    </form>

    <a class="btn" href="./create.php">+ Создать задачу</a>
    <table>
        <?php
        while ($row = $userTasks->fetchArray(SQLITE3_ASSOC)) {
            echo '<tr>';
            echo '<td>' . $row['type'] . '</td>';
            echo '<td>' . $row['theme'] . '</td>';
            echo '<td>' . $row['location'] . '</td>';
            echo '<td>' . $row['date_time'] . '</td>';
            echo '<td><a href="/tasks/edit.php?task_id=' . $row['id'] . '">Изменить</a></td>';
            echo '</tr>';
        }
        ?>
    </table>
</body>