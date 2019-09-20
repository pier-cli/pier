.PHONY: install

install:
	nix-env -if derivation.nix
