-include ../global.mk

# Aliases for executables
CD ?= cd
CMAKE ?= cmake
MKDIR ?= mkdir
RM ?= rm

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).f90
TRACK = fortran
CMAKE_GENERATOR ?= "Unix Makefiles"

boot-tests::
	$(MKDIR) -p $(EXERCISE)/build
	$(CD) $(EXERCISE)/build && $(CMAKE) -G $(CMAKE_GENERATOR) ..

test::
	$(CD) build && $(MAKE) && $(MAKE) test

# coverage::

clean::
	$(RM) -rf build testlib
