#!/bin/bash

set -euo pipefail

readonly OUT_FILE=out
readonly PHRASE="c353d3b790d0342e475b349605d770ca68fed0f0d022eefba3a9ab29ffbf"
readonly SALT="1661dd8198c5c28803cdc2e207e49ad64dbb8a99221276a8d3d910b42ecd"

readonly FACTOR=50000

rm $OUT_FILE || true

for f in $(seq 1 5 100)
do
	d=$(calc $FACTOR \* $f) #$(calc 10 ^ $f))
	echo "[*] factor: $f difficulty: $d"
	echo "[*] factor: $f difficulty: $d" >> $OUT_FILE
	{ time ./target/release/mcaptcha-cli -d $d -p $PHRASE -s $SALT > /dev/null; } 2>> $OUT_FILE
	echo >> $OUT_FILE
	echo >> $OUT_FILE
done
