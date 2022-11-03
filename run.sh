#!/bin/sh

set -e

day=$1

rustc "day${day}.rs" && ( cat "${day}.input" | "./day${day}" )
