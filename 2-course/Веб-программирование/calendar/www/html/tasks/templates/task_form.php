<label for="theme">Тема*</label>
<input type="text" id="theme" name="theme" value="<?php echo $theme ?? '' ?>" required><br><br>

<label for="type">Тип:</label>
<select id="type" name="type">
    <option value="встреча" <?php echo $type === 'встреча' ? 'selected' : '' ?>>Встреча</option> 
    <option value="звонок" <?php echo  $type === 'звонок' ? 'selected' : '' ?>>Звонок</option>
    <option value="совещание" <?php echo $type === 'совещание' ? 'selected' : '' ?>>Совещание</option>
    <option value="дело" <?php echo $type === 'дело' ? 'selected' : '' ?>>Дело</option>
</select><br><br>

<label for="location">Место:</label>
<input type="text" id="location" name="location" value="<?php echo $location ?? '' ?>"><br><br>

<label for="date_time">Дата и время*</label>
<input type="datetime-local" id="date_time" name="date_time" value="<?php echo $date_time ?? '' ?>" required><br><br>

<label for="duration">Длительность (в минутах)</label>
<input type="number" id="duration" name="duration" min="1" value="<?php echo $duration ?? '' ?>"><br><br>

<label for="comment">Комментарий</label><br>
<textarea id="comment" name="comment" rows="4" cols="50"><?php echo $comment ?? '' ?></textarea><br><br>

<label for="completed">Выполнена</label>
<input type="checkbox" name="completed" id="completed" <?php echo $completed === 'on' ? 'checked' : '' ?>>