#!/bin/sh

set -e

day=$1

rustc "$day.rs" && ( cat "$day.input" | "./$day" )
