import java.io.File


data class Tag (
    val tagName: String,
    var content: String,
    val className: String = ""
) {
    fun toHTML(): String {
        return "<$tagName class=\"$className\">$content</$tagName>"
    }
}

fun tableRow(vararg cells: Tag): String {
    val row = Tag("tr", "")

    for (cell in cells)
        row.content += cell.toHTML()

    return row.toHTML()
}


fun main() {
    data class Message(
        val address: String?,
        val topic: String?,
        val sign: String?,
        val text: String?
    ) {
        fun toHTML(): String {
            val styles = Tag(
                "style",
                "table{border-collapse:collapse;}td{padding:10px}td:first-child{font-weight:bold;background:blueviolet;color:#fff}"
            ).toHTML()
            var content = ""

            address?.let {
                content += tableRow(
                    Tag("td", "Address"),
                    Tag("td", it)
                )
            }
            topic?.let {
                content += tableRow(
                    Tag("td", "Topic"),
                    Tag("td", it)
                )
            }
            text?.let {
                content += tableRow(
                    Tag("td", "Text"),
                    Tag("td", it)
                )
            }
            sign?.let {
                content += tableRow(
                    Tag("td", "Sign"),
                    Tag("td", it)
                )
            }

            return Tag("table", content).toHTML() + styles
        }
    }

    val emailContent = Message(
        "askbill@microsoft.com",
        "Example Topic",
        null,
        "Hello, John, I'm glad to see you"
    ).toHTML()

    File("email.html").writeText(emailContent)
}