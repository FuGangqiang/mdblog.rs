use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::{Path, PathBuf};


pub struct Post {
    pub root: PathBuf,
    pub path: PathBuf,
    pub head: String,
    pub body: String,
    pub metadata: HashMap<String, String>,
}


impl Post {
    pub fn new<P: AsRef<Path>>(root: P, path: P) -> Post {
        Post {
            root: root.as_ref().to_owned(),
            path: path.as_ref().to_owned(),
            head: String::new(),
            body: String::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn load(&mut self) -> ::std::io::Result<()> {
        let mut pf = File::open(self.root.join(&self.path))?;
        let mut content = String::new();
        pf.read_to_string(&mut content)?;
        let v: Vec<&str> = content.splitn(2, "\n\n").collect();
        if v.len() != 2 {
            return Err(Error::new(ErrorKind::Other,
                                  format!("post({path}) must both have `head` and `body` parts",
                                          path=self.path.display())));
        }
        self.head = v[0].to_string();
        self.body = v[1].to_string();
        for line in self.head.lines() {
            let pair: Vec<&str> = line.splitn(2, ':').collect();
            if pair.len() != 2 {
                return Err(Error::new(ErrorKind::Other,
                                      format!("post({path}) `head` part parse error: {line}",
                                              path=self.path.display(),
                                              line=line)));
            }
            self.metadata.insert(pair[0].trim().to_owned(), pair[1].trim().to_owned());
        }
        Ok(())
    }
}
