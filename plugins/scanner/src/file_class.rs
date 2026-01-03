use std::path::{Path, PathBuf};

enum FileType {
    Executable(ExecutableType),
    Script(ScriptType),
    Archive(ArchiveType),
    Document(DocumentType),
    GenericBinary,
    Unknown,
}

enum ExecutableType {
    Elf,
}

enum ScriptType {
    Shell,
    Python,
    Perl,
    Php,
    Other,
}

enum ArchiveType {
    Zip,
    Tar,
    Gzip,
    SevenZip,
    Unknown,
}

enum DocumentType {
    Pdf,
    Other,
}

pub struct FileContext {
    path: PathBuf,
    detected_type: FileType,
    extension: Option<String>,
    size: u64,
}

struct MagicSig {
    offset: usize,
    bytes: &'static [u8],
    file_type: FileType,
}

static MAGIC_SIG: &[MagicSig] = &[
    MagicSig {
        offset: 0,
        bytes: b"\x7FELF",
        file_type: FileType::Executable(ExecutableType::Elf),
    },
    MagicSig {
        offset: 0,
        bytes: b"PK\x03\x04",
        file_type: FileType::Archive(ArchiveType::Zip),
    },
];

pub fn get(path: &Path) -> FileContext {
    println!("Finding file class for: {}", path.display());
    return FileContext {
        path: path.to_path_buf(),
        detected_type: FileType::Unknown,
        extension: None,
        size: 0,
    };
}
