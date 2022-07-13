#
# base.Makefile
# Copyright (C) 2022 veypi <i@veypi.com>
# 2022-07-13 23:40
# Distributed under terms of the Apache license.
#

branch=$(strip $(shell awk -F ':' '/branch/ {print $$2;}' cfg.yml))
v0=$(strip $(shell awk -F ':' '/tag0/ {print $$2;}' cfg.yml))
v1=$(strip $(shell awk -F ':' '/tag1/ {print $$2;}' cfg.yml))
v2=$(strip $(shell awk -F ':' '/tag2/ {print $$2;}' cfg.yml))

# _tag=$(branch)-$(v0)-$(v1)-$(v2)
_tag=$(branch)
ifneq ($(v0),)
	_tag:=$(_tag)-$(v0)
endif
ifneq ($(v1),)
	_tag:=$(_tag)-$(v1)
endif
ifneq ($(v2),)
	_tag:=$(_tag)-$(v2)
endif


version:
	@echo $(_tag)

tags:
ifeq ($(branch),master)
	@git tag -l | grep ''
else
	@git tag -l | grep $(branch)
endif

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
	@git push origin --tags

dropTag:
	@git tag -d $(_tag)
	@git push origin :refs/tags/$(_tag)

updateTag:dropTag tag
