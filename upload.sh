#!/bin/bash

# SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
#
# SPDX-License-Identifier: MIT

set -xEeuo  pipefail
readonly UPLOAD_DIR="cli/master"
readonly DUMBSERVE_HOST="https://$DUMBSERVE_USERNAME:$DUMBSERVE_PASSWORD@dl.mcaptcha.org"


curl --location --request DELETE "$DUMBSERVE_HOST/api/v1/files/delete" \
	--header 'Content-Type: application/json' \
	--data-raw "{
		\"path\": \"$UPLOAD_DIR\"
	}"


pushd dist/
for file in *
do
	curl -v \
		-F upload=@$file  \
		"$DUMBSERVE_HOST/api/v1/files/upload?path=$UPLOAD_DIR"
done
popd
