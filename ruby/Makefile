-include ../global.mk

# Aliases for executables
GEM ?= gem
GIT ?= git
RM ?= rm
RUBY ?= ruby
SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE_SNAKE_CASE).rb
TRACK = ruby

boot-tests::
	$(GEM) install minitest
	$(SED) -i -e '/skip$/d' $(EXERCISE_SNAKE_CASE)_test.rb
	$(GIT) add $(EXERCISE_SNAKE_CASE)_test.rb
	$(GIT) commit -m "Enable all tests"

test::
	$(RUBY) -r minitest/pride $(EXERCISE_SNAKE_CASE)_test.rb
