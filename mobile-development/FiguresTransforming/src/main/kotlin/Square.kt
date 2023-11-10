// TODO: дополнить определение класса размерами и позицией
class Square(var x: Int, var y: Int, val sideSize: Int) : Transforming, Figure(0) {
    override fun area(): Float {
        return (sideSize * sideSize).toFloat()
    }

    // Interface Transforming
    override fun rotate(direction: RotateDirection, centerX: Int, centerY: Int) {
        TODO("Not yet implemented")
    }

    override fun resize(zoom: Int) {
        TODO("Not yet implemented")
    }
}