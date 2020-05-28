created: 2018-04-23T09:48:50+08:00
tags: [tutorial]

now you have a blog project created by [mdblog][],
then you can use `new` subcommand to create a new bolg post.

[mdblog]: https://crates.io/crates/mdblog

## new blog post

```
mdblog new another
```

`mdblog` will create a new post with path `posts/another.md`,
you can also add blog tags to the new post using the `-t/--tag` argument:

```
mdblog new another -t test
```

## post title

`mdblog` use the post filename as the post title,
so `posts/another.md` blog post's title is `another`.

## two parts

every bolg post have two parts splitted by the first blank line:

* headers
* body

the `headers` part uses [yaml][] format, the body part uses [markdown][] format.

[yaml]: http://yaml.org
[markdown]: http://commonmark.org

the `posts/another.md` file content automatic created by mdblog:

```
created: 2018-04-23T10:01:09+08:00
tags: [test]

this is a new post!
```

the post headers part is:

```
created: 2018-04-23T10:01:09+08:00
tags: [test]
```

the post body part is:

```
this is a new post!
```


### headers part

`headers` parts is the blog post metadata:

* `created`: the post created time
* `tags`: the post blog tags
* `description`: the post description
* `hidden`: the hidden flag
* `title`: the blog title, use the file name if empty


### body part

the blog post content is converted by `mdblog` using body part,
you can use any markdown grammar in the `body` part.
