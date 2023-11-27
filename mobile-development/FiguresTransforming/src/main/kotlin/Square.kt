class Square(private var topLeftCord: Point, private var sideSize: Int) : Figure(0) {
    override fun area(): Float {
        return (sideSize * sideSize).toFloat()
    }

    override fun move(to: Point) {
        topLeftCord = topLeftCord.move(to)
    }

    override fun rotate(direction: RotateDirection, rotationCenter: Point) {
        topLeftCord = when (direction) {
            RotateDirection.Clockwise -> Point(x = topLeftCord.x, y = topLeftCord.y - sideSize)
            RotateDirection.CounterClockwise -> Point(x = topLeftCord.x + sideSize, y = topLeftCord.y)
        }
        topLeftCord = topLeftCord.rotate(direction, rotationCenter)
    }

    override fun resize(zoom: Int) {
        sideSize *= zoom
    }

    override fun toString(): String {
        return "Rect<top-left-point: $topLeftCord, side-size: $sideSize>"
    }
}