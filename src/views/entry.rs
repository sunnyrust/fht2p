use chrono::{DateTime, Local, TimeZone};
use std::cmp::Ordering;
use std::fmt;
use std::fs::{self, DirEntry, FileType};
use std::io;
use std::path::Path;
use std::time;

pub struct EntryMetadata {
    pub name: String,
    pub size: Option<u64>,
    pub modified: Option<DateTime<Local>>,
    pub typo: Option<FileType>,
}

impl EntryMetadata {
    pub fn new(d: &DirEntry, follow_links: bool, show_hider: bool) -> Option<Self> {
        let name = d.file_name().to_string_lossy().into_owned();
        if !show_hider && name.starts_with('.') {
            return None;
        }
        let metadata = d.metadata().ok();
        let typo = metadata.as_ref().map(|md| md.file_type());
        if !follow_links && typo.as_ref().map(|t| t.is_symlink()).unwrap_or(true) {
            return None;
        }
        Some(Self {
            name,
            typo,
            size: metadata.as_ref().map(|md| md.len()),
            modified: metadata.as_ref().and_then(|md| {
                md.modified()
                    .ok()
                    .and_then(|mt| mt.duration_since(time::UNIX_EPOCH).ok())
                    .map(|sd| Local.timestamp(sd.as_secs() as i64, sd.subsec_nanos()))
            }),
        })
    }
    pub fn read_dir<P: AsRef<Path>>(dir: P, follow_links: bool, show_hider: bool, order: &EntryOrder) -> io::Result<Vec<Self>> {
        let entries = fs::read_dir(dir)?;
        let mut entries_vec = Vec::new();
        // let mut name_len_max = 0;
        entries.filter_map(|e| e.ok()).for_each(|e| {
            if let Some(d) = EntryMetadata::new(&e, follow_links, show_hider) {
                entries_vec.push(d)
            }
        });
        order.sort(&mut entries_vec);
        Ok(entries_vec)
    }
}

#[derive(Debug)]
pub enum EntryOrder {
    /// if use None, conflicts with Option::None,
    Empty,
    Name,
    NameRev,
    Size,
    SizeRev,
    Modified,
    ModifiedRev,
}

impl EntryOrder {
    pub fn new(req_query: Option<&str>) -> Self {
        use self::EntryOrder::*;
        match req_query {
            None => Empty,
            Some(s) => {
                let lower = s.to_lowercase();
                match lower.as_str() {
                    "sort=name" => Name,
                    "sort=namerev" => NameRev,
                    "sort=size" => Size,
                    "sort=sizerev" => SizeRev,
                    "sort=modified" => Modified,
                    "sort=modifiedrev" => ModifiedRev,
                    _ => Empty,
                }
            }
        }
    }
    pub fn next(&self) -> (&'static str, &'static str, &'static str) {
        use self::EntryOrder::*;
        match *self {
            Empty | NameRev | ModifiedRev | SizeRev => ("Name", "Modified", "Size"),
            Name => ("NameRev", "Modified", "Size"),
            Size => ("Name", "Modified", "SizeRev"),
            Modified => ("Name", "ModifiedRev", "Size"),
        }
    }
    pub fn sort(&self, entries: &mut Vec<EntryMetadata>) {
        use self::EntryOrder::*;
        match *self {
            Empty => {}
            Name => entries.sort_by(|a, b| a.name.cmp(&b.name)),
            NameRev => entries.sort_by(|b, a| a.name.cmp(&b.name)),
            Size => entries.sort_by(|b, a| match (a.size.as_ref(), b.size.as_ref()) {
                (Some(aa), Some(bb)) => aa.cmp(bb),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            }),
            SizeRev => entries.sort_by(|a, b| match (a.size.as_ref(), b.size.as_ref()) {
                (Some(aa), Some(bb)) => aa.cmp(bb),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            }),
            Modified => entries.sort_by(|b, a| match (a.modified.as_ref(), b.modified.as_ref()) {
                (Some(aa), Some(bb)) => aa.cmp(bb),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            }),
            ModifiedRev => entries.sort_by(|a, b| match (a.modified.as_ref(), b.modified.as_ref()) {
                (Some(aa), Some(bb)) => aa.cmp(bb),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            }),
        }
    }
}

impl fmt::Display for EntryOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::EntryOrder::*;
        f.write_str(match *self {
            Empty => "Empty",
            Name => "Name",
            NameRev => "NameRev",
            Size => "Size",
            SizeRev => "SizeRev",
            Modified => "Modified",
            ModifiedRev => "ModifiedRev",
        })
    }
}
