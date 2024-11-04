# 🛠️ rekson
rectify your json

## 🍎 Motivation
I frequently edit json.
This is a language with poorly written grammar, which makes it completely unusable.
Here is a simple program to fix manually edited json.

## ✨ Features
 - [x] trailing comma (`{"a":3,} -> {"a":3}`)
 - [x] comma between values (`[1 2] -> [1, 2]`)
 - [x] correct quotes (```{`key`: 'value'} -> {"key": "value"}```)
 - [x] multiline strings
 - [x] correct booleans (`True -> true`)
 - [x] correct nulls (`None -> null`)
 - [x] correct colon (`{a=3} -> {"a":3}`)
 - [ ] convert numbers (`0xff -> 256`)
 - [ ] insert colon (`{b 4} -> {"b":4}`)
 - [ ] async io

## ✅ Pros
 - written in Rust => instant format on write
 - no dependencies => faster to build than to install an npm package
 - can be used as standalone executable
 - can be used as standalone library

## ❌ Cons
 - barebone simplicity => adding new features requires rethinking the architecture
 - this is not a formatter (intended to use in conjunction with one, for example [`jq`](https://github.com/jqlang/jq))
 - single-threaded blocking stdin and stdout (for now)
 - no configuration/cli params (for now)
 - small testsuite (for now)

## 👀 Alternatives
  - https://github.com/adhocore/php-json-fixer
  - https://github.com/rhysd/fixjson
  - https://github.com/Berkmann18/json-fixer
