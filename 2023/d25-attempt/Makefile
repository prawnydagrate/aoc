CC = gcc
# CFLAGS = -Wall -Wextra -I./include -g
CFLAGS = -Wall -I./include -g

PROJECT = aoc23d25
TEST_PROJECT = test-aoc23d25

SRCDIR = src
INCDIR = include
OBJDIR = build/obj
BINDIR = build/bin

MAIN_FNAME = main
TEST_FNAME = test

SRCFILES = $(filter-out $(SRCDIR)/$(TEST_FNAME).c, $(wildcard $(SRCDIR)/*.c))
TEST_SRCFILES = $(filter-out $(SRCDIR)/$(MAIN_FNAME).c, $(wildcard $(SRCDIR)/*.c))

OBJFILES = $(SRCFILES:$(SRCDIR)/%.c=$(OBJDIR)/%.o)
TEST_OBJFILES = $(TEST_SRCFILES:$(SRCDIR)/%.c=$(OBJDIR)/%.o)

EXEC = $(BINDIR)/$(PROJECT)
TEST_EXEC = $(BINDIR)/$(TEST_PROJECT)

$(EXEC): $(OBJFILES)
	@mkdir -p $(BINDIR)
	$(CC) $(OBJFILES) -o $(EXEC)

$(TEST_EXEC): $(TEST_OBJFILES)
	@mkdir -p $(BINDIR)
	$(CC) $(TEST_OBJFILES) -o $(TEST_EXEC)

$(OBJDIR)/%.o: $(SRCDIR)/%.c
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

build: $(EXEC)

test: $(TEST_EXEC)
	@./$(TEST_EXEC)

run: $(EXEC)
	@./$(EXEC)

clean:
	rm -rf $(OBJDIR) $(BINDIR)

.PHONY: build clean

all: $(EXEC)
