# Mdblog

Create static site blog from markdown files with features:

* TeX style math support
* file path is the post url
* file name is the post title
* post can be hidden(link does not be insert into index/tag page)


# Install

`mdblog` is implemented by rust language, so you need cargo command:

```
cargo install mdblog
```

`mdblog` will be installed in your cargo binary directory(`~/.cargo/bin/`).


# CLI

`mdblog` can be use as a command:

```
$ mdblog -h
Usage:
    mdblog init <blog>
    mdblog build
    mdblog serve [-p <port>]
    mdblog -v | --version
    mdblog -h | --help

Options:
    -h, --help          Display this message
    -v, --version       Print version info and exit
    -p, --port <port>   Serve with port number
```

### init blog

```
$ mdblog init myblog
```

blog directory(`myblog`) layout is initialized:

```
myblog
├── Config.toml
├── media
├── posts
│   └── hello.md
└── _themes
```

* `Config.toml`: blog config file
* `media`: blog media directory
* `posts`: blog posts directory
* `posts/hello.md`: a markdown style post
* `_themes`: blog themes directory

### build blog

```
$ cd myblog
$ mdblog build
```

the blog static files are build into the subdir `_build`, the current blog directory(`myblog`) layout is:

```
myblog
├── _builds
├── Config.toml
├── media
├── posts
│   └── hello.md
└── _themes
```

* `_builds`: generated static-site top directory

### serve blog

```
$ mdblog serve
```

open the site index page automatically,
and re-generate your static-site when you add or change content,

### new post

create a markdown file `posts/another.md` with the content:

```
date: 2018-01-01 00:00:00
tags: hello, another

This is just another post.
```

refresh the index page, you will find the new post.


# Config.toml

```toml
footer_note = "Keep It Simple, Stupid!"
site_logo = "/static/logo.png"
site_motto = "Simple is Beautiful!"
site_name = "Mdblog"
theme = "simple"
```
