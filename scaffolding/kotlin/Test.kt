package com.github.doctordalek1963.aocYEARNUM

import com.github.doctordalek1963.aocYEARNUM.DayDAYNUM.Companion.TEST_INPUT
import com.github.doctordalek1963.aocYEARNUM.DayDAYNUM.Companion.realInput
import kotlin.test.Test
import kotlin.test.assertEquals

object DayDAYNUMTest {
    @Test
    fun testPart1() {
        assertEquals(-1, DayDAYNUM(TEST_INPUT).processPart1())
        assertEquals(-1, DayDAYNUM(realInput).processPart1())
    }

    @Test
    fun testPart2() {
        assertEquals(-1, DayDAYNUM(TEST_INPUT).processPart2())
        assertEquals(-1, DayDAYNUM(realInput).processPart2())
    }
}
