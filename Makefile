branch=$(strip $(shell awk -F ':' '/branch/ {print $$2;}' cfg.yml))
v0=$(strip $(shell awk -F ':' '/tag0/ {print $$2;}' cfg.yml))
v1=$(strip $(shell awk -F ':' '/tag1/ {print $$2;}' cfg.yml))
v2=$(strip $(shell awk -F ':' '/tag2/ {print $$2;}' cfg.yml))

_tag=$(branch)-$(v0)-$(v1)-$(v2)


version:
	@echo $(_tag)

tags:
	@git tag -l | grep $(branch)

_tagname=$(strip $(t))
gotag:
ifeq ($(_tagname),)
	@echo please use: make gotag t=_tagname
else
	@echo checkout to tag $(_tagname)
	@git checkout -b $(_tagname) $(_tagname)
endif

tag:
	@git tag $(_tag)
	@git push origin $(_tag)

dropTag:
	@git tag -d $(_tag)
	@git push origin :refs/tags/$(_tag)

updateTag:dropTag tag


d=proc-macro-workshop
f=$(shell ls $(d)/$(v1)/tests | awk -F '-' '/$(v2)/ {print $0}')
macro/test:
	@echo running $(f)
	@cd $(d)/$(v1) && cargo test

macro/expand:
	@cd $(d) && cargo expand

