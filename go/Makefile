-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
GO ?= go
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).go
TRACK = go

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"

test::
	$(GO) test -v -race

bench:
	$(GO) test -v --bench . --benchmem

# coverage::
# clean::
