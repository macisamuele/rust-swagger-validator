__version__ = '0.0.0'

try:
    from rust_swagger_validator._rust_module import __build__  # noqa
except ImportError:  # Support for inplace build
    from _rust_module import __build__  # noqa
