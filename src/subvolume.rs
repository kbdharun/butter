mod list;
pub use list::SubvolList;
mod object;
pub use object::{Attribute, GSubvolume};

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zbus::zvariant::{Optional, Type};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, Type)]
pub struct Subvolume {
    pub subvol_path: PathBuf,
    pub mount_path: Optional<PathBuf>,
    pub is_mountpoint: bool,
    pub uuid: Uuid,
    pub id: u64,
    pub created_unix_secs: i64,
    pub snapshot_source_uuid: Optional<Uuid>,
}
