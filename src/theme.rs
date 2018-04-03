use std::path::{Path, PathBuf};
use tera::Tera;
use utils::{write_file, read_file};
use errors::{Error, Result};

/// blog theme object
#[derive(Default)]
pub struct Theme {
    /// theme root directory
    pub root: PathBuf,
    /// theme name
    pub name: String,
    /// theme renderer
    pub renderer: Tera,
    favicon: Vec<u8>,
    logo: Vec<u8>,
    main_css: Vec<u8>,
    main_js: Vec<u8>,
    base: Vec<u8>,
    index: Vec<u8>,
    post: Vec<u8>,
    tag: Vec<u8>,
    rss: Vec<u8>,
}

impl Theme {
    pub fn new<P: AsRef<Path>>(root: P, name: &str) -> Result<Theme> {
        debug!("loading theme: {}", &name);
        let root = root.as_ref();
        let mut theme = Theme {
            root: root.to_owned(),
            name: name.to_string(),
            renderer: Tera::default(),
            ..Default::default()
        };
        let src_dir = root.join(name);
        if !src_dir.exists() {
            if name != "simple" {
                return Err(Error::ThemeNotFound(name.to_string()));
            }
            theme.favicon.extend_from_slice(&SIMPLE_FAVICON);
            theme.logo.extend_from_slice(&SIMPLE_LOGO);
            theme.main_css.extend_from_slice(&SIMPLE_MAIN_CSS);
            theme.main_js.extend_from_slice(&SIMPLE_MAIN_JS);
            theme.base.extend_from_slice(&SIMPLE_BASE);
            theme.index.extend_from_slice(&SIMPLE_INDEX);
            theme.post.extend_from_slice(&SIMPLE_POST);
            theme.tag.extend_from_slice(&SIMPLE_TAG);
            theme.rss.extend_from_slice(&SIMPLE_RSS);
            theme.init_template()?;
            return Ok(theme);
        }

        read_file(&src_dir.join("static/favicon.png"), &mut theme.favicon)?;
        read_file(&src_dir.join("static/logo.png"), &mut theme.logo)?;
        read_file(&src_dir.join("static/main.css"), &mut theme.main_css)?;
        read_file(&src_dir.join("static/main.js"), &mut theme.main_js)?;
        read_file(&src_dir.join("templates/base.tpl"), &mut theme.base)?;
        read_file(&src_dir.join("templates/index.tpl"), &mut theme.index)?;
        read_file(&src_dir.join("templates/post.tpl"), &mut theme.post)?;
        read_file(&src_dir.join("templates/tag.tpl"), &mut theme.tag)?;
        read_file(&src_dir.join("templates/rss.tpl"), &mut theme.rss)?;
        theme.init_template()?;
        return Ok(theme);
    }

    /// init renderer template.
    fn init_template(&mut self) -> Result<()> {
        self.renderer.add_raw_template("base.tpl", ::std::str::from_utf8(&self.base)?)?;
        self.renderer.add_raw_template("index.tpl", ::std::str::from_utf8(&self.index)?)?;
        self.renderer.add_raw_template("post.tpl", ::std::str::from_utf8(&self.post)?)?;
        self.renderer.add_raw_template("tag.tpl", ::std::str::from_utf8(&self.tag)?)?;
        self.renderer.add_raw_template("rss.tpl", ::std::str::from_utf8(&self.rss)?)?;
        Ok(())
    }

    /// create theme directory.
    pub fn init_dir(&self, name: &str) -> Result<()> {
        let dest_dir = self.root.join(name);
        if dest_dir.exists() {
            info!("theme({}) already existed", name);
            return Ok(());
        }
        debug!("init theme({}) ...", name);
        write_file(&dest_dir.join("static/favicon.png"), &self.favicon)?;
        write_file(&dest_dir.join("static/logo.png"), &self.logo)?;
        write_file(&dest_dir.join("static/main.css"), &self.main_css)?;
        write_file(&dest_dir.join("static/main.js"), &self.main_js)?;
        write_file(&dest_dir.join("templates/base.tpl"), &self.base)?;
        write_file(&dest_dir.join("templates/index.tpl"), &self.index)?;
        write_file(&dest_dir.join("templates/post.tpl"), &self.post)?;
        write_file(&dest_dir.join("templates/tag.tpl"), &self.tag)?;
        write_file(&dest_dir.join("templates/rss.tpl"), &self.rss)?;
        Ok(())
    }

    /// export theme static files.
    pub fn export_static<P: AsRef<Path>>(&self, root: P) -> Result<()> {
        debug!("exporting theme({}) static ...", self.name);
        let dest_dir = root.as_ref();
        write_file(&dest_dir.join("static/favicon.png"), &self.favicon)?;
        write_file(&dest_dir.join("static/logo.png"), &self.logo)?;
        write_file(&dest_dir.join("static/main.css"), &self.main_css)?;
        write_file(&dest_dir.join("static/main.js"), &self.main_js)?;
        Ok(())
    }
}

static SIMPLE_FAVICON: &'static [u8] = include_bytes!("simple_theme/static/favicon.png");
static SIMPLE_LOGO: &'static [u8] = include_bytes!("simple_theme/static/logo.png");
static SIMPLE_MAIN_CSS: &'static [u8] = include_bytes!("simple_theme/static/main.css");
static SIMPLE_MAIN_JS: &'static [u8] = include_bytes!("simple_theme/static/main.js");
static SIMPLE_BASE: &'static [u8] = include_bytes!("simple_theme/templates/base.tpl");
static SIMPLE_INDEX: &'static [u8] = include_bytes!("simple_theme/templates/index.tpl");
static SIMPLE_POST: &'static [u8] = include_bytes!("simple_theme/templates/post.tpl");
static SIMPLE_TAG: &'static [u8] = include_bytes!("simple_theme/templates/tag.tpl");
static SIMPLE_RSS: &'static [u8] = include_bytes!("simple_theme/templates/rss.tpl");
