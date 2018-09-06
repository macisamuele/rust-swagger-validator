#!/bin/bash
set -euo pipefail -o posix -o functrace

SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

pushd "${SCRIPT_DIR}"

if [ -f appveyor_token.txt ]; then
    appveyorToken="$(cat appveyor_token.txt)"
else
    echo "
You can avoid writing the token all the times by writing it once in
${SCRIPT_DIR}/appveyor_token.txt
(be aware that the file should not be  committed on the repository)
" > /dev/stderr
    read -s -p "Introduce appveyor token
    NOTE: no characters will be visible while typing to protect your password length
" appveyorToken
fi

curl \
    --silent \
    --verbose \
    --request DELETE \
    --header "Authorization: Bearer ${appveyorToken}" \
    --header "Content-Type: application/json" \
    https://ci.appveyor.com/api/projects/macisamuele/rust-swagger-validator/buildcache &> /dev/stdout | \
    sed -r "s/${appveyorToken}/APPVEYOR_TOKEN/g"

popd
