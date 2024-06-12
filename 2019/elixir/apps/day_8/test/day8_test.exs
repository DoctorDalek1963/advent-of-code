defmodule Day8Test do
  use ExUnit.Case
  doctest Day8, import: true

  test "process part 1" do
    assert Day8.process_part1(Util.get_input()) === 1206
  end

  test "process part 2" do
    assert Day8.process_part2(Util.get_input()) === [
             ~c"1111000110111000110011100",
             ~c"1000000010100101001010010",
             ~c"1110000010100101000010010",
             ~c"1000000010111001011011100",
             ~c"1000010010101001001010000",
             ~c"1111001100100100111010000"
           ]

    # When viewed as an image, this spells "EJRGP":

    ####   ## ###   ##  ###
    #       # #  # #  # #  #
    ###     # #  # #    #  #
    #       # ###  # ## ###
    #    #  # # #  #  # #
    ####  ##  #  #  ### #
  end
end
