class Point(val x: Int, val y: Int) {
    fun move(to: Point): Point {
        return Point(
            x = this.x + to.x,
            y = this.y + to.y
        )
    }

    fun rotate(direction: RotateDirection, rotationCenter: Point): Point {
        val dx = x - rotationCenter.x
        val dy = y - rotationCenter.y

        val rotationMatrix = when (direction) {
            RotateDirection.CounterClockwise -> arrayOf(intArrayOf(0, -1), intArrayOf(1, 0))
            RotateDirection.Clockwise -> arrayOf(intArrayOf(0, 1), intArrayOf(-1, 0))
        }

        return Point(
            x = rotationMatrix[0][0] * dx + rotationMatrix[0][1] * dy + rotationCenter.x,
            y = rotationMatrix[1][0] * dx + rotationMatrix[1][1] * dy + rotationCenter.y,
        )
    }

    override fun toString(): String {
        return "{$x, $y}"
    }
}