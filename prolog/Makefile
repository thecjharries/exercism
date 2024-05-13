-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed
SWIPL ?= swipl

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).pl
TRACK = prolog

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
test::
	swipl -f $(EXERCISE_SNAKE_CASE).pl -s $(EXERCISE_SNAKE_CASE)_tests.plt -g run_tests,halt -t 'halt(1)' -- --all
# coverage::
# clean::
