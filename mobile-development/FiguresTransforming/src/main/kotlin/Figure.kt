abstract class Figure(val id: Int) : Movable, Transforming {
    abstract fun area(): Float
}