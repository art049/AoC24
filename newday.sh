#!/bin/bash
# Usage ./newday.sh <DAY>
DAY=$(echo $1 | sed 's/^0*//')

set -e

mkdir -p inputs puzzles
aoc download -d $DAY --input-file inputs/day$DAY.txt --puzzle-file puzzles/day$DAY.md --overwrite
cp -n template.rs src/day$DAY.rs

echo pub mod day$DAY\; >>src/lib.rs
