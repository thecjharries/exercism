-include ../global.mk

# Aliases for executables
PNPM ?= pnpm
RM ?= rm

# Exercism variables
SUBMISSIONS = $(EXERCISE).js
TRACK = javascript

boot::
	cd $(EXERCISE) && $(PNPM) install

clean::
	$(RM) -rf node_modules
	$(RM) -rf pnpm-lock.yaml
