-include ../global.mk

# Aliases for executables
# CP ?= cp
CRYSTAL ?= crystal
# GH ?= gh
GIT ?= git
# RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = src/$(EXERCISE_SNAKE_CASE).cr
TRACK = crystal

boot-tests::
	$(SED) -i 's/pending "/it "/g' $(EXERCISE)/spec/$(EXERCISE_SNAKE_CASE)_spec.cr
	$(GIT) add $(EXERCISE)/spec/*.cr
	$(GIT) commit -m "Enable all tests"

test::
	$(CRYSTAL) spec

# coverage::
# clean::
