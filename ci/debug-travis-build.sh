#!/bin/bash
set -euo pipefail -o posix -o functrace

SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

pushd "${SCRIPT_DIR}"

if [ $# -ne 1 ]; then
    echo -e "Usage: $0 <travis build id>\nMore Information on https://docs.travis-ci.com/user/running-build-in-debug-mode/" > /dev/stderr
    exit 1
fi
travisBuildId="$1"

if ! command -v travis &> /dev/null; then
    echo "travis CLI tool is required for $0 to work.
    Please install it and add it to your path.
    Documentation on https://github.com/travis-ci/travis.rb
    Tip: on Mac OS you can run \`brew install travis\`" > /dev/stderr
    exit 1
fi

travisToken=$(travis token --pro --no-interactive)

curl \
    --silent \
    --verbose \
    --request POST \
    --header "Content-Type: application/json" \
    --header "Accept: application/json" \
    --header "Travis-API-Version: 3" \
    --header "Authorization: token ${travisToken}" \
    --data "{\"quiet\": true}" \
    https://api.travis-ci.com/job/${travisBuildId}/debug &> /dev/stdout | \
    sed -r "s/${travisToken}/TRAVIS_TOKEN/g"

popd
