#!/bin/bash

# updates Cargo.toml:
# - cuts everything after the first [[bin]]
# - adds all files in src/bin/*/*.rs as separate [[bin]] entries

cd "$(dirname "$0")"

sed -e '/\[\[bin\]\]/,$d' Cargo.toml > /tmp/apue.toml
mv /tmp/apue.toml Cargo.toml

for l in `ls src/bin/*/*.rs`; do
	bin=${l//src\/bin\/[^\/]*\//}
	bin=${bin//.rs/}
	echo "[[bin]]"
	echo name=\"$bin\"
	echo path = \"$l\"
	echo
done >> Cargo.toml