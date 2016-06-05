# mdblog

Create blog from markdown files.

# unstable

This project is at a very early stage and the API is a subject of changes.

## commands

```
mdblog init blog
mdblog build [-t theme]
mdblog server [-p port]
```

## directory struct

```
- blog
    - posts
        - p0.md
        - d1
            - p1.md
        - d2
            - d3
                - p2.md
    - config.toml
    - builds
        - index.html
        - blog
            - post
                - p0.html
                - d1
                    - p1.html
                - d2
                    - d3
                        - p2.html
            - tag
                - index.html
                - t1.html
            - static
                - css
                    - main.css
                    - highlight.css
                - js
                    - main.js
                    - highlight.js
                - images
                    - logo.png
                    - favicon.png
```

## config.toml

```toml
[blog]
theme = simple
```
