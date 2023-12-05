.PHONY: all rust

all: input.txt rust

input.txt:
	if [[ -z "${AOC_SESSION}" ]]; then echo "Set environment variable AOC_SESSION to match session cookie in the browser"; else curl -L https://adventofcode.com/2023/day/4/input -o input.txt -H "Cookie: session=${AOC_SESSION}"; fi

rust:
	cd rust && cargo test test_ --all-features

