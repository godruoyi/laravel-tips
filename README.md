# Laravel Tips [![Build Status]][actions]

[Build Status]: https://img.shields.io/github/actions/workflow/status/godruoyi/laravel-tips/ci.yml?branch=master

[actions]: https://github.com/godruoyi/laravel-tips/actions?query=branch%3Amaster

A small command program for laravel tips written in Rust. this repository is side project for learning Rust, if you are
interested in it, enjoy 🐕‍🦺🦧🦥.

## Principle

We load the all tips from [LaravelDaily laravel-tips](https://github.com/LaravelDaily/laravel-tips) and store them in a
json file, and then we use [bat](https://github.com/sharkdp/bat) to display the tips in the terminal.

## Basic Usage

```
Usage: laraveltips [-v] [<command>] [<args>]

A command line tool for laravel tips

Options:
  -v, --version     show version
  --help            display usage information
  --path|-p         set laravel tips storage path, default is ~/.laravel/tips.json

Commands:
  random            random laravel tips
  sync              sync laravel tips from laravel docs
```

## ScreenShot

![asciicast](https://user-images.githubusercontent.com/16079222/234809580-9742230b-5730-4bea-8cbf-e38ea4f84fef.gif)