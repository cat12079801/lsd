mod date;
mod filetype;
mod indicator;
mod name;
mod owner;
mod permissions;
mod size;
mod symlink;

pub use self::date::Date;
pub use self::filetype::FileType;
pub use self::indicator::Indicator;
pub use self::name::Name;
pub use self::owner::Owner;
pub use self::permissions::Permissions;
pub use self::size::Size;
pub use self::symlink::SymLink;
pub use icon::Icons;

use std::fs::read_link;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Meta {
    pub name: Name,
    pub path: PathBuf,
    pub permissions: Permissions,
    pub date: Date,
    pub owner: Owner,
    pub file_type: FileType,
    pub size: Size,
    pub symlink: SymLink,
    pub indicator: Indicator,
}

impl Meta {
    pub fn from_path(path: &PathBuf) -> Option<Self> {
        let a = 1;
        let m = if read_link(path).is_ok() {
            let metadata = path
                .symlink_metadata()
                .expect("failed to retrieve symlink metadata");
            (metadata, true)
        } else {
            let metadata = path
                .symlink_metadata()
                .expect("failed to retrieve symlink metadata");
            (metadata, false)
        };
        let metadata = if read_link(path).is_ok() {
            // If the file is a link, retrieve the metadata without following
            // the link.
            let metadata = path
                .symlink_metadata()
                .expect("failed to retrieve symlink metadata");
            let is_dir: bool = match path.metadata() {
                Ok(res) => res.file_type().is_dir(),
                Err(err) => {
                    println!("cannot access '{}': {}", path.display(), err);
                    return None;
                }
            };
            (metadata, is_dir)
        } else {
            match path.metadata() {
                //Ok(res) => (res, None),
                Ok(res) => (res, false),
                Err(err) => {
                    println!("cannot access '{}': {}", path.display(), err);
                    return None;
                }
            }
        };
        println!("######################");
        println!("{:?}", path);
        //println!("{:?}", metadata);
        //println!("{:?}", metadata.symlink_dir());

        let permissions = Permissions::from(&metadata);
        let file_type = FileType::new(&metadata, &permissions); //Some( Self {sym: meta, nosym: meta} )
        let name = Name::new(&path, file_type);

        Some(Self {
            path: path.to_path_buf(),
            symlink: SymLink::from(path.as_path()),
            size: Size::from(&metadata),
            date: Date::from(&metadata),
            indicator: Indicator::from(file_type),
            owner: Owner::from(&metadata),
            permissions,
            name,
            file_type,
        })
    }
}
