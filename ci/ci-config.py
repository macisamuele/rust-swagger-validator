from argparse import ArgumentParser
from collections import defaultdict
from enum import Enum
from pathlib import Path
from sys import maxsize
from typing import DefaultDict
from typing import List
from typing import Mapping
from typing import MutableMapping

import jsonschema
from ruamel.yaml import YAML


def _init_yaml() -> YAML:
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

    def specific_version(self) -> str:
        if self == PythonVersion.PY27:
            return '2.7.15'
        elif self == PythonVersion.PY35:
            return '3.5.5'
        elif self == PythonVersion.PY36:
            return '3.6.5'
        elif self == PythonVersion.PY37:
            return '3.7.0'
        else:
            raise RuntimeError('Unsupported PythonVersion')

    def windows_path(self) -> str:
        if self == PythonVersion.PY27:
            return 'C:\Python27-x64'
        elif self == PythonVersion.PY35:
            return 'C:\Python35-x64'
        elif self == PythonVersion.PY36:
            return 'C:\Python36-x64'
        elif self == PythonVersion.PY37:
            return 'C:\Python37-x64'
        else:
            raise RuntimeError('Unsupported PythonVersion')

    def tox_py_target(self):
        if self == PythonVersion.PY27:
            return 'py27'
        elif self == PythonVersion.PY35:
            return 'py35'
        elif self == PythonVersion.PY36:
            return 'py36'
        elif self == PythonVersion.PY37:
            return 'py37'
        else:
            raise RuntimeError('Unsupported PythonVersion')


class CI(Enum):
    APPVEYOR = ('appveyor_template.yaml', '../.appveyor.yml')
    CIRCLECI = ('circleci_template.yaml', '../.circleci/config.yml')
    TRAVISCI = ('travis_template.yaml', '../.travis.yml')

    @classmethod
    def get_CIs_for_OS(cls, os: OS) -> List['CI']:
        if os == OS.WINDOWS:
            return [cls.APPVEYOR]
        else:
            # return [cls.CIRCLECI, cls.TRAVISCI]
            return [cls.TRAVISCI]

    def get_task(
        self,
        python_version: PythonVersion,
        os: OS,
        env: MutableMapping[str, str],
        allow_failure: bool,
    ) -> Mapping[str, str]:
        if os == OS.WINDOWS and 'TOXENV' in env:
            env['TOXENV'] = ','.join(
                # This is needed on windows to overcome the issue of multiple installations
                # of the same interpreter (32 and 64 bits). By using py (with no version specifier)
                # tox will use the same python interpreter that executes tox
                # https://packaging.python.org/guides/supporting-windows-using-appveyor/#testing-with-tox
                'py' if toxenv == python_version.tox_py_target() else toxenv
                for toxenv in env['TOXENV'].split(',')
            )

        task = {}
        if self == CI.APPVEYOR:
            task['PYTHON'] = python_version.windows_path()
            task.update(env)
            if allow_failure:
                task['ALLOW_FAILURE'] = 'true'

        elif self == CI.CIRCLECI:
            raise NotImplementedError()
        elif self == CI.TRAVISCI:
            task['os'] = os.value
            task['env'] = ' '.join(f'{k}={v}' for k, v in env.items())

            if os == OS.LINUX:
                task['python'] = python_version.value
                if python_version == PythonVersion.PY37:
                    task['sudo'] = 'required'
            elif os == OS.OSX:
                task['language'] = 'generic'
                task['env'] = ' '.join([task['env'], f'PYTHON={python_version.specific_version()}']).strip()

            if allow_failure:
                task['env'] = ' '.join([task['env'], 'ALLOW_FAILURE=true']).strip()
        else:
            raise RuntimeError('Unsupported CI')
        return task

    def get_allowed_failure(self, task: Mapping[str, str]) -> Mapping[str, str]:
        if self == CI.APPVEYOR:
            return {
                # In appveyor is enough to set ALLOW_FAILURE to 'true' (according to the template) to make
                # task failure allowed. So what matters is returning something, the content is not important
                # as it won't be printed on the final file. I'm stringing the env for debugging purposes only
                'env': ' '.join(f'{k}={v}' for k, v in task.items()),
            }
        elif self == CI.CIRCLECI:
            raise NotImplementedError()
        elif self == CI.TRAVISCI:
            return {
                'env': task['env'],
            }
        else:
            raise RuntimeError('Unsupported CI')

    def write_configs(self, tasks: List[Mapping[str, str]], allowed_failures: List[Mapping[str, str]]) -> None:
        ci_config = yaml.load(Path(__file__).resolve().parent / self.value[0])
        if self == CI.APPVEYOR:
            ci_config['environment']['matrix'] = tasks
        elif self == CI.CIRCLECI:
            raise NotImplementedError()
        elif self == CI.TRAVISCI:
            ci_config['matrix']['include'] = tasks
            if allowed_failures:
                ci_config['matrix']['allow_failures'] = allowed_failures
        else:
            raise RuntimeError('Unsupported CI')

        yaml.dump(ci_config, Path(__file__).resolve().parent / self.value[1])


def generate_configs(config_path: str) -> None:
    config = yaml.load(Path(config_path))

    tasks_ci_mapping: DefaultDict[CI, List[Mapping[str, str]]] = defaultdict(list)
    allowed_failures_ci_mapping: DefaultDict[CI, List[Mapping[str, str]]] = defaultdict(list)
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

    for ci, tasks in tasks_ci_mapping.items():
        if not tasks:
            continue
        ci.write_configs(tasks, allowed_failures_ci_mapping[ci])


def validate_config(config_path: str) -> None:
    # This is needed as jsonschema expects `dict` objects while the default type (rt) returns ordered dicts
    yml = YAML(typ='safe')

    config = yml.load(Path(config_path))
    schema = yml.load(Path(__file__).resolve().parent / 'config-schema.json')

    jsonschema.validate(
        instance=config,
        schema=schema,
    )


def main(argv: str=None) -> int:
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
