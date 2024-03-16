-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
GRADLE ?= gradle
# GIT ?= git
RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = src/main/groovy/$(EXERCISE_PASCAL_CASE).groovy
TRACK = groovy

boot-tests::
	$(SED) -i '/@Ignore/d' $(EXERCISE)/src/test/groovy/$(EXERCISE_PASCAL_CASE)Spec.groovy
	$(GIT) add $(EXERCISE)/
	$(GIT) commit -m "Enable all tests"

test::
	$(GRADLE) test

# coverage::

clean::
	$(RM) -rf build/ .gradle/
