use std::path::{Path, PathBuf};

use tracing::{debug, info};
use tera::Tera;

use crate::error::{Error, Result};
use crate::utils::{read_file, write_file};

macro_rules! try_init_template {
    ($render:expr, $tpl_name:expr, $tpl_str:expr) => {
        let template_content = match std::str::from_utf8(&$tpl_str) {
            Ok(content) => content,
            Err(_) => return Err(Error::ThemeFileEncoding($tpl_name.into())),
        };
        $render.add_raw_template($tpl_name, template_content)?;
    };
}

macro_rules! try_read_file {
    ($src_dir: expr, $p: expr, $buf: expr) => {
        let p = $src_dir.join($p);
        if p.exists() {
            read_file(&p, $buf)?;
        }
    };
}

macro_rules! try_write_file {
    ($src_dir: expr, $dest_dir: expr, $p: expr, $buf: expr) => {
        let p = $src_dir.join($p);
        if p.exists() {
            write_file(&$dest_dir.join($p), $buf)?;
        }
    };
}

/// blog theme object
#[derive(Default)]
pub struct Theme {
    /// theme root directory
    pub root: PathBuf,
    /// theme name
    pub name: String,
    /// theme renderer
    pub renderer: Tera,
    main_css: Vec<u8>,
    atom: Vec<u8>,
    base: Vec<u8>,
    index: Vec<u8>,
    post: Vec<u8>,
    tag: Vec<u8>,
    tags: Vec<u8>,
}

impl Theme {
    /// create new `Theme`
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
                return Err(Error::ThemeNotFound(name.into()));
            }
            theme.main_css.extend_from_slice(SIMPLE_MAIN_CSS);
            theme.atom.extend_from_slice(SIMPLE_ATOM);
            theme.base.extend_from_slice(SIMPLE_BASE);
            theme.index.extend_from_slice(SIMPLE_INDEX);
            theme.post.extend_from_slice(SIMPLE_POST);
            theme.tag.extend_from_slice(SIMPLE_TAG);
            theme.tags.extend_from_slice(SIMPLE_TAGS);
            theme.init_template()?;
            return Ok(theme);
        }

        try_read_file!(src_dir, "static/main.css", &mut theme.main_css);
        read_file(&src_dir.join("templates/atom.tpl"), &mut theme.atom)?;
        read_file(&src_dir.join("templates/base.tpl"), &mut theme.base)?;
        read_file(&src_dir.join("templates/index.tpl"), &mut theme.index)?;
        read_file(&src_dir.join("templates/post.tpl"), &mut theme.post)?;
        read_file(&src_dir.join("templates/tag.tpl"), &mut theme.tag)?;
        read_file(&src_dir.join("templates/tags.tpl"), &mut theme.tags)?;
        theme.init_template()?;
        return Ok(theme);
    }

    /// init renderer template.
    fn init_template(&mut self) -> Result<()> {
        try_init_template!(self.renderer, "atom.tpl", self.atom);
        try_init_template!(self.renderer, "base.tpl", self.base);
        try_init_template!(self.renderer, "index.tpl", self.index);
        try_init_template!(self.renderer, "post.tpl", self.post);
        try_init_template!(self.renderer, "tag.tpl", self.tag);
        try_init_template!(self.renderer, "tags.tpl", self.tags);
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
        write_file(&dest_dir.join("static/main.css"), &self.main_css)?;
        write_file(&dest_dir.join("templates/atom.tpl"), &self.atom)?;
        write_file(&dest_dir.join("templates/base.tpl"), &self.base)?;
        write_file(&dest_dir.join("templates/index.tpl"), &self.index)?;
        write_file(&dest_dir.join("templates/post.tpl"), &self.post)?;
        write_file(&dest_dir.join("templates/tag.tpl"), &self.tag)?;
        write_file(&dest_dir.join("templates/tags.tpl"), &self.tags)?;
        Ok(())
    }

    /// export theme static files.
    pub fn export_static<P: AsRef<Path>>(&self, root: P) -> Result<()> {
        debug!("exporting theme({}) static ...", self.name);
        let src_dir = self.root.join(&self.name);
        let dest_dir = root.as_ref();
        try_write_file!(src_dir, dest_dir, "static/main.css", &self.main_css);
        Ok(())
    }
}

static SIMPLE_MAIN_CSS: &[u8] = include_bytes!("themes/simple/static/main.css");
static SIMPLE_ATOM: &[u8] = include_bytes!("themes/simple/templates/atom.tpl");
static SIMPLE_BASE: &[u8] = include_bytes!("themes/simple/templates/base.tpl");
static SIMPLE_INDEX: &[u8] = include_bytes!("themes/simple/templates/index.tpl");
static SIMPLE_POST: &[u8] = include_bytes!("themes/simple/templates/post.tpl");
static SIMPLE_TAG: &[u8] = include_bytes!("themes/simple/templates/tag.tpl");
static SIMPLE_TAGS: &[u8] = include_bytes!("themes/simple/templates/tags.tpl");
