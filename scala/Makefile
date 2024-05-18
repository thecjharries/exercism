-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
RM ?= rm
SBT ?= sbt
SED ?= sed

# Exercism variables
SUBMISSIONS = src/main/scala/$(EXERCISE_PASCAL_CASE).scala
TRACK = scala

boot-tests::
	$(SED) -Ei '/\s+pending\s*/d' $(EXERCISE)/src/test/scala/$(EXERCISE_PASCAL_CASE)Test.scala
	$(GIT) add $(EXERCISE)/
	$(GIT) commit -m "Enable all tests" || exit 0

test::
	$(SBT) test

# coverage::

clean::
	$(RM) -rf project target
