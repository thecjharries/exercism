-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed
SPAGO ?= spago

# Exercism variables
SUBMISSIONS = src/$(EXERCISE_PASCAL_CASE).purs
TRACK = purescript

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
test::
	$(SPAGO) test
# coverage::
# clean::
