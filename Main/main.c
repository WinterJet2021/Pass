#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ncurses.h>

#define MAX_LENGTH 256

void getUserInput(char* prompt, char* buffer);
void generateHTML(const char* title, const char* category, const char* province, const char* district, const char* latitude, const char* longitude, const char* description, const char* imagePath);
void displayHTML();

int main() {
    initscr();               // Initialize ncurses mode
    raw();                   // Capture input one character at a time
    keypad(stdscr, TRUE);    // Enable keyboard mapping
    noecho();                // Disable echoing by default

    int choice;
    char title[MAX_LENGTH], category[MAX_LENGTH], province[MAX_LENGTH], district[MAX_LENGTH];
    char latitude[MAX_LENGTH], longitude[MAX_LENGTH], description[MAX_LENGTH * 4], imagePath[MAX_LENGTH];
    char choiceStr[5];  // Buffer to store choice as a string

    while (1) {
        clear();
        printw("1. Create New Attraction Page\n");
        printw("2. Display Attraction Page\n");
        printw("3. Exit\n");
        printw("Choose an option: ");
        refresh();

        echo();  // Enable echo for reading choice
        getnstr(choiceStr, 4);
        noecho(); // Turn echo off after reading input

        choice = atoi(choiceStr);  // Convert input to integer

        switch (choice) {
            case 1:
                clear();
                getUserInput("Enter Attraction Title: ", title);
                getUserInput("Enter Category: ", category);
                getUserInput("Enter Province: ", province);
                getUserInput("Enter District: ", district);
                getUserInput("Enter Latitude: ", latitude);
                getUserInput("Enter Longitude: ", longitude);
                getUserInput("Enter Description: ", description);
                getUserInput("Enter Image Path (URL or local file): ", imagePath);
                generateHTML(title, category, province, district, latitude, longitude, description, imagePath);
                break;
            case 2:
                displayHTML();
                break;
            case 3:
                endwin();  // End ncurses mode
                return 0;
            default:
                printw("Invalid option. Try again.\n");
                refresh();
                break;
        }
        printw("Press any key to continue...");
        refresh();
        getch();  // Wait for user input before continuing
    }

    endwin();  // End ncurses mode
    return 0;
}