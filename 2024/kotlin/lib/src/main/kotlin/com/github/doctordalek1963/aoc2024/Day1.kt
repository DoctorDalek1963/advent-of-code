package com.github.doctordalek1963.aoc2024

import java.util.regex.Pattern
import kotlin.math.absoluteValue

class Day1(private val input: String) : AocDay() {
    private fun parseLists(): Pair<ArrayList<Int>, ArrayList<Int>> {
        val list1 = arrayListOf<Int>()
        val list2 = arrayListOf<Int>()

        for (line in input.lineSequence()) {
            val nums = line
                .split(Pattern.compile(" +"))
                .filter { it != "" }
                .map { it.toInt() }

            if (nums.isNotEmpty()) {
                list1.add(nums[0])
                list2.add(nums[1])
            }
        }

        return Pair(list1, list2)
    }

    override fun processPart1(): Int {
        val (list1, list2) = parseLists()
        list1.sort()
        list2.sort()

        return list1.zip(list2).sumOf { (a, b) -> (a - b).absoluteValue }
    }

    override fun processPart2(): Int {
        val (list1, list2) = parseLists()
        val occurrenceMap: HashMap<Int, Int> = hashMapOf()

        for (k in list2) {
            val v = occurrenceMap[k]
            if (v != null) {
                occurrenceMap[k] = v + 1
            } else {
                occurrenceMap[k] = 1
            }
        }

        return list1.sumOf { it * (occurrenceMap[it] ?: 0) }
    }

    companion object {
        const val TEST_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3"""

        val realInput = getInput(1)

        @JvmStatic
        fun main(args: Array<String>) {
            println("For test input:")
            Day1(TEST_INPUT).printParts()
            println("\nFor real input:")
            Day1(realInput).printParts()
        }
    }
}
