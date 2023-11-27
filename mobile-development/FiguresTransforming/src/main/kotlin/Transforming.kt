interface Transforming {
    fun resize(zoom: Int)
    fun rotate(direction: RotateDirection, rotationCenter: Point)
}

enum class RotateDirection {
    Clockwise,
    CounterClockwise
}