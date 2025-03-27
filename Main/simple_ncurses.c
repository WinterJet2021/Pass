#include <ncurses.h>
#include <stdio.h>

int main() {
    char name[100];

    initscr();               // Start ncurses mode
    raw();                   // Capture input one character at a time
    keypad(stdscr, TRUE);    // Enable keyboard mapping
    noecho();                // Do not echo characters typed by the user

    printw("Enter your name: ");
    refresh();               // Refresh screen to print prompt

    echo();                  // Enable echoing for input
    getnstr(name, 99);        // Read user input with size limit
    noecho();                 // Disable echoing again

    clear();
    printw("Hello, %s!\n", name);
    printw("Press any key to exit...");
    refresh();

    getch();                 // Wait for a key press
    endwin();                // End ncurses mode
    return 0;
}