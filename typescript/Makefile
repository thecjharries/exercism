-include ../global.mk

# Aliases for executables
GIT ?= git
PNPM ?= pnpm
RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE).ts
TRACK = typescript

boot::
	cd $(EXERCISE) && $(PNPM) install

boot-tests::
	$(SED) -i 's/xit/it/g' $(EXERCISE)/*.test.ts
	$(GIT) add $(EXERCISE)/*.test.ts
	$(GIT) commit -m "Enable all tests"

test::
	$(PNPM) test

clean::
	$(RM) -rf node_modules
	$(RM) -rf pnpm-lock.yaml
