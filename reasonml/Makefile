-include ../global.mk

# Aliases for executables
GIT ?= git
PNPM ?= pnpm
PNPX ?= pnpx
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
	$(PNPX) jest --watchAll --env=node

clean::
	$(RM) -rf node_modules pnpm-lock.yaml .bsb.lock .merlin __tests__/$(EXERCISE_PASCAL_CASE)_test.bs.js lib src/$(EXERCISE_PASCAL_CASE).bs.js
