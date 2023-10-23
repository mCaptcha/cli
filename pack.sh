#!/bin/bash

# SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
#
# SPDX-License-Identifier: MIT

set -xEeuo  pipefail

GPG_TTY=$(tty)
DIST_DIR=dist

pack() {
	SRC=build/$1/
	TARBALL=$DIST_DIR/$1.tar.gz
	

	cp README.md  $SRC
	cp -r ./LICENSES/ $SRC
	tar -cvzf $TARBALL $SRC
	gpg --verbose \
		--pinentry-mode loopback \
		--batch --yes \
		--passphrase $GPG_PASSWORD \
		--local-user $KEY \
		--output $TARBALL.asc \
		--sign --detach \
		--armor $TARBALL
	pushd $DIST_DIR
	local_tar=$1.tar.gz
	sha256sum $local_tar > $local_tar.sha256
	sha256sum $local_tar.asc >> $local_tar.sha256
	popd
}

rm -rf $DIST_DIR
mkdir $DIST_DIR

for i in build/*
do
	arch=$(echo $i | cut -d '/' -f 2)
	pack $arch
done
