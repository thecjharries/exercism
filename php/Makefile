-include ../global.mk

# Aliases for executables
# CP ?= cp
# GH ?= gh
# GIT ?= git
PHPUNIT ?= $(XDG_CONFIG_HOME)/composer/vendor/bin/phpunit
# RM ?= rm
# SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_PASCAL_CASE).php
TRACK = php

# boot-tests::
# 	$(GIT) add $(EXERCISE)/
# 	$(GIT) commit -m "Enable all tests"
test::
	$(PHPUNIT) $(EXERCISE_PASCAL_CASE)Test.php
# coverage::
# clean::
