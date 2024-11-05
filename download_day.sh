#!/bin/bash
source .env

day=$1
year=2020
digits=00
padded_day=${digits:${#day}:${#digits}}${day}

PUZZLE_URL="https://adventofcode.com/${year}/day/${day}/input"
OUT_FILE="data/inputs/${padded_day}.txt"

mkdir -p data/inputs

curl "${PUZZLE_URL}" -H "cookie: session=${AOC_SESSION}" -o "${OUT_FILE}" 2>/dev/null

