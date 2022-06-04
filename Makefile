branch=$(strip $(shell awk -F ':' '/branch/ {print $$2;}' cfg.yml))
version=$(strip $(shell awk -F ':' '/version/ {print $$2;}' cfg.yml))
sub=$(strip $(shell awk -F ':' '/sub/ {print $$2;}' cfg.yml))
d=proc-macro-workshop
f=$(shell ls $(d)/$(sub)/tests | grep 01 | awk -F '-' '/$(version)/ {print $0}')
tagname=$(strip $(t))


tag=$(branch)-$(sub)-$(version)
version:
	@echo "branch:    " $(branch)
	@echo "tag:       " $(tag)
	@echo "test file: " $(sub)/tests/$(f)

tags:
	@git tag -l | grep $(branch)

tag:
ifeq ($(tagname),)
	@echo please use: make tag t=TagName
else
	@echo checkout to tag $(tagname)
	@git checkout -b $(tagname) tagname
endif

addtag:
	@git tag $(tag)
	@git push origin $(tag)

dropTag:
	@git tag -d $(tag)
	@git push origin :refs/tags/$(tag)

updateTag:dropTag addtag

test:
	@cd $(d)/$(sub) && cargo test

expand:
	@cd $(d) && cargo expand

run:test expand
