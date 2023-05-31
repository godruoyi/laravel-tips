# Laravel Tips [![Build Status]][actions]

[Build Status]: https://img.shields.io/github/actions/workflow/status/godruoyi/laravel-tips/ci.yml?branch=master

[actions]: https://github.com/godruoyi/laravel-tips/actions?query=branch%3Amaster

A small command program for laravel tips written in Rust. this repository is side project for learning Rust, if you are
interested in it, enjoy 🐕‍🦺🦧🦥.

## Principle

We load the all tips from [LaravelDaily Tips](https://github.com/LaravelDaily/laravel-tips) and store them in a
json file, then we use [bat](https://github.com/sharkdp/bat) to display the tips in the terminal.

## TODO

- [x] Support file and SQLite engin
- [x] Use SQLite to store tips
- [x] Add search command
- [x] Multi thread download tips
- [ ] Use SQLite FT5 support full-text search
- [ ] Find a way to display the tips in the terminal(may [charmbracelet/glow](https://github.com/charmbracelet/glow))

## Basic Usage

```
A command line tool for laravel tips

Options:
  -v, --version     show version
  -e, --engin       specify the search engine, default is SQLite, support
                    [sqlite, file]
  --file-path       specify the file path to store tips, available when engin is
                    file, default is $HOME/.laravel/.tips
  --help            display usage information

Commands:
  random            random laravel tips
  sync              sync laravel tips from laravel docs
  search            search laravel tips by keyword
```

## ScreenShot

![asciicast](https://user-images.githubusercontent.com/16079222/234809580-9742230b-5730-4bea-8cbf-e38ea4f84fef.gif)