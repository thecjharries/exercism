-include ../global.mk

# Aliases for executables
# CP ?= cp
DOTNET ?= dotnet
# GH ?= gh
# GIT ?= git
RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_PASCAL_CASE).vb
TRACK = vbnet

boot-tests::
	$(SED) -Ei 's/\(Skip[^)]+\)//g' $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.vb
	$(GIT) add $(EXERCISE)/$(EXERCISE_PASCAL_CASE)Tests.vb
	$(GIT) commit -m "Enable all tests" || exit 0

test::
	$(DOTNET) test

# coverage::

clean::
	$(RM) -rf bin/ obj/
