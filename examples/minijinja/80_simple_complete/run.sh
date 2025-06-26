#!/usr/bin/env bash

set -eoux pipefail

punktf_binary="${EXAMPLES_BINARY:-punktf}"

"${punktf_binary}" \
	--verbose \
	deploy \
	--source . \
	--profile simple \
	--template-engine minijinja \
	--target "${EXAMPLES_TARGET:-/tmp}" \
	--dry-run
