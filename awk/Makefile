-include ../global.mk

# Aliases for executables
BATS ?= bats
# CP ?= cp
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE).awk
TRACK = awk

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

test::
	BATS_RUN_SKIPPED=true $(BATS) *.bats

# coverage::
# clean::
