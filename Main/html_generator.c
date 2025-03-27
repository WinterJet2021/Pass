#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>

#define MAX_LENGTH 256

void generateHTML(const char* title, const char* category, const char* province, const char* district, const char* latitude, const char* longitude, const char* description, const char* imagePath) {
    char filePath[MAX_LENGTH] = "./attraction.html";  // Ensure it's created in the current directory
    FILE *file = fopen(filePath, "w");
    
    if (!file) {
        // Print detailed error message
        printf("Error: Could not create HTML file. Reason: %s\n", strerror(errno));
        return;
    }

    fprintf(file, "<html>\n<head>\n<title>%s</title>\n</head>\n<body>\n", title);
    fprintf(file, "<h1>%s</h1>\n", title);
    fprintf(file, "<p><strong>Category:</strong> %s</p>\n", category);
    fprintf(file, "<p><strong>Location:</strong> %s, %s</p>\n", province, district);
    fprintf(file, "<p><strong>Coordinates:</strong> %s, %s</p>\n", latitude, longitude);
    fprintf(file, "<p><strong>Description:</strong> %s</p>\n", description);
    if (strlen(imagePath) > 0) {
        fprintf(file, "<img src='%s' alt='%s' width='500'>\n", imagePath, title);
    }
    fprintf(file, "</body>\n</html>");
    
    fclose(file);
    
    // Confirm file creation
    printf("HTML file successfully created at: %s\n", filePath);
}