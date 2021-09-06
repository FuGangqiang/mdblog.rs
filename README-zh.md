# Mdblog

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![MIT/Apache2 licensed][license-image]
[![dependency status][deps-image]][deps-link]
[![Build Status][travis-image]][travis-link]

[crate-image]: https://img.shields.io/crates/v/mdblog.svg
[crate-link]: https://crates.io/crates/mdblog
[docs-image]: https://docs.rs/mdblog/badge.svg
[docs-link]: https://docs.rs/mdblog
[license-image]: https://img.shields.io/crates/l/mdblog.svg
[deps-image]: https://deps.rs/repo/github/fugangqiang/mdblog.rs/status.svg
[deps-link]: https://deps.rs/repo/github/fugangqiang/mdblog.rs
[travis-image]: https://travis-ci.org/FuGangqiang/mdblog.rs.svg?branch=master
[travis-link]: https://travis-ci.org/FuGangqiang/mdblog.rs

静态博客构建器，特性有：

* 支持 TeX 数学公式
* 使用文件路径生成博客文章 url
* 使用文件名作为博客文章标题
* 可以隐藏博客文章（首页文章列表不显示）

可以通过[示例博客网址](https://fugangqiang.github.io/mdblog.rs/)来进一步了解 `mdblog`。


# 安装

`mdblog` 由 rust 语言（2018 版）实现, 需要使用 `cargo` 命令安装:

```
cargo +nightly install mdblog
```

上面命令会将 `mdblog` 安装在 cargo 的二进制安装文件夹中，linux 操作系统下通常是 `~/.cargo/bin/`。


# 命令使用

`mdblog` 命令使用说明如下：

```
$ mdblog -h
static site generator from markdown files

USAGE:
    mdblog <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build    Build the blog static files
    help     Prints this message or the help of the given subcommand(s)
    init     Initialize the blog directory layout
    new      Create a blog post
    serve    Serve the blog, rebuild on change
    theme    Blog theme operations
```

通过下面方法检查子命令使用方法：

```
$ mdblog serve -h
Serve the blog, rebuild on change

USAGE:
    mdblog serve [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>    Serve the blog at http://127.0.0.1:<port> [default: 5000]
```


### 初始化博客

```
$ mdblog init myblog
```

上面命令初始化博客文件夹(`myblog`)，目录结构如下：

```
myblog
├── config.toml
├── media
├── posts
│   ├── hello.md
│   └── math.md
└── _themes
```

* `config.toml`: 博客配置文件
* `media`: 博客媒体文件夹，可以在里面放一些图片、视频文件
* `posts`: 博客文章文件夹
* `posts/hello.md`: 博客文章的一个 `markdown` 示例
* `_themes`: 博客样式文件夹


### 构建博客

```
$ cd myblog
$ mdblog build
```

上面命令会构建博客网站静态文件，存放到 `_build` 目录中，此时博客目录结构如下：

```
myblog
├── config.toml
├── media
├── posts
│   ├── hello.md
│   └── math.md
├── _themes
└── _builds
```

* `_builds`: 博客网站静态文件顶层目录


### 本地预览博客

```
$ mdblog serve
```

上面命令会自动在浏览器打开博客首页，此后当修改博客时，会自动重构博客静态文件。


### 创建博客文章

```
$ mdblog new another
```

上面命令生成一篇新的博客文章，
标题为 `another`，
文章文件路径为 `posts/another.md`，
直接编辑这个文件，
当 `serve` 命令运行时，会自动更新博客文章内容，
然后刷新一下浏览器相关页面，
预览新的博客内容。


# 博客配置文件：config.toml

```toml
site_url = ""
site_name = "Mdblog"
site_motto = "Simple is Beautiful!"
footer_note = "Keep It Simple, Stupid!"
media_dir = "media"
build_dir = "_build"
theme = "simple"
theme_root_dir = "_themes"
rebuild_interval = 2
posts_per_page = 20
```

上面是博客配置的选项及其默认值，说明如下：

- site_url: 博客首页 url，如果想要博客作为现有一个网站的子内容，挂载到某个 url 路径下，可以配置这个选项
- site_name: 博客名称，博客顶栏显示
- site_motto: 博客格言
- footer_note: 博客底栏备注
- media_dir: 媒体文件夹路径
- build_dir: 博客最终构建的静态文件存放路径
- theme: 博客样式名
- theme_root_dir: 博客样式配置文件路径
- rebuild_interval: `serve` 命令时，修改博客出发重新构建时间间隔，单位为秒
- posts_per_page: 首页文章目录页面每页文章链接数量

博客配置文件的使用示例可以参考 `docs` 目录的相关配置。
