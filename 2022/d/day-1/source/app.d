import std.algorithm;
import std.array;
import std.conv;
import std.stdio;
import std.string;

Appender!(Appender!(int[])[]) parseGroups(File input)
{
	auto currentGroup = appender!(int[])();
	auto allGroups = appender!(Appender!(int[])[])();

	string line;
	while ((line = strip(input.readln)) !is null)
	{
		if (line != "")
		{
			int num = to!int(line);
			currentGroup.put(num);
		}
		else
		{
			allGroups.put(currentGroup);
			currentGroup = appender!(int[])();
		}
	}

	allGroups.put(currentGroup);
	currentGroup = appender!(int[])();

	return allGroups;
}

int part1(File input)
{
	auto allGroups = parseGroups(input);

	int maxCalories = int.min;
	foreach (group; allGroups)
	{
		int caloriesInGroup = 0;
		foreach (elem; group)
			caloriesInGroup += elem;

		maxCalories = max(maxCalories, caloriesInGroup);
	}

	return maxCalories;
}

int part2(File input)
{
	auto groupSums = parseGroups(input)
		.opSlice
		.map!(g => g.opSlice.sum)
		.array
		.sort!("a > b");

	return groupSums[0] + groupSums[1] + groupSums[2];
}

File get_test_input()
{
	return File("test_input.txt", "r");
}

File get_input()
{
	return File("input.txt", "r");
}

void main()
{
	writefln("Part 1: %s", part1(get_input));
	writefln("Part 2: %s", part2(get_input));
}

// Part 1
unittest
{
	assert(part1(get_test_input) == 24_000);
	assert(part1(get_input) == 69_528);
}

// Part 2
unittest
{
	assert(part2(get_test_input) == 45_000);
	assert(part2(get_input) == 206_152);
}
