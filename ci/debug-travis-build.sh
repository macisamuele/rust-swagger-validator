#!/bin/bash
set -euo pipefail -o posix -o functrace

SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

pushd "${SCRIPT_DIR}"

if [ $# -ne 1 ]; then
    echo -e "Usage: $0 <travis build id>\nMore Information on https://docs.travis-ci.com/user/running-build-in-debug-mode/" > /dev/stderr
    exit 1
fi
travisBuildId="$1"

if [ -f travis_token.txt ]; then
    travisToken="$(cat travis_token.txt)"
else
    echo "
You can avoid writing the token all the times by writing it once in
${SCRIPT_DIR}/travis_token.txt
(be aware that the file should not be  committed on the repository)
" > /dev/stderr
    read -s -p "Introduce travis token
    NOTE: no characters will be visible while typing to protect your password length
" travisToken
fi

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
