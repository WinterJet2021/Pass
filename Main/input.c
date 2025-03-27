#include <ncurses.h>
#include <stdio.h>
#include <string.h>

#define MAX_LENGTH 256

void getUserInput(char* prompt, char* buffer) {
    printw("%s", prompt);
    refresh();
    echo();  // Enable echoing for typing
    getnstr(buffer, MAX_LENGTH - 1);
    noecho(); // Disable echoing again
}