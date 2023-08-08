use crate::network::radarr::DiskSpace;

#[derive(Default, Debug)]
pub struct RadarrData {
    pub free_space: u64,
    pub total_space: u64,
}

impl From<&DiskSpace> for RadarrData {
    fn from(disk_space: &DiskSpace) -> Self {
        RadarrData {
            free_space: disk_space.free_space.as_u64().unwrap(),
            total_space: disk_space.total_space.as_u64().unwrap()
        }
    }
}
