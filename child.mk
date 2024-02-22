-include ../global.mk

# Aliases for executables
# CODE ?= code
# CP ?= cp
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE)
TRACK = @@TRACK@@

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
# boot-vscode::
# 	$(CODE) --reuse-window $(SUBMISSIONS)
# test::
# coverage::
# clean::
