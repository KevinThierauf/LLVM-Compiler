use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::Result;

struct FileInfo {
    fullPath: PathBuf,
    fileSource: String,
}

impl FileInfo {
    fn new(path: PathBuf) -> Result<Self> {
        let fullPath = path.canonicalize()?;
        let mut fileSource = String::new();
        File::open(&fullPath)?.read_to_string(&mut fileSource)?;
        return Ok(Self {
            fullPath,
            fileSource,
        });
    }
}

#[derive(Clone)]
pub struct SourceFile {
    fileInfo: Rc<FileInfo>,
}

impl SourceFile {
    pub fn new(path: PathBuf) -> Result<Self> {
        return Ok(Self {
            fileInfo: Rc::new(FileInfo::new(path)?),
        });
    }

    pub fn fromSource(fullPath: PathBuf, fileSource: String) -> Self {
        return Self {
            fileInfo: Rc::new(FileInfo {
                fullPath,
                fileSource,
            }),
        };
    }

    pub fn getFilePath(&self) -> &Path {
        return &self.fileInfo.fullPath;
    }

    pub fn getLength(&self) -> usize {
        return self.fileInfo.fileSource.len();
    }

    pub fn getSource(&self) -> &str {
        return &self.fileInfo.fileSource;
    }
}

#[derive(Clone)]
pub struct FilePos {
    index: usize,
    sourceFile: SourceFile,
}

impl FilePos {
    pub fn new(sourceFile: SourceFile, index: usize) -> Self {
        assert!(index < sourceFile.fileInfo.fileSource.len(), "invalid index");
        return Self {
            index,
            sourceFile,
        };
    }

    pub fn getIndex(&self) -> usize {
        return self.index;
    }

    pub fn getSourceFile(&self) -> &SourceFile {
        return &self.sourceFile;
    }
}

pub struct FileRange {
    start: FilePos,
    length: usize,
}

impl Display for FileRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_str(&format!("{:?}:{} (through {})", self.start.sourceFile.fileInfo.fullPath, self.getStartIndex(), self.getEndIndex()));
    }
}

impl Debug for FileRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return <Self as Display>::fmt(self, f);
    }
}

impl FileRange {
    pub fn new(start: FilePos, length: usize) -> Self {
        assert!(start.index + length <= start.sourceFile.fileInfo.fileSource.len(), "range out of bounds");
        return Self {
            start,
            length,
        };
    }

    pub fn getSourceInRange(&self) -> &str {
        debug_assert!(self.getEndIndex() <= self.start.sourceFile.fileInfo.fileSource.len());
        let startPos = self.start.index;
        return &self.start.sourceFile.fileInfo.fileSource[startPos..startPos + self.length];
    }

    pub fn getStartIndex(&self) -> usize {
        return self.start.getIndex();
    }

    pub fn getEndIndex(&self) -> usize {
        return self.getStartIndex() + self.length;
    }
}
