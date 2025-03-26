package com.github.doctordalek1963.aoc2024

import com.github.doctordalek1963.aoc2024.Day1.Companion.TEST_INPUT
import com.github.doctordalek1963.aoc2024.Day1.Companion.realInput
import kotlin.test.Test
import kotlin.test.assertEquals

object Day1Test {
    @Test
    fun testPart1() {
        assertEquals(11, Day1(TEST_INPUT).processPart1())
        assertEquals(2_580_760, Day1(realInput).processPart1())
    }

    @Test
    fun testPart2() {
        assertEquals(31, Day1(TEST_INPUT).processPart2())
        assertEquals(25_358_365, Day1(realInput).processPart2())
    }
}
