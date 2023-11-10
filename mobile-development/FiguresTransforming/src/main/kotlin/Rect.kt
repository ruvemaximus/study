// сочетание определения класса и конструктора одновременно объявляет переменные и задаёт их значения
class Rect(var x: Int, var y: Int, val width: Int, val height: Int) : Movable, Transforming, Figure(0) {
    var color: Int = -1 // при объявлении каждое поле нужно инициализировать

    lateinit var name: String
    // значение на момент определения неизвестно (только для объектных типов)
    // дополнительный конструктор вызывает основной
    constructor(rect: Rect) : this(rect.x, rect.y, rect.width, rect.height)

    override fun move(dx: Int, dy: Int) {
        x += dx; y += dy
    }

    override fun area(): Float {
        return (width*height).toFloat()
    }

    override fun resize(zoom: Int) {
        TODO("Not yet implemented")
    }

    override fun rotate(direction: RotateDirection, centerX: Int, centerY: Int) {
        TODO("Not yet implemented")
    }
}