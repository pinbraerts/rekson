# rekson
rectify your json

## Motivation
I frequently edit json.
This is a language with poorly written grammar, which makes it completely unusable.
Here is a simple program to fix manually edited json.

## Features
 - [x] trailing comma (`{"a":3,} -> {"a":3}`)
 - [x] comma between values (`[1 2] -> [1, 2]`)
 - [ ] insert/correct colon (`{a=3 b 4} -> {"a":3,"b":4}`)
 - [ ] multiline strings
 - [ ] correct quotes (`{key: 'value'} -> {"key": "value"}`)
 - [ ] correct booleans (`True -> true`)
 - [ ] correct nulls (`None -> null`)
 - [ ] async io

## Pros
 - written in Rust => instant save on write
 - no dependencies => faster to build than to install some npm packages
 - can be used as standalone executable
 - can be used as standalone library

## Cons
 - barebone simplicity => adding new features requires rethinking the architecture
 - this is not a formatter (intended to use in conjunction with one, for example [`jq`](https://github.com/jqlang/jq))
 - single-threaded blocking stdin and stdout (for now)
 - no configuration/cli params (for now)
 - small testsuite (for now)

## Alternatives
  - https://github.com/adhocore/php-json-fixer
  - https://github.com/rhysd/fixjson
  - https://github.com/Berkmann18/json-fixer
