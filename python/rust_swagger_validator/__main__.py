# -*- coding: utf-8 -*-
"""
Simple python file to show capabilities of the rust-binded library

NOTE: this is at the moment used for simple manual testing
"""
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function
from __future__ import unicode_literals


def main():
    # type: () -> int
    """
    Run a set of actions with the library
    """
    from os.path import abspath
    from os.path import join

    from six.moves.urllib.parse import urljoin
    from rust_swagger_validator import _rust_module
    from rust_swagger_validator._rust_module import RustSwaggerSpec
    from rust_swagger_validator.swagger_spec import SwaggerSpec

    print(_rust_module.__build__)
    print(_rust_module.convert_string(1))
    print(_rust_module.no_parameters())
    print(_rust_module.__dict__)

    spec_url = urljoin('file:', abspath(join('test-data', 'json-valid-specs', 'swagger.json')))
    print(RustSwaggerSpec.from_url(spec_url).uri)
    print(RustSwaggerSpec.from_url(spec_url, False).uri)
    try:
        print(RustSwaggerSpec.from_url('test-data/json-valid-specs/swagger.json').uri)
    except Exception as exception:  # pylint: disable=W0703
        print('Exception: {}'.format(exception))
    try:
        print(RustSwaggerSpec.from_url('does-not-really-matter', True).uri)
    except Exception as exception:  # pylint: disable=W0703
        print('Exception: {}'.format(exception))
    swagger_spec = RustSwaggerSpec.from_url(spec_url, False)
    print(type(swagger_spec))
    print(isinstance(swagger_spec, RustSwaggerSpec))
    print(isinstance(swagger_spec, SwaggerSpec))
    print(type(swagger_spec).__bases__)

    return 0


if __name__ == '__main__':
    exit(main())
