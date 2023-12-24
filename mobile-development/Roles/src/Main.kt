import java.io.FileInputStream
import java.util.*

fun main() {
    val sc = Scanner(FileInputStream("roles.txt"))

    val roles = sortedMapOf<String, MutableList<String>>()
    sc.nextLine()
    while (sc.hasNextLine()) {
        val line = sc.nextLine()
        if ("textLines" in line) {
            break
        }
        roles[line] = arrayListOf()
    }

    var i = 1
    while (sc.hasNextLine()) {
        val line = sc.nextLine()
        val (role, text) = line.split(':', limit = 2)
        roles[role]!!.add("$i) $text")
        i++
    }

    for (role in roles) {
        val text = role.value.joinToString("\n")
        println("${role.key}:\n$text\n")
    }
}
