-include ../global.mk

# Aliases for executables
CD ?= cd
CMAKE ?= cmake
MKDIR ?= mkdir
RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).h $(EXERCISE_SNAKE_CASE).cpp
TRACK = cpp
CMAKE_GENERATOR ?= "Unix Makefiles"

boot-tests::
	$(MKDIR) -p $(EXERCISE)/build
	$(CD) $(EXERCISE)/build && $(CMAKE) -G $(CMAKE_GENERATOR) ..
	$(SED) -i '1s/^/#define EXERCISM_RUN_ALL_TESTS\n/' $(EXERCISE)/$(EXERCISE_SNAKE_CASE)_test.cpp
	$(GIT) add $(EXERCISE)/$(EXERCISE_SNAKE_CASE)_test.cpp
	$(GIT) commit -m "Enable all tests"

clean::
	$(RM) -rf build

test::
	$(CD) build && $(MAKE)
