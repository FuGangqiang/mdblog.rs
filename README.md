# Mdblog

create blog from markdown files.


# Api documentation

* [master](https://fugangqiang.github.io/docs/mdblog.rs/mdblog/)


# features

* markdown format
* TeX style math support
* highlight code block
* post tags index
* hidden post
* post title is the title of markdown file
* post url is some to path of markdown file


# Install

rustc 1.13 or later needed

```
cargo install mdblog
```


# Commands

```
mdblog init blog
mdblog build [-t theme]
mdblog server [-p port]   # unimplemented
```


# Usage

### init blog directory

```
$ mdblog init myblog
```

and the init blog directory tree is:

```
$ tree myblog
myblog
├── config.toml
├── media
├── posts
│   └── hello.md
└── _themes
    └── simple
        ├── static
        │   ├── css
        │   │   ├── highlight.css
        │   │   └── main.css
        │   ├── img
        │   │   ├── favicon.png
        │   │   └── logo.png
        │   └── js
        │       ├── highlight.js
        │       └── main.js
        └── templates
            ├── base.tpl
            ├── index.tpl
            ├── post.tpl
            └── tag.tpl

9 directories, 12 files
```


### build blog

```
$ cd myblog
$ mdblog build
```
and the result blog directory tree is:

```
$ tree .
.
├── _builds
│   ├── blog
│   │   ├── posts
│   │   │   └── hello.html
│   │   └── tags
│   │       ├── hello.html
│   │       └── world.html
│   ├── index.html
│   ├── media
│   └── static
│       ├── css
│       │   ├── highlight.css
│       │   └── main.css
│       ├── img
│       │   ├── favicon.png
│       │   └── logo.png
│       └── js
│           ├── highlight.js
│           └── main.js
├── config.toml
├── media
├── posts
│   └── hello.md
└── _themes
    └── simple
        ├── static
        │   ├── css
        │   │   ├── highlight.css
        │   │   └── main.css
        │   ├── img
        │   │   ├── favicon.png
        │   │   └── logo.png
        │   └── js
        │       ├── highlight.js
        │       └── main.js
        └── templates
            ├── base.tpl
            ├── index.tpl
            ├── post.tpl
            └── tag.tpl

18 directories, 22 files
```

### check blog in web broswer: [http://127.0.0.1:8000](http://127.0.0.1:8000)

```
$ cd _builds
$ python3 -m http.server --bind localhost 8000
```


# config.toml

```toml
[blog]
theme = simple
```
