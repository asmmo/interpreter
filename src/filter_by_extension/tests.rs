use super::*;
#[derive(Clone)]
enum EntryMock {
    File(String),
    Dir((String, Vec<EntryMock>)),
}
impl DirectoryEntry for EntryMock {
    fn is_file(&self) -> ThreadSafeGenResult<bool> {
        Ok(match self {
            EntryMock::Dir(_) => false,
            EntryMock::File(_) => true,
        })
    }
    fn is_dir(&self) -> ThreadSafeGenResult<bool> {
        Ok(match self {
            EntryMock::Dir(_) => true,
            EntryMock::File(_) => false,
        })
    }
    fn name(&self) -> OsString {
        match self {
            EntryMock::File(name) => name.into(),
            EntryMock::Dir((name, _)) => name.into(),
        }
    }
    fn get_read_dir(
        &self,
    ) -> ThreadSafeGenResult<Option<BoxedIterOfThreadSafeGenResultOfBoxedDirectoryEntry>> {
        Ok(match self {
            EntryMock::Dir((_, entries)) => Some(Box::new(
                entries.clone().into_iter().map(|e| Ok(Box::new(e).into())),
            )),
            EntryMock::File(_) => None,
        })
    }
}
impl From<Box<EntryMock>> for Box<dyn DirectoryEntry> {
    fn from(boxed_dir_entry: Box<EntryMock>) -> Self {
        Box::new(*boxed_dir_entry)
    }
}
// BoxedIterOfThreadSafeGenResultOfBoxedDirectoryEntry
#[test]
fn x() {
    let root = EntryMock::Dir((
        "root".into(),
        vec![
            EntryMock::Dir((
                "dir".into(),
                vec![
                    EntryMock::File("file1.rs".into()),
                    EntryMock::File("file2.txt".into()),
                    EntryMock::Dir(("dir2".into(), vec![EntryMock::File("file3.rs".into())])),
                ],
            )),
            EntryMock::File("file4.rs".into()),
        ],
    ));

    let mut files = LinkedList::new();
    filter_by_extension_recursively(root.get_read_dir().unwrap().unwrap(), ".rs", &mut files)
        .unwrap();
    assert_eq!(files.iter().filter(|e| e.is_file().unwrap()).count(), 3);
    assert_eq!(
        files.into_iter().map(|e| e.name()).collect::<Vec<_>>(),
        vec!["file1.rs", "file3.rs", "file4.rs"]
    );
}
