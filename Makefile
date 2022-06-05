branch=$(strip $(shell awk -F ':' '/branch/ {print $$2;}' cfg.yml))
v0=$(strip $(shell awk -F ':' '/tag0/ {print $$2;}' cfg.yml))
v1=$(strip $(shell awk -F ':' '/tag1/ {print $$2;}' cfg.yml))
v2=$(strip $(shell awk -F ':' '/tag2/ {print $$2;}' cfg.yml))

tag=$(branch)-$(v0)-$(v1)-$(v2)


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

updateTag:dropTag tag


d=proc-macro-workshop
f=$(shell ls $(d)/$(v1)/tests | awk -F '-' '/$(v2)/ {print $0}')
macro/test:
	@echo running $(f)
	@cd $(d)/$(v1) && cargo test

macro/expand:
	@cd $(d) && cargo expand

