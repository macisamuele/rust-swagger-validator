# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function
from __future__ import unicode_literals

from argparse import ArgumentParser
from collections import defaultdict
from copy import deepcopy
from enum import Enum
from pathlib import Path
from re import sub
from sys import maxsize
from typing import Any  # noqa: F401
from typing import cast
from typing import DefaultDict  # noqa: F401
from typing import List
from typing import Mapping
from typing import MutableMapping  # noqa: F401
from typing import Optional  # noqa: F401
from typing import Text
from typing import Union

import jsonschema
from ruamel.yaml import YAML


def _init_yaml():
    # type: () -> YAML
    yaml = YAML()
    yaml.indent = 2
    # Prevent ruamel.yaml to wrap yaml lines
    yaml.width = maxsize
    return yaml


yaml = _init_yaml()


class OS(Enum):
    LINUX = 'linux'
    OSX = 'osx'
    WINDOWS = 'windows'


class PythonVersion(Enum):
    PY27 = '2.7'
    PY35 = '3.5'
    PY36 = '3.6'
    PY37 = '3.7'

    def specific_version(self):
        # type: () -> Text
        if self == PythonVersion.PY27:
            return '2.7.15'
        elif self == PythonVersion.PY35:
            return '3.5.6'
        elif self == PythonVersion.PY36:
            return '3.6.6'
        else:
            return '3.7.0'

    def windows_path(self):
        # type: () -> Text
        if self == PythonVersion.PY27:
            return 'C:\\Python27-x64'
        elif self == PythonVersion.PY35:
            return 'C:\\Python35-x64'
        elif self == PythonVersion.PY36:
            return 'C:\\Python36-x64'
        else:
            return 'C:\\Python37-x64'

    def circleci_docker_container(self):
        # type: () -> Text
        return 'circleci/python:{}'.format(self.specific_version())


class CI(Enum):
    APPVEYOR = ('appveyor_template.yaml', '../.appveyor.yml')
    CIRCLECI = ('circleci_template.yaml', '../.circleci/config.yml')
    TRAVISCI = ('travis_template.yaml', '../.travis.yml')

    @classmethod
    def get_CIs_for_OS(cls, os):
        # type: (OS) -> List['CI']
        if os == OS.WINDOWS:
            return [cls.TRAVISCI, cls.APPVEYOR]
        elif os == OS.OSX:
            return [cls.TRAVISCI]
        elif os == OS.LINUX:
            return [cls.TRAVISCI, cls.CIRCLECI]
        else:
            raise RuntimeError('Unsupported CI')

    def get_task(
        self,
        python_version,  # type: PythonVersion
        os,  # type: OS
        environment,  # type: MutableMapping[Text, Text]
        allow_failure,  # type: bool
    ):
        # type: (...) -> Mapping[Text, Any]
        env = deepcopy(environment)  # type: MutableMapping[Text, Text]
        if os == OS.WINDOWS and 'TOXENV' in env:
            env['TOXENV'] = ','.join(
                # This is needed on windows to overcome the issue of multiple installations
                # of the same interpreter (32 and 64 bits). By using py (with no version specifier)
                # tox will use the same python interpreter that executes tox
                # https://packaging.python.org/guides/supporting-windows-using-appveyor/#testing-with-tox
                sub('py\\d\\d', 'py', toxenv)
                for toxenv in env['TOXENV'].split(',')
            )

        task = {}  # type: MutableMapping[Text, Any]
        if self == CI.APPVEYOR:
            task['PYTHON'] = python_version.windows_path()
            task.update(env)
            if allow_failure:
                task['ALLOW_FAILURE'] = 'true'

        elif self == CI.CIRCLECI:
            task['docker'] = [{'image': python_version.circleci_docker_container()}]
            if allow_failure:
                task['environment'] = dict(env, ALLOW_FAILURE='true')
            else:
                task['environment'] = env

        elif self == CI.TRAVISCI:
            task['os'] = os.value
            task['env'] = ' '.join('{}={}'.format(k, v) for k, v in env.items())

            if os == OS.LINUX:
                task['python'] = python_version.value
