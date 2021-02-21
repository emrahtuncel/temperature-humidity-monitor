# specify the shell, in case the SHELL variable is not set or is not
# inherited from the environment
SHELL = /bin/bash

# set suffix list, to prevent confusion between different make programs
# line 17 clears an implied suffix list, and line 18 sets a new one
.SUFFIXES:
.SUFFIXES: .c .h .o

# project set up and directories
CC	= armv6-rpi-linux-gnueabihf-gcc
INCLDIR	= source/
BINDIR	= bin/
OBJDIR	= obj/
SRCDIR	= source/
AR = armv6-rpi-linux-gnueabihf-ar


# final executable name
_BIN	= libdht.a
BIN	= $(addprefix $(BINDIR), $(_BIN))

# files; here all object files will be stored in $(OBJDIR), with the
# same base names as corresponding c files from SRCDIR
SRC	= $(wildcard $(SRCDIR)*.c)
_OBJS	= $(patsubst $(SRCDIR)%.c, %.o, $(SRC))
OBJS	= $(addprefix $(OBJDIR), $(_OBJS))

# compilation flags
CFLAGS = -Wall -g -I$(INCLDIR)
OFLAGS = rcs


# compile binary and object files
.PHONY: all
all: dir $(BIN) test

dir:
	@mkdir -p $(OBJDIR)
	@mkdir -p $(BINDIR)

$(BIN): $(OBJS)
	$(AR) $(OFLAGS) $(BIN) $(OBJS) 

$(OBJDIR)%.o: $(SRCDIR)%.c
	$(CC) $(CFLAGS) -c $< -o $@

# clean entire project directory
.PHONY: clean
clean:
	- rm -rf $(BINDIR) $(OBJDIR) main.o sensor

.PHONY: test
test:
	$(CC) $(CFLAGS) -c main.c -o main.o
	$(CC)  -o sensor main.o bin/libdht.a