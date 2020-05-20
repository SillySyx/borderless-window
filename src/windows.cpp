#include <iostream>
#include <string>
#include <windows.h>

BOOL CALLBACK listWindowCallback(HWND wnd, LPARAM lParam)
{
	if (!IsWindowVisible(wnd))
		return 1;

	int length = GetWindowTextLength(wnd);
	if (!length)
		return 1;

	length++;

	TCHAR *buffer = (TCHAR*)malloc(length);
	GetWindowText(wnd, buffer, length);

	std::string format("%i \t - %s\n");

	if ((unsigned long)wnd < 1000000ul)
		format = "%i \t\t - %s\n";

	printf(format.c_str(), wnd, buffer);

	free(buffer);

	return 1;
}

auto listWindows() -> void
{
	printf("visible windows\n");
	EnumWindows(listWindowCallback, 0);
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
	auto windowHandle = (HWND)std::atoi(argv[2]);

	printf("making window %s borderless\n", argv[2]);

	auto hwnd = (HWND)IntToPtr(windowHandle);

	SetWindowLongPtr(hwnd, GWL_STYLE, WS_POPUP);

	auto x = 0;
	auto y = 0;
	auto width = GetSystemMetrics(SM_CXSCREEN);
	auto height = GetSystemMetrics(SM_CYSCREEN);
	auto flags = SWP_SHOWWINDOW | SWP_FRAMECHANGED;

	SetWindowPos(hwnd, HWND_TOP, x, y, width, height, flags);
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
