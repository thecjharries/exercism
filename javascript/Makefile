include ../global.mk

# Aliases for executables
PNPM ?= pnpm
RM ?= rm

boot::
	$(PNPM) install

clean::
	$(RM) -rf node_modules
	$(RM) -rf pnpm-lock.yaml
