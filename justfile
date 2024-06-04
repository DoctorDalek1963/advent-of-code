set dotenv-load

_default:
	@just --list

# format all the Rust files
fmt:
	fd -e rs -x rustfmt

# run this day with dhat-heap to profile memory usage
dhat-heap:
	cd {{invocation_directory()}} && cargo run --release --features dhat-heap

# open the dhat viewer to view dhat-heap.json files properly
open-dhat-viewer:
	xdg-open dhat/dh_view.html

# check and test only the years whose files are passed in, used for pre-commit
check-changed *args:
	#!/usr/bin/env python
	import re
	import subprocess

	input = "{{args}}".split()
	paths = set()
	for file in input:
		path = re.search(r"20\d\d/[^/]+/", file)
		if path is not None:
			paths.add(path.group(0))

	for path in paths:
		exit_code = subprocess.run(["nix", "flake", "check", f"path:{path}", "--print-build-logs", "--keep-going"]).returncode
		if exit_code != 0:
			exit(exit_code)

# check every flake
check-all:
	fd --full-path -t d '20\d\d/[^/]+$' -x nix flake check path:{} --print-build-logs --keep-going

# copy the scaffolding for the given day
_copy-scaffolding year lang day:
	mkdir -p "{{year}}/{{lang}}/day-{{day}}"
	cp -r ./scaffolding/{{lang}}/* "{{year}}/{{lang}}/day-{{day}}"
	@just _hydrate_{{lang}} {{year}} {{day}}

# fill in the Rust template for the given day
_hydrate_rust year day:
	fd -e rs -e toml . "{{year}}/rust/day-{{day}}/" -X sd -s "DAYNUM" "{{day}}"

# get the input file for the given day
_get-input year lang day:
	#!/usr/bin/env python
	from aocd import get_data

	with open('{{justfile_directory()}}/{{year}}/{{lang}}/day-{{day}}/input.txt', 'w') as f:
		f.write(get_data(day={{day}}, year={{year}}) + '\n')

# get the input files for every day
get-all-inputs:
	#!/usr/bin/env python
	import asyncio
	import re
	import subprocess

	async def main():
		days = subprocess.run(["fd", "-t", "d", "day-"], capture_output=True).stdout.split()
		processed_days = [re.match(r"(20\d\d)/([^/]+)/day-(\d+)/", d.decode("utf-8")).groups() for d in days]

		await asyncio.gather(
			*[asyncio.create_subprocess_shell(f"just _get-input {year} {lang} {day}") for (year, lang, day) in processed_days]
		)

	asyncio.run(main())

# setup the scaffolding and input file for the given day
setup year lang day:
	@just _copy-scaffolding {{year}} {{lang}} {{day}}
	@just _get-input {{year}} {{lang}} {{day}}

# setup the scaffolding and input file for today
today lang:
	just setup "$(date +'%Y')" "{{lang}}" "$(date +'%d' | sed 's/^0//')"
