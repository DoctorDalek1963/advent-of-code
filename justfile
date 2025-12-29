set dotenv-load := true

_default:
    @just --list

# (rust only) run this day with dhat-heap to profile memory usage
[group("profile")]
dhat-heap:
    cd {{ invocation_directory() }} && cargo run --release --features dhat-heap

# open the dhat viewer to view dhat-heap.json files properly
[group("profile")]
open-dhat-viewer:
    xdg-open dhat/dh_view.html

# check and test only the years whose files are passed in, used for pre-commit
[group("check")]
check-changed *args:
    #!/usr/bin/env python
    import os
    import re
    import subprocess

    input = "{{ args }}".split()
    paths = set()
    for file in input:
        path = re.search(r"20\d\d/[^/]+/", file)
        if path is not None:
            paths.add(path.group(0))

    for path in paths:
        child = subprocess.run(
            ["direnv allow && direnv exec . just check"],
            cwd=path,
            shell=True,
            env=(os.environ | {"_nix_direnv_force_reload": "1"}),
        )

        if (exit_code := child.returncode) != 0:
            exit(exit_code)

# check every day
[group("check")]
check-all:
    #!/usr/bin/env bash
    rc=0
    _nix_direnv_force_reload=1

    for d in {{ source_directory() }}/20??/*; do
        direnv allow "$d"
        cd "$d"
        direnv exec . just check-year
        [ $? -ne 0 ] && rc=1
    done
    exit $rc

# copy the Elixir scaffolding for the given day
[group("setup")]
_copy-scaffolding-elixir year day:
    mkdir -p "{{ source_directory() }}/{{ year }}/elixir/apps/"
    cp -r {{ source_directory() }}/scaffolding/elixir "{{ source_directory() }}/{{ year }}/elixir/apps/day_{{ day }}"
    fd -e ex -e exs . "{{ source_directory() }}/{{ year }}/elixir/apps/day_{{ day }}/" -X sd -s "DAYNUM" "{{ day }}"
    sd -s "YEARNUM" "{{ year }}" "{{ source_directory() }}/{{ year }}/elixir/apps/day_{{ day }}/lib/dayDAYNUM.ex"
    fd . "{{ source_directory() }}/{{ year }}/elixir/apps/day_{{ day }}" -x rename "DAYNUM" "{{ day }}" {} || true

# copy the Kotlin scaffolding for the given day
[group("setup")]
_copy-scaffolding-kotlin year day:
    cp {{ source_directory() }}/scaffolding/kotlin/Day.kt "{{ source_directory() }}/{{ year }}/kotlin/lib/src/main/kotlin/com/github/doctordalek1963/aoc{{ year }}/Day{{ day }}.kt"
    cp {{ source_directory() }}/scaffolding/kotlin/Test.kt "{{ source_directory() }}/{{ year }}/kotlin/lib/src/test/kotlin/com/github/doctordalek1963/aoc{{ year }}/Day{{ day }}Test.kt"
    sd -s "DAYNUM" "{{ day }}" "{{ source_directory() }}/{{ year }}/kotlin/lib/src/main/kotlin/com/github/doctordalek1963/aoc{{ year }}/Day{{ day }}.kt" \
        "{{ source_directory() }}/{{ year }}/kotlin/lib/src/test/kotlin/com/github/doctordalek1963/aoc{{ year }}/Day{{ day }}Test.kt"
    sd -s "YEARNUM" "{{ year }}" "{{ source_directory() }}/{{ year }}/kotlin/lib/src/main/kotlin/com/github/doctordalek1963/aoc{{ year }}/Day{{ day }}.kt" \
        "{{ source_directory() }}/{{ year }}/kotlin/lib/src/test/kotlin/com/github/doctordalek1963/aoc{{ year }}/Day{{ day }}Test.kt"

# copy the Rust scaffolding for the given day
[group("setup")]
_copy-scaffolding-rust year day:
    mkdir -p "{{ source_directory() }}/{{ year }}/rust/day-{{ day }}"
    cp -r {{ source_directory() }}/scaffolding/rust/* "{{ source_directory() }}/{{ year }}/rust/day-{{ day }}"
    fd -e rs -e toml . "{{ source_directory() }}/{{ year }}/rust/day-{{ day }}/" -X sd -s "DAYNUM" "{{ day }}"

# copy the Zig scaffolding for the given day
[group("setup")]
_copy-scaffolding-zig year day:
    mkdir -p "{{ source_directory() }}/{{ year }}/zig/day-{{ day }}"
    cp -r {{ source_directory() }}/scaffolding/zig/* "{{ source_directory() }}/{{ year }}/zig/day-{{ day }}"
    fd -e zig -e zon . "{{ source_directory() }}/{{ year }}/zig/day-{{ day }}/" -X sd -s "DAYNUM" "{{ day }}"

# get the input file for the given day
[group("setup")]
get-input year lang day:
    #!/usr/bin/env python
    from aocd import get_data

    # Map from the language to the right path for input.txt
    language_map = {
        "d": "{{ source_directory() }}/{{ year }}/d/day-{{ day }}/input.txt",
        "elixir": "{{ source_directory() }}/{{ year }}/elixir/apps/day_{{ day }}/input.txt",
        "kotlin": "{{ source_directory() }}/{{ year }}/kotlin/lib/inputs/day{{ day }}.txt",
        "rust": "{{ source_directory() }}/{{ year }}/rust/day-{{ day }}/input.txt",
        "zig": "{{ source_directory() }}/{{ year }}/zig/day-{{ day }}/src/input.txt",
    }

    with open(language_map["{{ lang }}"], "w") as f:
        f.write(get_data(day={{ day }}, year={{ year }}) + '\n')

# get the input files for every day
[group("setup")]
get-all-inputs:
    #!/usr/bin/env python
    import asyncio
    import re
    import subprocess

    async def main():
        days = subprocess.run(["fd", "-t", "d", "day"], capture_output=True).stdout.split()
        processed_days = [re.match(r"(20\d\d)/([^/]+)/(apps/)?day[-_](\d+)/", d.decode("utf-8")).groups() for d in days]

        await asyncio.gather(
            *[asyncio.create_subprocess_shell(f"just get-input {year} {lang} {day}") for (year, lang, _, day) in processed_days]
        )

    asyncio.run(main())

# setup the scaffolding and input file for the given day
[group("setup")]
setup year lang day:
    @just _copy-scaffolding-{{ lang }} {{ year }} {{ day }}
    @just get-input {{ year }} {{ lang }} {{ day }}

# setup the scaffolding and input file for today
[group("setup")]
today lang:
    just setup "$(date +'%Y')" "{{ lang }}" "$(date +'%d' | sed 's/^0//')"
