-include ../global.mk

# Aliases for executables
CHEZ ?= chez
# CP ?= cp
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE).scm
TRACK = scheme

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

test::
	$(CHEZ) --script test.scm

# coverage::
# clean::
