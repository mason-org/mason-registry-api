#!/usr/bin/env bash

set -euo pipefail

BASE_URL=${BASE_URL:-"https://api.mason-registry.dev"}
ERRORS=$(mktemp)

function assert_equal {
    if [[ $1 != "$2" ]]; then
        echo "Expected \"$2\" to equal \"$1\". $3" | tee -a "$ERRORS"
    fi
}

function assert_response {
    declare EXPECT_STATUS=$1
    declare EXPECT_CONTENT_TYPE=$2

    declare BUFFER
    declare URL
    declare HTTP_CODE
    BUFFER=$(mktemp)
    < /dev/stdin cat > "$BUFFER"
    URL=$(awk "NR==1" "$BUFFER")
    HTTP_CODE=$(awk "NR==2" "$BUFFER")
    CONTENT_TYPE=$(awk "NR==3" "$BUFFER")

    assert_equal "$EXPECT_STATUS" "$HTTP_CODE" "$URL"
    assert_equal "$EXPECT_CONTENT_TYPE" "$CONTENT_TYPE" "$URL"

    rm "$BUFFER"
}

function assert_schema {
    declare ENDPOINT=$1
    declare SCHEMA_FILE=$2
    declare BUFFER
    BUFFER=$(mktemp -t XXXXXXXX.json)
    < /dev/stdin cat > "$BUFFER"
    if ajv validate -d "$BUFFER" --spec=draft2020 -c ajv-formats -s "$SCHEMA_FILE" > /dev/null; then
        echo "${ENDPOINT} is valid."
    else
        echo "Failed to validate ${ENDPOINT}" | tee -a "$ERRORS"
    fi
    rm "$BUFFER"
}

function assert_ok_json {
    declare ENDPOINT=$1
    declare SCHEMA_FILE=$2

    declare STDOUT
    STDOUT=$(mktemp)
    declare STDERR
    STDERR=$(mktemp)

    curl -sS -w "%{stderr}%{url}\n%{http_code}\n%{content_type}" "${BASE_URL}${ENDPOINT}" \
        >  "$STDOUT" \
        2> "$STDERR"

    < "$STDOUT" assert_schema "$ENDPOINT" "$SCHEMA_FILE"
    < "$STDERR" assert_response 200 "application/json"
}

function assert_not_found_json {
    declare ENDPOINT=$1

    declare STDOUT
    STDOUT=$(mktemp)
    declare STDERR
    STDERR=$(mktemp)

    curl -sS -w "%{stderr}%{url}\n%{http_code}\n%{content_type}" "${BASE_URL}${ENDPOINT}" \
        >  "$STDOUT" \
        2> "$STDERR"

    < "$STDOUT" assert_schema "$ENDPOINT" "./schemas/errors/not_found.json"
    < "$STDERR" assert_response 404 "application/json"
}

# mason
assert_ok_json  /api/mason/sponsors        ./schemas/mason/sponsors.json
assert_ok_json  /api/mason/renovate/badge  ./schemas/badge.json

# npm
assert_ok_json         /api/npm/typescript-language-server/versions/all           ./schemas/versions/all.json
assert_ok_json         /api/npm/typescript-language-server/versions/latest        ./schemas/versions/version.json
assert_ok_json         /api/npm/typescript-language-server/versions/3.0.0         ./schemas/versions/version.json
assert_ok_json         /api/npm/@ansible/ansible-language-server/versions/latest  ./schemas/versions/version.json
assert_not_found_json  /api/npm/typescript-language-server/versions/17287138

# packagist
assert_ok_json         /api/packagist/laravel/pint/versions/all       ./schemas/versions/all.json
assert_ok_json         /api/packagist/laravel/pint/versions/latest    ./schemas/versions/version.json
assert_ok_json         /api/packagist/laravel/pint/versions/v1.3.0    ./schemas/versions/version.json
assert_not_found_json  /api/packagist/laravel/pint/versions/17287138

# pypi
assert_ok_json         /api/pypi/cmake-language-server/versions/all       ./schemas/versions/all.json
assert_ok_json         /api/pypi/cmake-language-server/versions/0.1.6     ./schemas/versions/version.json
assert_not_found_json  /api/pypi/cmake-language-server/versions/17287138

# repo
for path in repo github; do
    assert_ok_json         /api/$path/sumneko/vscode-lua/releases/all       ./schemas/versions/all.json
    assert_ok_json         /api/$path/sumneko/vscode-lua/releases/latest    ./schemas/github/releases/release.json
    assert_ok_json         /api/$path/sumneko/vscode-lua/releases/v3.6.4    ./schemas/github/releases/release.json
    assert_not_found_json  /api/$path/sumneko/vscode-lua/releases/17287138
    assert_ok_json         /api/$path/sumneko/vscode-lua/tags/all           ./schemas/versions/all.json
    assert_ok_json         /api/$path/sumneko/vscode-lua/tags/latest        ./schemas/github/tags/tag.json
    assert_ok_json         /api/$path/sumneko/vscode-lua/tags/v3.6.4        ./schemas/github/tags/tag.json
    assert_not_found_json  /api/$path/sumneko/vscode-lua/tags/17287138
done

# rubygems
assert_ok_json         /api/rubygems/solargraph/versions/all       ./schemas/versions/all.json
assert_ok_json         /api/rubygems/solargraph/versions/latest    ./schemas/versions/version.json
assert_ok_json         /api/rubygems/solargraph/versions/0.48.0    ./schemas/versions/version.json
assert_not_found_json  /api/rubygems/solargraph/versions/17287138

# crates
assert_ok_json         /api/crate/crates_io_api/versions/all       ./schemas/versions/all.json
assert_ok_json         /api/crate/crates_io_api/versions/latest    ./schemas/versions/version.json
assert_ok_json         /api/crate/crates_io_api/versions/0.8.1     ./schemas/versions/version.json
assert_not_found_json  /api/crate/crates_io_api/versions/17287138

# golang
assert_ok_json  /api/golang/golang.org%2Fx%2Ftools%2Fgopls/versions/all  ./schemas/versions/all.json

if [[ $(wc -l "$ERRORS" | awk '{print $1}') -gt 0 ]]; then
    exit 1
fi
