class Circle(private var center: Point, private var radius: Int) : Figure(0) {
    override fun resize(zoom: Int) {
        radius *= zoom
    }

    override fun rotate(direction: RotateDirection, rotationCenter: Point) {
        center = center.rotate(direction, rotationCenter)
    }

    override fun area(): Float {
        return (Math.PI * radius * radius).toFloat()
    }

    override fun move(to: Point) {
        center = center.move(to)
    }

    override fun toString(): String {
        return "Circle<at: $center, radius: $radius>"
    }
}