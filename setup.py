import re

from setuptools import setup
from setuptools_rust import Binding
from setuptools_rust import RustExtension
from setuptools_rust import Strip


version = '0.0.0'
with open('python/rust_swagger_validator/__init__.py') as f:
    for line in f.readlines():
        match = re.match('^__version__ = \'(?P<version>.*)\'$', line)
        if match:
            version = match.group('version')
            break
    else:
        raise RuntimeError('No version is specified in __init__')


setup(
    name='rust_swagger_validator',
    version=version,
    setup_requires=['setuptools-rust>=0.9.2'],
    rust_extensions=[
        RustExtension(
            target='rust_swagger_validator._rust_module',
            path='Cargo.toml',
            binding=Binding.PyO3,
            strip=Strip.All,
        ),
    ],
    packages=['rust_swagger_validator'],
    package_dir={'': 'python'},
    zip_safe=False,
)
