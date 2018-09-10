# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function
from __future__ import unicode_literals


if __name__ == '__main__':
    from os.path import abspath
    from os.path import join

    from six.moves.urllib.parse import urljoin
    from rust_swagger_validator import _rust_module
    from rust_swagger_validator._rust_module import RustSwaggerSpec as SwaggerSpec

    print(_rust_module.__build__)
    print(_rust_module.convert_string(1))
    print(_rust_module.no_parameters())
    print(_rust_module.__dict__)

    spec_url = urljoin('file:', abspath(join('test-data', 'json-valid-specs', 'swagger.json')))
    print(SwaggerSpec.from_url(spec_url).uri)
    print(SwaggerSpec.from_url(spec_url, False).uri)
    try:
        print(SwaggerSpec.from_url('test-data/json-valid-specs/swagger.json').uri)
    except Exception as e:
        print('Exception: {}'.format(e))
    try:
        print(SwaggerSpec.from_url('does-not-really-matter', True).uri)
    except Exception as e:
        print('Exception: {}'.format(e))
    swagger_spec = SwaggerSpec.from_url(spec_url, False)
    print(type(swagger_spec))
    print(isinstance(swagger_spec, SwaggerSpec))
    print(type(swagger_spec).__bases__)
