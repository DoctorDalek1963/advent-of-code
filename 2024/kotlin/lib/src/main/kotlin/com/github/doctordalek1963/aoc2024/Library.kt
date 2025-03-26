package com.github.doctordalek1963.aoc2024

import java.io.File

abstract class AocDay {
    abstract fun processPart1(): Int

    abstract fun processPart2(): Int

    fun printParts() {
        val p1 = processPart1()
        println("Part 1: $p1")
        val p2 = processPart2()
        println("Part 2: $p2")
    }
}

fun getInput(day: Int): String = File("inputs/day$day.txt").bufferedReader().use { it.readText() }
