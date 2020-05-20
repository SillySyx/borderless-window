#include <iostream>
#include <cstring>

auto listWindows() -> void
{
}

auto listCommands() -> void
{
	printf("available commands\n");
	printf("-l, --list       - lists visible windows\n");
	printf("-b, --borderless - make window fullscreen borderless\n");
}

auto shouldListWindows(int argc, char *argv[]) -> bool
{
	if (argc < 2)
		return false;

	return std::strcmp(argv[1], "--list") == 0 || std::strcmp(argv[1], "-l") == 0;
}

auto shouldMakeWindowBorderless(int argc, char *argv[]) -> bool
{
	if (argc < 3)
		return false;

	auto validCommand = std::strcmp(argv[1], "--borderless") == 0 || std::strcmp(argv[1], "-b") == 0;
	auto validWindowHandle = std::strlen(argv[1]) > 0;

	if (!validCommand || !validWindowHandle)
		return false;

	return true;
}

auto makeWindowBorderless(int argc, char *argv[]) -> void
{
}

auto parseCommand(int argc, char *argv[]) -> void
{
	if (shouldListWindows(argc, argv))
	{
		listWindows();
		return;
	}

	if (shouldMakeWindowBorderless(argc, argv))
	{
		makeWindowBorderless(argc, argv);
		return;
	}

	listCommands();
}

auto main(int argc, char *argv[]) -> int
{
	parseCommand(argc, argv);

	return 0;
}
