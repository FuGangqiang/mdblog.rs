# Mdblog

Create blog from markdown files.


# Unstable

This project is at an early stage and the API is a subject of changes.


# Install

```
cargo install mdblog
```


# Commands

```
mdblog init blog
mdblog build [-t theme]
mdblog server [-p port]
```


# Usage

### init blog directory

```
$ mdblog init myblog
```

### blog init directory tree

```
$ tree myblog
myblog
├── config.toml
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

8 directories, 12 files
```


### build blog

```
$ cd myblog
$ mdblog build
```


### build blog directory tree

```
$ tree .
.
├── _builds
│   ├── blog
│   │   ├── modified.html
│   │   ├── posts
│   │   │   └── hello.html
│   │   └── tags
│   │       ├── hello.html
│   │       └── world.html
│   ├── index.html
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

16 directories, 23 files
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
