if __name__ == '__main__':
    from os.path import abspath
    from os.path import join

    from six.moves.urllib.parse import urljoin
    from rust_swagger_validator import _rust_module

    print(_rust_module.__build__)
    print(_rust_module.convert_string(1))
    print(_rust_module.no_parameters())
    print(_rust_module.__dict__)

    spec_url = urljoin('file:', abspath(join('test-data', 'json-valid-specs', 'swagger.json')))
    print(_rust_module.SwaggerSpec.from_url(spec_url).uri)
    print(_rust_module.SwaggerSpec.from_url(spec_url, False).uri)
    try:
        print(_rust_module.SwaggerSpec.from_url('test-data/json-valid-specs/swagger.json').uri)
    except Exception as e:
        print(f'Exception: {e}')
    try:
        print(_rust_module.SwaggerSpec.from_url('does-not-really-matter', True).uri)
    except Exception as e:
        print(f'Exception: {e}')
