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
- [ ] Use SQLite FT5 support full-text search
- [ ] Support PostgreSQL engin
- [ ] PostgreSQL vector search?

## Basic Usage

```
A command line tool for laravel tips

Options:
  -v, --version     show version
  -e, --engin       specify the search engine, default is SQLite, support
                    [sqlite, file]
  --path            specify the path to store tips, default is $HOME/.laravel
  --help            display usage information

Commands:
  random            random laravel tips
  sync              sync laravel tips from laravel docs
  search            search laravel tips by keyword
```

## ScreenShot

![laravel-tips](https://user-images.githubusercontent.com/16079222/242636067-803c1c9c-1dfe-4f18-abaf-2921a734888d.gif)