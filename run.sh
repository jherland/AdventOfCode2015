#!/bin/sh

set -e

day=$1

cat "${day}.input" | cargo run --bin "day${day}"
