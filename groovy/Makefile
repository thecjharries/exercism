-include ../global.mk

# Aliases for executables
CHMOD ?= chmod
# CP ?= cp
# GH ?= gh
# GIT ?= git
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = src/main/groovy/$(EXERCISE_PASCAL_CASE).groovy
TRACK = groovy

boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
	$(CHMOD) +x $(EXERCISE)/gradlew

test::
	./gradlew test

# coverage::
# clean::
