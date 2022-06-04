branch=$(strip $(shell awk -F ':' '/branch/ {print $$2;}' tag.yml))
version=$(strip $(shell awk -F ':' '/version/ {print $$2;}' tag.yml))

ifneq ($(version),)
	tag=$(branch)-$(version)
else
	tag=$(branch)
endif

version:
	@echo "branch:" $(branch)
ifneq ($(version),)
	@echo "version:" $(version)
endif

tag:
	@git tag $(tag)
	@git push origin $(tag)

dropTag:
	@git tag -d $(tag)
	@git push origin :refs/tags/$(tag)

updateTag:dropTag tag

