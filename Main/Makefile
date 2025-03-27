# Makefile for Console-Based Tourism Information Entry

CC = gcc
CFLAGS = -Wall
LDFLAGS = -lncurses

all: tourism_entry

tourism_entry: main.o input.o html_generator.o display.o
	$(CC) $(CFLAGS) -o tourism_entry main.o input.o html_generator.o display.o $(LDFLAGS)

main.o: main.c
	$(CC) $(CFLAGS) -c main.c

input.o: input.c
	$(CC) $(CFLAGS) -c input.c

html_generator.o: html_generator.c
	$(CC) $(CFLAGS) -c html_generator.c

display.o: display.c
	$(CC) $(CFLAGS) -c display.c

clean:
	rm -f *.o tourism_entry attraction.html