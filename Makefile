branch=$(strip $(shell awk -F ':' '/branch/ {print $$2;}' tag.yml))
version=$(strip $(shell awk -F ':' '/version/ {print $$2;}' tag.yml))
sub=$(strip $(shell awk -F ':' '/sub/ {print $$2;}' tag.yml))
d=proc-macro-workshop
f=$(shell ls $(d)/$(sub)/tests | grep 01 | awk -F '-' '/$(version)/ {print $0}')


tag=$(branch)-$(sub)-$(version)
version:
	@echo "branch:    " $(branch)
	@echo "tag:       " $(tag)
	@echo "test file: " $(sub)/tests/$(f)

tag:
	@git tag $(tag)
	@git push origin $(tag)

dropTag:
	@git tag -d $(tag)
	@git push origin :refs/tags/$(tag)

updateTag:dropTag tag

test:
	@cd $(d)/$(sub) && cargo test

expand:
	@cd $(d) && cargo expand

run:test expand
