-include ../global.mk

# Aliases for executables
CD ?= cd
CMAKE ?= cmake
MKDIR ?= mkdir
RM ?= rm

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).h $(EXERCISE_SNAKE_CASE).cpp
TRACK = cpp
CMAKE_GENERATOR ?= "Unix Makefiles"

boot-tests::
	$(MKDIR) -p $(EXERCISE)/build
	$(CD) $(EXERCISE)/build && $(CMAKE) -G $(CMAKE_GENERATOR) ..

clean::
	$(RM) -rf build

test::
	$(CD) build && $(MAKE)
