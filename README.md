# 🛠️ rekson
rectify your json

## 🍎 Motivation
I frequently edit [json](https://json.org) files.
It is a completely unusable configuration language with poorly written grammar.
So I came up with a simple program to fix manually edited json.
This is not a formatter, rather it is intended to be used in conjunction with one,
for example [`jq`](https://github.com/jqlang/jq).

## ✨ Features
 - [x] trailing comma (`{,"a":3,} -> {"a":3}`)
 - [x] comma between values (`[1 2] -> [1, 2]`)
 - [x] correct quotes (```{`key`: 'value'} -> {"key": "value"}```)
 - [x] multiline strings
 - [x] correct booleans (`True -> true`)
 - [x] correct nulls (`None -> null`)
 - [x] quote unknown values (`{a:3} -> {"a":3}`)
 - [x] correct colon (`{"a"=3} -> {"a":3}`)
 - [x] insert colon (`{b 4} -> {"b":4}`)
 - [x] fix brackets (`{[{[{]] -> {[{[{}]}]}`)
 - [x] replace parenthesis with brackets (`((),()) -> [[],[]]`)
 - [x] async io

## 💡 Plans
 - [ ] strip comments
 - [ ] convert numbers (`0xff -> 256`)
 - [ ] COW (reduce memory copying)
 - [ ] multithreading

## ✅ Pros
 - written in Rust => instant format on write
 - no dependencies => faster to build than to install an npm package
 - can be used as standalone executable
 - can be used as standalone library

## ❌ Cons
 - barebone simplicity => adding new features requires rethinking the architecture
 - single-threaded (for now)
 - no configuration (for now)
 - small testsuite (for now)

## 🚀 Installation

### crates.io
```bash
cargo install rekson
```

### From source
```bash
cargo install --git https://github.com/pinbraerts/rekson
```

### Binary
Prebuilt binaries can be found at the [releases](https://github.com/pinbraerts/rekson/releases) page.

## 👀 Alternatives
  - https://github.com/adhocore/php-json-fixer
  - https://github.com/rhysd/fixjson
  - https://github.com/Berkmann18/json-fixer
