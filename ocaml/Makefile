-include ../global.mk

# Aliases for executables
# CP ?= cp
DUNE ?= dune
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).ml
TRACK = ocaml

boot-feature-branch::
	rm -rf $(EXERCISE)/Makefile
# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
test::
	$(DUNE) test
# coverage::
clean::
	$(DUNE) clean
