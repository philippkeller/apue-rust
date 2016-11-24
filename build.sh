#!/bin/bash

# builds Cargo.toml for linux and macos
# and only includes files in src/bin/*/*.rs which
# are not excluded by a #!cfg(target_os="...") attribute

cd "$(dirname "$0")"

for os in linux macos; do
	echo "creating Cargo.toml for $os"
	mkdir -p $os
	cp Cargo.toml $os/Cargo.toml
	echo "" >> $os/Cargo.toml
	for l in `ls src/bin/*/*.rs`; do
			bin=${l//src\/bin\/[^\/]*\//}
			bin=${bin//.rs/}
			echo "[[bin]]"
			echo name=\"$bin\"
			echo path = \"$l\"
			echo
	done >> $os/Cargo.toml
done

if [ `uname -s` == "Darwin" ]; then
	echo "building for Macos"
	cargo build --manifest-path macos/Cargo.toml
elif [ `uname -s` == "Linux" ]; then
	echo "building for Linux"
	cargo build --manifest-path linux/Cargo.toml
fi
