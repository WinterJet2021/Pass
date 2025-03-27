#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <unistd.h>
#include <ncurses.h>

#define MAX_FILES 100
#define MAX_FILENAME 256

void displayHTML() {
    struct dirent *entry;
    DIR *d = opendir(".");  // Open the current directory
    if (d == NULL) {
        printw("Error: Could not open directory.\n");
        refresh();
        return;
    }

    char filenames[MAX_FILES][MAX_FILENAME];
    int file_count = 0;

    // Read directory contents and list HTML files
    printw("Available HTML Files:\n");
    while ((entry = readdir(d)) != NULL) {
        if (strstr(entry->d_name, ".html") != NULL) {
            snprintf(filenames[file_count], MAX_FILENAME, "%s", entry->d_name);
            printw("%d. %s\n", file_count + 1, filenames[file_count]);
            file_count++;
            if (file_count >= MAX_FILES) break;
        }
    }
    closedir(d);

    if (file_count == 0) {
        printw("No HTML files found in the current directory.\n");
        refresh();
        return;
    }

    // Ask user to select a file
    char choiceStr[5];
    int choice;
    printw("Enter the number of the file you want to open: ");
    refresh();
    echo();  // Enable input display
    getnstr(choiceStr, 4);  // Use getnstr() to capture input
    noecho();  // Disable input display

    choice = atoi(choiceStr);  // Convert input to integer

    if (choice < 1 || choice > file_count) {
        printw("Invalid choice. Returning to menu.\n");
        refresh();
        return;
    }

    // Open the selected file
    char command[MAX_FILENAME + 20];
    snprintf(command, sizeof(command), "explorer.exe %s", filenames[choice - 1]);

    // Try opening the file in Windows Browser (WSL)
    if (system(command) == 0) return;

    // Try Linux command
    snprintf(command, sizeof(command), "xdg-open %s", filenames[choice - 1]);
    if (system(command) == 0) return;

    // Try MacOS command
    snprintf(command, sizeof(command), "open %s", filenames[choice - 1]);
    if (system(command) == 0) return;

    // If all attempts fail
    printw("Error: Could not open HTML file. Please try opening it manually.\n");
    refresh();
}