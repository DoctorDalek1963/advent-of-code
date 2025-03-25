package com.github.doctordalek1963.aocYEARNUM

class DayDAYNUM(private val input: String) : AocDay() {
    override fun processPart1(): Int {
        return 0
    }

    override fun processPart2(): Int {
        return 0
    }

    companion object {
        const val TEST_INPUT = """"""

        val realInput = getInput(DAYNUM)

        @JvmStatic
        fun main(args: Array<String>) {
            println("For test input:")
            DayDAYNUM(TEST_INPUT).printParts()
            println("\nFor real input:")
            DayDAYNUM(realInput).printParts()
        }
    }
}
