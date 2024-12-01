#!/bin/bash
# Usage ./newday.sh <DAY>
DAY=$(echo $1 | sed 's/^0*//')

set -e

source .env

mkdir -p inputs puzzles
aoc download -d $DAY --input-file inputs/day$DAY.txt --puzzle-file puzzles/day$DAY.md --overwrite
cp -n template.rs src/day$DAY.rs

code puzzles/day$DAY.md src/bin/day$DAY.rs inputs/day$DAY.txt
