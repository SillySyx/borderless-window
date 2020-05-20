# window-management

## Building

Building with microsoft c++ compiler, needs visual studio development tools/command prompt
```
cl src/windows.cpp user32.lib
```

Building with gcc
```
g++ -Wall -std=c++14 -o dist/win-api src/linux.cpp
```