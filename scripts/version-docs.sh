#!/usr/bin/env bash
# [AIR-3][AIS-3][BPC-3][RES-3]
set -eo pipefail

VERSION_TAG=$(git describe --tags --always --match 'v*.*.*')
COMMIT_HASH=$(git rev-parse --short HEAD)
BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

DOC_DIR="docs/versions/${VERSION_TAG}-${COMMIT_HASH}"
mkdir -p "${DOC_DIR}"

rsync -aq docs/ "${DOC_DIR}/" \
  --exclude='versions/' \
  --exclude='node_modules/' \
  --exclude='.cache/'

find "${DOC_DIR}" -type f -name '*.md' -exec sed -i.bak \
  -e "s/{{VERSION}}/${VERSION_TAG}/g" \
  -e "s/{{COMMIT}}/${COMMIT_HASH}/g" \
  -e "s/{{DATE}}/${BUILD_DATE}/g" {} \;

rm -f "${DOC_DIR}"/*.bak

ln -sfn "${DOC_DIR}" docs/current

echo "Versioned docs generated: ${DOC_DIR}" 