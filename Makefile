PYTHON_MODULE_ROOT := python/rust_swagger_validator

define check_env_variable
$(if $(strip $($1)),,$(error "$1" ENV VARIABLE IS REQUIRED!))
endef

ifndef PRE_COMMIT_BIN
    PRE_COMMIT_BIN := ./venv/bin/pre-commit
endif

ifndef EDITOR
	ifneq ("$(wildcard /etc/alternatives/editor)","")
		EDITOR := /etc/alternatives/editor
	else
		EDITOR := vi
	endif
endif

venv: requirements-dev.txt setup.cfg setup.py ${PYTHON_MODULE_ROOT}/__init__.py
	-deactivate
	rm -rf venv/  # Ensure that venv does not exist
	tox -e venv

${PRE_COMMIT_BIN}: venv
	@true

.PHONY: development
development: venv ${PRE_COMMIT_BIN}
	${PRE_COMMIT_BIN} install --install-hooks

.PHONY: test
test:
	tox

.PHONY: clean
clean:
	rm -rf .tox/ .pytest_cache/ .coverage venv/
	find -name *.pyc -delete

.PHONY: lint
lint: ${PRE_COMMIT_BIN}
	${PRE_COMMIT_BIN} run --all-files
	touch rust/src/lib.rs   # touch a file of the rust project to "force" cargo to recompile it so clippy will actually run
	cargo +nightly clippy --lib --all-features -- -D clippy::pedantic -D clippy::nursery

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
