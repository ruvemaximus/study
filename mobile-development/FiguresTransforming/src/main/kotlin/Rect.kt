class Rect(
    private var topLeftCord: Point, private var width: Int, private var height: Int
) : Figure(0) {
    override fun move(to: Point) {
        topLeftCord = topLeftCord.move(to)
    }

    override fun area(): Float {
        return (width * height).toFloat()
    }

    override fun resize(zoom: Int) {
        width *= zoom
        height *= zoom
    }

    override fun rotate(direction: RotateDirection, rotationCenter: Point) {
        topLeftCord = when (direction) {
            RotateDirection.Clockwise -> Point(x = topLeftCord.x, y = topLeftCord.y - height)
            RotateDirection.CounterClockwise -> Point(x = topLeftCord.x + width, y = topLeftCord.y)
        }
        topLeftCord = topLeftCord.rotate(direction, rotationCenter)
        width = height.also { height = width }
    }

    override fun toString(): String {
        return "Rect<top-left-point: $topLeftCord, width: $width, height: $height>"
    }
}