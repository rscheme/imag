use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;

use glob::glob;
use glob::Paths;

use storage::file::File;
use storage::file_id::*;

type BackendOperationResult = Result<(), StorageBackendError>;

pub struct StorageBackend {
    basepath: String,
}

impl StorageBackend {

    fn new(basepath: String) -> StorageBackend {
        StorageBackend {
            basepath: basepath,
        }
    }

    fn getFileList(&self) -> Option<Vec<FileID>> {
        let list = glob(&self.basepath[..]);

        if let Ok(globlist) = list {
            let mut v = vec![];
            for entry in globlist {
                if let Ok(path) = entry {
                    v.push(from_pathbuf(&path));
                } else {
                    // Entry is not a path
                }
            }

            Some(v)
        } else {
            None
        }
    }

    fn createEmpty(&self) -> Option<FileID> {
        use std::fs::File;
        use uuid::Uuid;
        let uuid = Uuid::new_v4().to_hyphenated_string();
        let pathstr = self.basepath + uuid.as_str();
        let path = Path::new(&pathstr);

        if let Ok(f) = File::create(path) {
            Some(uuid)
        } else {
            None
        }
    }

    fn createFile() -> File {
    }

    fn writeFile(f: File) -> BackendOperationResult {
    }

    fn createFileWithContent(content: String) -> BackendOperationResult {
    }

    fn readFile(id: FileID) -> String {
    }

    // TODO: Meta files are not covered yet

}

#[derive(Debug)]
pub struct StorageBackendError {
    pub action: String,             // The file system action in words
    pub desc: String,               // A short description
    pub explanation: String,        // A long, user friendly description
    pub dataDump: Option<String>    // Data dump, if any
}

impl StorageBackendError {
    fn new(action: &'static str,
           desc  : &'static str,
           explan: &'static str,
           data  : Option<String>) -> StorageBackendError
    {
        StorageBackendError {
            action:         String::from(action),
            desc:           String::from(desc),
            explanation:    String::from(explan),
            dataDump:       data,
        }
    }
}

impl Error for StorageBackendError {

    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl Display for StorageBackendError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "StorageBackendError[{}]: {}\n\n{}",
               self.action, self.desc, self.explanation)
    }
}

