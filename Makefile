branch=$(strip $(shell awk -F ':' '/branch/ {print $$2;}' cfg.yml))
tag0=$(strip $(shell awk -F ':' '/tag0/ {print $$2;}' cfg.yml))
tag1=$(strip $(shell awk -F ':' '/tag1/ {print $$2;}' cfg.yml))
tag2=$(strip $(shell awk -F ':' '/tag2/ {print $$2;}' cfg.yml))

tag=$(branch)-$(tag0)-$(tag1)-$(tag2)


version:
	@echo $(tag)

tags:
	@git tag -l | grep $(branch)

tagname=$(strip $(t))
gotag:
ifeq ($(tagname),)
	@echo please use: make gotag t=TagName
else
	@echo checkout to tag $(tagname)
	@git checkout -b $(tagname) $(tagname)
endif

tag:
	@git tag $(tag)
	@git push origin $(tag)

dropTag:
	@git tag -d $(tag)
	@git push origin :refs/tags/$(tag)

updateTag:dropTag addtag


d=proc-macro-workshop
f=$(shell ls $(d)/$(sub)/tests | grep 01 | awk -F '-' '/$(version)/ {print $0}')
macro/test:
	@cd $(d)/$(sub) && cargo test

macro/expand:
	@cd $(d) && cargo expand