#                if python_version == PythonVersion.PY37:
#                    task['sudo'] = 'required'
            elif os == OS.OSX:
                task['language'] = 'generic'
                task['env'] = ' '.join([task['env'], 'PYTHON={}'.format(python_version.specific_version())]).strip()

            if allow_failure:
                task['env'] = ' '.join([task['env'], 'ALLOW_FAILURE=true']).strip()

            if env.get('TOXENV') == 'coverage' and os == OS.LINUX:
                # Small hacks to get rust coverage running on travis -> https://github.com/codecov/example-rust
                task['sudo'] = 'required'
                task['addons'] = {
                    'apt': {
                        'packages': [
                            'libcurl4-openssl-dev',
                            'libelf-dev',
                            'libdw-dev',
                            'cmake',
                            'gcc',
                            'binutils-dev',
                            'libiberty-dev',
                        ],
                    },
                }

        else:
            raise RuntimeError('Unsupported CI')
        return task

    def get_allowed_failure(
        self,
        task,  # type: Mapping[Text, Union[Text, Mapping[Text, Text]]]
    ):
        # type: (...) -> Mapping[Text, Union[Text, Mapping[Text, Text]]]
        if self == CI.APPVEYOR:
            return {
                # In appveyor is enough to set ALLOW_FAILURE to 'true' (according to the template) to make
                # task failure allowed. So what matters is returning something, the content is not important
                # as it won't be printed on the final file. I'm Textinging the env for debugging purposes only
                'env': ' '.join('{}={}'.format(k, v) for k, v in task.items()),
            }
        elif self == CI.CIRCLECI:
            return deepcopy(task)
        elif self == CI.TRAVISCI:
            return {
                'env': task['env'],
            }
        else:
            raise RuntimeError('Unsupported CI')

    def write_configs(
        self,
        tasks,  # type: List[Mapping[Text, Union[Text, Mapping[Text, Text]]]]
        allowed_failures,  # type: List[Mapping[Text, Union[Text, Mapping[Text, Text]]]]
    ):
        # type: (...)  -> None
        ci_configuration_file = Path(__file__).resolve().parent / self.value[1]
        if not tasks:
            if ci_configuration_file.exists():
                ci_configuration_file.unlink()
        else:
            ci_config = yaml.load(Path(__file__).resolve().parent / self.value[0])
            if self == CI.APPVEYOR:
                ci_config['environment']['matrix'] = tasks
            elif self == CI.CIRCLECI:
                ci_config['jobs'] = {
                    task['environment'].get('TOXENV'): dict(ci_config['default'], **task)
                    for task in tasks
                    if isinstance(task, dict)
                }
                ci_config['workflows']['build_and_test']['jobs'] = list(ci_config['jobs'])
            elif self == CI.TRAVISCI:
                ci_config['matrix']['include'] = tasks
                if allowed_failures:
                    ci_config['matrix']['allow_failures'] = allowed_failures
            else:
                raise RuntimeError('Unsupported CI')

            yaml.dump(ci_config, ci_configuration_file)


def generate_configs(config_path):
    # type: (Text) -> None
    config = yaml.load(Path(config_path))

    tasks_ci_mapping = defaultdict(list)  # type: DefaultDict[CI, List[Mapping[Text, Union[Text, Mapping[Text, Text]]]]]
    allowed_failures_ci_mapping = defaultdict(list)  # type: DefaultDict[CI, List[Mapping[Text, Union[Text, Mapping[Text, Text]]]]]
    for testing_environment in config['testing_environments']:
        python_version = PythonVersion(testing_environment['python'])
        for os in testing_environment['os']:
            operating_system = OS(os)
            for ci in CI.get_CIs_for_OS(operating_system):
                allowed_failure = testing_environment.get('allowed_failure', False)
                task = ci.get_task(
                    python_version,
                    operating_system,
                    testing_environment.get('env', {}),
                    allowed_failure,
                )
                if task is not None:
                    tasks_ci_mapping[ci].append(task)
                if allowed_failure:
                    allowed_failures_ci_mapping[ci].append(
                        ci.get_allowed_failure(task),
                    )
    for ci in CI:
        tasks = tasks_ci_mapping.get(ci, cast(List[Mapping[Text, Union[Text, Mapping[Text, Text]]]], []))
        allowed_failures = allowed_failures_ci_mapping.get(ci, cast(List[Mapping[Text, Union[Text, Mapping[Text, Text]]]], {}))
        ci.write_configs(tasks, allowed_failures)


def validate_config(config_path):
    # type: (Text) -> None
    # This is needed as jsonschema expects `dict` objects while the default type (rt) returns ordered dicts
    yml = YAML(typ='safe')

    config = yml.load(Path(config_path))
    schema = yml.load(Path(__file__).resolve().parent / 'config-schema.json')

    jsonschema.validate(
        instance=config,
        schema=schema,
    )


def main(argv=None):
    # type: (Optional[List[Text]]) -> int
    parser = ArgumentParser('Support for CI-Configs')
    parser.add_argument(
        '--validate',
        action='store_true',
        help='Validate the provided configs',
    )
    parser.add_argument(
        '--generate',
        action='store_true',
        help='Generate the AppVeyor, CircleCi and TravisCI configurations',
    )
    parser.add_argument(
        '--config-file',
        default='ci/config.yaml',
        dest='config_file',
        help='Configuration file (default %(default)s)',
    )
    args = parser.parse_args(argv)

    if args.validate:
        validate_config(args.config_file)

    if args.generate:
        generate_configs(args.config_file)

    return 0


if __name__ == '__main__':
    exit(main())
