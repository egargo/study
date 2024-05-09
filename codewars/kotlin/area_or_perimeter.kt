// https://www.codewars.com/kata/5ab6538b379d20ad880000ab/train/kotlin

object Solution {
    fun areaOrPerimeter(l:Int, w:Int):Int {
        if (l == w) {
            return l * w;
        }
        return (2 * l) + (2 * w);
    }
}


fun main() {
    val sol = Solution.areaOrPerimeter(12, 12);
    println(sol);
}
