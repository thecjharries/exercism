-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed
TCLSH ?= tclsh

# Exercism variables
SUBMISSIONS = $(EXERCISE).tcl
TRACK = tcl

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

test::
	RUN_ALL=1 $(TCLSH) $(EXERCISE).test
# coverage::
# clean::
