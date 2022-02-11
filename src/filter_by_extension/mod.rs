use std::collections::LinkedList;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::os::unix::ffi::OsStrExt;
type ThreadSafeGenResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
type BoxedIterOfThreadSafeGenResultOfBoxedDirectoryEntry =
    Box<dyn Iterator<Item = ThreadSafeGenResult<Box<dyn DirectoryEntry>>>>;

pub trait DirectoryEntry {
    fn is_file(&self) -> ThreadSafeGenResult<bool>;
    fn is_dir(&self) -> ThreadSafeGenResult<bool>;
    fn name(&self) -> OsString;
    fn get_read_dir(
        &self,
    ) -> ThreadSafeGenResult<Option<BoxedIterOfThreadSafeGenResultOfBoxedDirectoryEntry>>;
}

impl DirectoryEntry for std::fs::DirEntry {
    fn is_file(&self) -> ThreadSafeGenResult<bool> {
        Ok(self.file_type()?.is_file())
    }

    fn is_dir(&self) -> ThreadSafeGenResult<bool> {
        Ok(self.file_type()?.is_dir())
    }

    fn name(&self) -> OsString {
        self.file_name()
    }

    fn get_read_dir(
        &self,
    ) -> ThreadSafeGenResult<Option<BoxedIterOfThreadSafeGenResultOfBoxedDirectoryEntry>> {
        if self.is_dir()? {
            Ok(Some(Box::new(std::fs::read_dir(self.path())?.map(
                |dir_entry| match dir_entry {
                    Ok(dir_entry) => Ok(Box::new(dir_entry).into()),
                    Err(err) => Err(err.into()),
                },
            ))))
        } else {
            Ok(None)
        }
    }
}

impl From<Box<DirEntry>> for Box<dyn DirectoryEntry> {
    fn from(boxed_dir_entry: Box<DirEntry>) -> Self {
        Box::new(*boxed_dir_entry)
    }
}
pub fn filter_by_extension(
    root: &str,
    ext: &str,
) -> ThreadSafeGenResult<LinkedList<Box<dyn DirectoryEntry>>> {
    let mut lst = LinkedList::new();
    filter_by_extension_recursively(
        Box::new(std::fs::read_dir(root)?.map(|dir_entry| match dir_entry {
            Ok(dir_entry) => Ok(Box::new(dir_entry).into()),
            Err(err) => Err(err.into()),
        })),
        ext,
        &mut lst,
    )?;
    Ok(lst)
}
fn filter_by_extension_recursively(
    root: BoxedIterOfThreadSafeGenResultOfBoxedDirectoryEntry,
    ext: &str,
    filtered_files: &mut LinkedList<Box<dyn DirectoryEntry>>,
) -> ThreadSafeGenResult<()> {
    for entry in root {
        let entry = entry?;
        if entry.is_dir()? {
            filter_by_extension_recursively(entry.get_read_dir()?.unwrap(), ext, filtered_files)?;
        } else if entry.is_file()?
            && entry.name().as_bytes().len() > ext.len()
            && &entry.name().as_bytes()[entry.name().as_bytes().len() - ext.len()..]
                == ext.as_bytes()
        {
            filtered_files.push_back(entry);
        } else {
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
