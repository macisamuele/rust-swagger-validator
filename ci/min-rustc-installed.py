#!/usr/bin/env python
from __future__ import print_function

import subprocess
import sys
from contextlib import contextmanager
from os import environ
from os.path import expanduser
from os.path import join
from os.path import pathsep

from pkg_resources import parse_version


@contextmanager
def add_to_path(*segments):
    old_path = environ.get('PATH')
    if old_path:
        segments = list(segments) + [old_path]
    environ['PATH'] = pathsep.join(segments)
    yield
    environ['PATH'] = old_path


# Constraints extracted from https://github.com/PyO3/pyo3/blob/master/build.rs#L14
MIN_VERSION = '1.30.0-nightly'
MIN_DATE = '2018-08-18'


def run_command(command):
    print('Run command: {command}'.format(command=command), file=sys.stderr)
    return_code, output = None, None
    with add_to_path(join(expanduser('~'), '.cargo', 'bin')):
        try:
            return_code, output = 0, subprocess.check_output(
                command,
                stderr=subprocess.STDOUT,
                shell=True,
            ).decode('utf-8')
        except subprocess.CalledProcessError as e:
            return_code, output = e.returncode, e.output.decode('utf-8')
        print('[return_code={return_code}] | {output}'.format(return_code=return_code, output=output), file=sys.stderr)
        return return_code, output


def check_rustc_version():
    return_code, rustc_version_output = run_command('rustc --version --verbose')
    assert return_code == 0, 'rustc --version --verbose failed'

    # `rustc --version --verbose` output looks like the following
    # rustc 1.30.0-nightly (73c78734b 2018-08-05)
    # binary: rustc
    # commit-hash: 73c78734bae8f2947a4bfdeabebeeb84ccf0b0e1
    # commit-date: 2018-08-05
    # host: x86_64-apple-darwin
    # release: 1.30.0-nightly
    # LLVM version: 7.0
    rustc_version = parse_version([
        line.replace('release: ', '')
        for line in rustc_version_output.splitlines()
        if line.startswith('release:')
    ][0])

    rustc_date = [
        line.replace('commit-date: ', '')
        for line in rustc_version_output.splitlines()
        if line.startswith('commit-date: ')
    ][0]

    assert parse_version(MIN_VERSION) <= rustc_version and MIN_DATE <= rustc_date, \
        'Min rustc requirement is not satisfied, please update cargo and rustc'


if __name__ == '__main__':
    check_rustc_version()
