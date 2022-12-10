set dotenv-load

# cargo check and test every day
check-and-test-all:
	for dir in $(fd -t d 20 -X fd -t d day); do just _check-and-test $dir; done

# cargo check and test the given directory
_check-and-test dir:
	#!/usr/bin/env bash
	cd {{dir}}
	cargo check
	cargo test
	cargo test --release

# copy the Rust project scaffolding for the given day
_copy-scaffolding day year:
	@mkdir -p "{{year}}"
	mkdir -p "{{year}}/day-{{day}}"
	cp -r ./scaffolding/rust/* "{{year}}/day-{{day}}"
	fd -e rs -e toml . "{{year}}/day-{{day}}/" -X sd -s "DAYNUM" "{{day}}"

# get the input file for the given day
_get-input day year:
	#!/usr/bin/env python

	from aocd import get_data
	with open('{{justfile_directory()}}/{{year}}/day-{{day}}/input.txt', 'w') as f:
		f.write(get_data(day={{day}}, year={{year}}) + '\n')

# setup the scaffolding and input file for the given day
setup day year:
	just _copy-scaffolding {{day}} {{year}}
	just _get-input {{day}} {{year}}

# setup the scaffolding and input file for today
today:
	just setup "$(date +'%d' | sed 's/^0//')" "$(date +'%Y')"

# make the current day use the nightly channel
nightly:
	echo "[toolchain]\nchannel = \"nightly\"" > {{invocation_directory()}}/rust-toolchain.toml
