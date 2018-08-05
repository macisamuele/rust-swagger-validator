if __name__ == '__main__':
    from rust_swagger_validator._rust_module import convert_string
    from rust_swagger_validator._rust_module import no_parameters
    from rust_swagger_validator._rust_module import __build__

    print(1)
    print(no_parameters())
    print(convert_string(1))
    print(__build__)
