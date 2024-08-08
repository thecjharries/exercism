-include ../global.mk

# Aliases for executables
GIT ?= git
PNPM ?= pnpm
PNPX ?= pnpx
RM ?= rm
SED ?= sed

# Exercism variables
SUBMISSIONS = $(EXERCISE).ts
TRACK = typescript

boot::
	cd $(EXERCISE) && $(PNPX) corepack yarn install

boot-tests::
	$(SED) -i 's/xit/it/g' $(EXERCISE)/*.test.ts
	$(GIT) add $(EXERCISE)/*.test.ts
	$(GIT) commit -m "Enable all tests" || exit 0

test::
	$(PNPX) corepack yarn test

clean::
	$(RM) -rf node_modules .yarn/
	$(RM) -rf .pnp.cjs .pnp.loader.mjs .yarn/install-state.gz yarn.lock
