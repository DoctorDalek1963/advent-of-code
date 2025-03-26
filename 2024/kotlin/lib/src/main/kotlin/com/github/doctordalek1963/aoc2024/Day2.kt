package com.github.doctordalek1963.aoc2024

import kotlin.math.absoluteValue

class Day2(
    private val input: String,
) : AocDay() {
    override fun processPart1(): Int =
        input
            .lineSequence()
            .map { Report(it.split(" ").filter { it != "" }.map { it.toInt() }).isSafe() }
            .filter { it }
            .count()

    override fun processPart2(): Int =
        input
            .lineSequence()
            .map {
                Report(it.split(" ").filter { it != "" }.map { it.toInt() })
                    .isSafeWithProblemDampener()
            }.filter { it }
            .count()

    companion object {
        const val TEST_INPUT =
            """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""

        val realInput = getInput(2)

        @JvmStatic
        fun main(args: Array<String>) {
            println("For test input:")
            Day2(TEST_INPUT).printParts()
            println("\nFor real input:")
            Day2(realInput).printParts()
        }
    }
}

private enum class Direction {
    INCREASING,
    DECREASING,
    ;

    companion object {
        fun from(
            a: Int,
            b: Int,
        ): Direction? =
            when {
                a < b -> INCREASING
                a > b -> DECREASING
                else -> null
            }
    }
}

class Report(
    private val levels: List<Int>,
) {
    fun isSafe(): Boolean {
        if (levels.size < 2) return false

        var direction: Direction? = null
        var prev: Int? = null

        for (i in levels) {
            if (prev == null) {
                prev = i
                continue
            } else {
                // If we're just starting, set the desired direction
                if (direction == null) {
                    val newDir = Direction.from(prev, i)

                    // If we're not strictly monotonic, we're not safe
                    if (newDir == null) {
                        return false
                    } else {
                        direction = newDir
                    }
                } else {
                    // We're not just starting, so check direction
                    if (direction != Direction.from(prev, i)) return false
                }

                // We need small diffs
                if ((i - prev).absoluteValue !in 1..3) return false

                prev = i
            }
        }

        return true
    }

    fun isSafeWithProblemDampener(): Boolean {
        for (i in levels.indices) {
            val newLevels = ArrayList(levels)
            newLevels.removeAt(i)

            if (Report(newLevels).isSafe()) return true
        }

        return false
    }
}
