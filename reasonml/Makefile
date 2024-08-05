-include ../global.mk

# Aliases for executables
GIT ?= git
PNPM ?= pnpm
RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = src/$(EXERCISE_PASCAL_CASE).re
TRACK = reasonml

# boot-tests::
# 	$(SED) -i 's/xtest/test/g' $(EXERCISE)/*.spec.js
# 	$(GIT) add $(EXERCISE)/*.spec.js
# 	$(GIT) commit -m "Enable all tests"

boot::
	cd $(EXERCISE) && $(PNPM) install

test::
	$(PNPM) test

clean::
	$(RM) -rf node_modules
	$(RM) -rf pnpm-lock.yaml
