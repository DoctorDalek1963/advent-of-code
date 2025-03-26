package com.github.doctordalek1963.aoc2024

import com.github.doctordalek1963.aoc2024.Day2.Companion.TEST_INPUT
import com.github.doctordalek1963.aoc2024.Day2.Companion.realInput
import kotlin.test.Test
import kotlin.test.assertEquals

object Day2Test {
    @Test
    fun testPart1() {
        assertEquals(2, Day2(TEST_INPUT).processPart1())
        assertEquals(526, Day2(realInput).processPart1())
    }

    @Test
    fun testPart2() {
        assertEquals(4, Day2(TEST_INPUT).processPart2())
        assertEquals(566, Day2(realInput).processPart2())
    }
}
