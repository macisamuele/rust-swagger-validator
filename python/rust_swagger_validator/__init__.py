# -*- coding: utf-8 -*-
"""
Top level rust-swagger-validator module
The module itself contains few versioning information
"""
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function
from __future__ import unicode_literals
__version__ = '0.0.0'

try:
    from rust_swagger_validator._rust_module import __build__  # noqa
except ImportError:  # pragma: no cover # Support for inplace build
    from _rust_module import __build__  # noqa
