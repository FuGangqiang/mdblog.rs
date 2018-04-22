created: 2018-04-22T12:31:00+08:00
tags: [tutorial]

when you installed [`mdblog`][],
you can use its subcommand `init` to initialize a new blog project.

[`mdblog`]: https://crates.io/crates/mdblog


## init blog project

```
mdblog init myblog
```

this command will create a new directory named `myblog` in the current directory,
and initialize its directory layout as:

```
myblog
├── config.toml
├── media/
├── posts/
└── _themes/
```

* `config.toml`: blog project config file
* `media`:  blog project media directory
* `posts`: blog project markdown posts directory
* `_themes`: blog project themes directory

## check blog project

now you can run `serve` command in the blog project directory
to check the web pages:

```
cd myblog
mdblog serve
```

the web browser will be automatic opened at the blog index page.
