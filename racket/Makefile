-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
RACO ?= raco
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE).rkt
TRACK = racket

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
test::
	$(RACO) test $(EXERCISE)-test.rkt
# coverage::
# clean::
