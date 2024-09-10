# build-in parameters
DEFAULT_GOAL = app
CC = clang

# paths
WORK_DIR  = $(shell pwd)
BUILD_DIR = $(WORK_DIR)/build
DEBUG_DIR = $(WORK_DIR)/debug
INCLUDE_DIR = $(WORK_DIR)/include
NAME ?= cond_control

# files
BINARY   = $(BUILD_DIR)/$(NAME)

# linker flags
LDFLAGS  = -L./lib -ldotstruct

# build flags
CFLAGS   = -Og -std=c23 -I$(INCLUDE_DIR)

.PHONY: all directory clean

all: directory $(BINARY)

# make the binary
$(BINARY): $(WORK_DIR)/src/$(NAME).c
	@$(CC) $(CFLAGS) -o $@.s -S $<
	@$(CC) $(CFLAGS) -o $@.o -c $<
	@$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $< $(INCLUDE_DIR)/*.c
	@objdump -d $@.o > $@.txt

# run the binary
run: all
	@$(BINARY)

# strace the binary
strace: all
	@strace -o ./doc/$(NAME).txt $(BINARY)

# create directory
directory:
	@mkdir -p $(BUILD_DIR) $(DEBUG_DIR)

# clean build files and directory
clean:
	rm -rf $(BUILD_DIR) $(DEBUG_DIR)
