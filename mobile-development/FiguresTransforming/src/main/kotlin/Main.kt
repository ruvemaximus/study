fun main() {
    val figures: Array<Figure> = arrayOf(
        Rect(Point(2, 4), 4, 2),
        Square(Point(0, 0), 2),
        Circle(Point(5, 5), 3)
    )
    val (rectangle, square, circle) = figures

    println("--- RECTANGLE ---")
    println("Init: $rectangle")
    rectangle.rotate(RotateDirection.CounterClockwise, Point(3, -3))
    println("Counter Clockwise Rotation: $rectangle")
    rectangle.rotate(RotateDirection.Clockwise, Point(3, -3))
    println("Clockwise Rotation: $rectangle")
    println()

    println("--- SQUARE ---")
    println("Init: $square")
    square.move(Point(2, 3))
    println("Moved to ${Point(2, 3)}: $square")
    square.rotate(RotateDirection.Clockwise, Point(0, 0))
    println("Rotated with center in ${Point(0, 0)}: $square")
    println()

    println("--- CIRCLE ---")
    println("Init: $circle")
    circle.move(Point(8, -1))
    println("Moved to ${Point(8, -1)}: $circle")
    println("Area: ${circle.area()}")
    println()
}