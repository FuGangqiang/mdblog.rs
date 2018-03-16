# Mdblog

Create static site blog from markdown files with features:

* markdown format
* TeX style math support
* highlight code block
* post tags index
* hidden post
* post title is the title of markdown file
* post url is some to path of markdown file


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
    mdblog server [-p <port>]  # unimplemented
    mdblog -v | --version
    mdblog -h | --help

Options:
    -h, --help          Display this message
    -v, --version       Print version info and exit
    -p, --port <port>   Server with port number
```

### init blog

```
$ mdblog init myblog
```

blog directory(`myblog`) layout is initialized:

```
$ tree myblog
myblog
├── Config.toml
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

the blog static files are build into the subdir `_build`, the current blog directory(`myblog`) layout is:

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
├── Config.toml
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


# Config.toml

```toml
theme = "simple"
site_logo = "/static/img/logo.png"
site_name = "Mdblog"
site_motto = "Simple is Beautiful!"
footer_note = "Keep It Simple, Stupid!"
```
