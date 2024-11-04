# 🛠️ rekson
rectify your json

## 🍎 Motivation
I frequently edit [json](https://json.org) files.
It is a completely unusable configuration language with poorly written grammar.
So I came up with a simple program to fix manually edited json.

## ✨ Features
 - [x] trailing comma (`{"a":3,} -> {"a":3}`)
 - [x] comma between values (`[1 2] -> [1, 2]`)
 - [x] correct quotes (```{`key`: 'value'} -> {"key": "value"}```)
 - [x] multiline strings
 - [x] correct booleans (`True -> true`)
 - [x] correct nulls (`None -> null`)
 - [x] correct colon (`{"a"=3} -> {"a":3}`)
 - [x] quote unknown values (`{a:3} -> {"a":3}`)
 - [x] async io

## 💡 Plans
 - [ ] fix brackets (`{[()} -> {[()]}`)
 - [ ] convert numbers (`0xff -> 256`)
 - [ ] insert colon (`{b 4} -> {"b":4}`)
 - [ ] multithreading
 - [ ] COW (reduce memory usage)

## ✅ Pros
 - written in Rust => instant format on write
 - no dependencies => faster to build than to install an npm package
 - can be used as standalone executable
 - can be used as standalone library

## ❌ Cons
 - barebone simplicity => adding new features requires rethinking the architecture
 - this is not a formatter (intended to use in conjunction with one, for example [`jq`](https://github.com/jqlang/jq))
 - single-threaded (for now)
 - no configuration (for now)
 - small testsuite (for now)

## 👀 Alternatives
  - https://github.com/adhocore/php-json-fixer
  - https://github.com/rhysd/fixjson
  - https://github.com/Berkmann18/json-fixer
