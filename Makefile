PYTHON_MODULE_ROOT := python/rust_swagger_validator

define check_env_variable
$(if $(strip $($1)),,$(error "$1" ENV VARIABLE IS REQUIRED!))
endef

ifndef EDITOR
	ifneq ("$(wildcard /etc/alternatives/editor)","")
		EDITOR := /etc/alternatives/editor
	else
		EDITOR := vi
	endif
endif

venv: requirements-dev.txt setup.py ${PYTHON_MODULE_ROOT}/__init__.py
	-deactivate
	rm -rf venv/  # Ensure that venv does not exist
	@false

.PHONY: development
development: venv
	./venv/bin/pre-commit install --install-hooks

.PHONY: test
test:
	@false

.PHONY: clean
clean:
	rm -rf .tox/ .pytest_cache/ .coverage venv/
	find -name *.pyc -delete

.PHONY: release
release: clean venv
	$(eval $(call check_env_variable,NEXT_VERSION))
ifneq ($(shell git rev-parse --abbrev-ref HEAD),master)
	$(error `make release` could be execute only on master branch)
endif
	echo "Running tests for extra safety"
	TOX_ARGS=--recreate $(MAKE) test
	echo "Clean old artifacts"
	rm -rf build/ dist/
	sed -ri "s/^(__version__ = )'.*'$/\1'${NEXT_VERSION}'/" ${PYTHON_MODULE_ROOT}/__init__.py
	sed -ri "s/^(=+)$$/\1\n\n${NEXT_VERSION} ($$(date "+%Y-%m-%d"))\n------------------\n- TODO: add notes/" CHANGELOG.md
	${EDITOR} CHANGELOG.md
	git add --patch language_formatters_pre_commit_hooks/__init__.py CHANGELOG.rst
	git commit -m "Release version ${NEXT_VERSION}"
	git tag "v${NEXT_VERSION}"
	venv/bin/python setup.py sdist bdist_wheel
	git push origin master --tags
