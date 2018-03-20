# Mdblog

Create static site blog from markdown files with features:

* markdown format
* TeX style math support
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
        │   ├── favicon.png
        │   ├── logo.png
        │   ├── main.css
        │   └── main.js
        └── templates
            ├── base.tpl
            ├── index.tpl
            ├── post.tpl
            └── tag.tpl

6 directories, 10 files
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
│       ├── favicon.png
│       ├── logo.png
│       ├── main.css
│       └── main.js
├── Config.toml
├── media
├── posts
│   └── hello.md
└── _themes
    └── simple
        ├── static
        │   ├── favicon.png
        │   ├── logo.png
        │   ├── main.css
        │   └── main.js
        └── templates
            ├── base.tpl
            ├── index.tpl
            ├── post.tpl
            └── tag.tpl

12 directories, 18 files
```

### check blog in web broswer: [http://127.0.0.1:8000](http://127.0.0.1:8000)

```
$ cd _builds
$ python3 -m http.server --bind localhost 8000
```


# Config.toml

```toml
footer_note = "Keep It Simple, Stupid!"
site_logo = "/static/logo.png"
site_motto = "Simple is Beautiful!"
site_name = "Mdblog"
theme = "simple"
```
