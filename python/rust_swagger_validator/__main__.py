try:
    from rust_swagger_validator._rust_module import convert_string
    from rust_swagger_validator._rust_module import no_parameters
    from rust_swagger_validator._rust_module import __build__
except ImportError:  # Support for inplace build
    from _rust_module import convert_string
    from _rust_module import no_parameters
    from _rust_module import __build__

if __name__ == '__main__':
    print(1)
    print(no_parameters())
    print(convert_string(1))
    print(__build__)
