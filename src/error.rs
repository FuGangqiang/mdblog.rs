use std::path::PathBuf;

error_chain!{
    links {
        Template(::tera::Error, ::tera::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error) #[doc = "Error during IO"];
        Fmt(::std::fmt::Error) #[doc = "Error during format"];
        Toml(::toml::de::Error) #[doc = "Error during deserialize toml config file"];
    }

    errors {
        RootDirExisted(path: PathBuf) {
             description("Blog root directory already exists")
             display("Blog root directory `{}` already exists", path.display())
        }
        ThemeNotFound(name: String) {
            description("Theme not found")
            display("Can not find theme `{}`", name)
        }
        PostHead(path: PathBuf) {
            description("Post head part parse error")
            display("Can not parse post `{}` head part", path.display())
        }
        PostNoBody(path: PathBuf) {
            description("Post must have body part")
            display("Post `{}` has not body part", path.display())
        }
    }
}
