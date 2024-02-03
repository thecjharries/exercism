-include ../global.mk

# Aliases for executables
NIM ?= nim
RM ?= rm

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).nim
TRACK = nim

test::
	$(NIM) r test_$(EXERCISE_SNAKE_CASE).nim
