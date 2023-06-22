# Laravel Tips [![Build Status]][actions]

[Build Status]: https://img.shields.io/github/actions/workflow/status/godruoyi/laravel-tips/ci.yml?branch=master

[actions]: https://github.com/godruoyi/laravel-tips/actions?query=branch%3Amaster

A small command program for laravel tips written in Rust. this repository is side project for learning Rust, if you are
interested get some luck laravel tips in your terminal, try it 🍡🦤.

## Principle

We load the all tips from [LaravelDaily Tips](https://github.com/LaravelDaily/laravel-tips) and store them using SQLite
or
file, then we use [termimad](https://github.com/Canop/termimad) to display the tips in the terminal.

## Features

- [x] Support file and SQLite engin
- [x] Search tips by keyword
- [x] Multi thread download tips
- [x] Beautiful display tips in terminal(powered by [termimad](https://github.com/Canop/termimad))
- [ ] How to release binary file for multi-platform?
- [ ] Use SQLite FT5 support full-text search
- [ ] Support PostgreSQL engin and vector search?

## Installation with Raycast

<a title="Install laravel-tips Raycast Extension" href="https://www.raycast.com/Godruoyi/laravel-tips">
<img src="https://www.raycast.com/Godruoyi/laravel-tips/install_button@2x.png" height="64" alt="" style="height: 64px;">
</a>

We support Raycast Extension now 🎉🎉🎉

![laravel-tips-raycast](https://github.com/godruoyi/laravel-tips-raycast/blob/main/metadata/0.png)

## Installation with source code

We have not released the binary file yet(coming soon), you need to build it from source code now.

> You need to install [Rust](https://www.rust-lang.org/) first.

```shell
git clone https://github.com/godruoyi/laravel-tips.git
cd laravel-tips
cargo build --release

# try it, -__- should sync first
./target/release/laraveltips sync

# ./target/release/laraveltips random
# ./target/release/laraveltips search api

# or move it to your bin path
cp ./target/release/laraveltips /usr/local/bin/laraveltips
laraveltips random
```

## Basic Usage

```
Usage: laraveltips [-v] [-e <engin>] [--path <path>] [-o <output>] [-q] [<command>] [<args>]

A command line tool for laravel tips

Options:
  -v, --version     show version
  --path            specify the path to store tips, default is $HOME/.laravel
  -e, --engin       specify the search engine, default is SQLite, support [sqlite, file]
  -o, --output      specify the output format, default is display in terminal, support [text, json]
  -q, --quiet       quiet mode, only output the result
  --help            display usage information

Commands:
  random            random laravel tips
  sync              sync laravel tips from laravel docs
  search            search laravel tips by keyword
```

## ScreenShot

![laravel-tips](https://user-images.githubusercontent.com/16079222/242636067-803c1c9c-1dfe-4f18-abaf-2921a734888d.gif)

## License

MIT License

