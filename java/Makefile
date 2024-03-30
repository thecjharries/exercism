-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
GRADLE ?= gradle
# GIT ?= git
RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = src/main/java/*.java
TRACK = java

boot-tests::
	$(SED) -i '/@Disabled/d' $(EXERCISE)/src/test/java/*.java
	$(GIT) add $(EXERCISE)/
	$(GIT) commit -m "Enable all tests"

test::
	$(GRADLE) test

# coverage::

clean::
	$(RM) -rf build/ .gradle/
