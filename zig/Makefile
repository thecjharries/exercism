-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed
ZIG ?= zig

# Exercism variables
SUBMISSIONS = $(EXERCISE).zig
TRACK = zig

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

test::
	$(ZIG) test test_$(EXERCISE).zig

# coverage::
# clean::
