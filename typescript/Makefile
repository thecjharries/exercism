-include ../global.mk

# Aliases for executables
GIT ?= git
RM ?= rm
SED ?= sed
YARN ?= yarn

# Exercism variables
SUBMISSIONS = $(EXERCISE).ts
TRACK = typescript

boot::
	cd $(EXERCISE) && $(YARN) install

boot-tests::
	$(SED) -i 's/xit/it/g' $(EXERCISE)/*.test.ts
	$(GIT) add $(EXERCISE)/*.test.ts
	$(GIT) commit -m "Enable all tests" || exit 0

test::
	$(YARN) test

clean::
	$(RM) -rf node_modules .yarn/cache/ .yarn/unplugged/
	$(RM) -rf .pnp.cjs .pnp.loader.mjs .yarn/install-state.gz yarn.lock
