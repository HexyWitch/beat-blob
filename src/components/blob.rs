pub struct BlobSpawn {
    pub interval: i32,
    pub timer: i32,
}
pub struct BlobGoal;

pub struct Blob {
    pub path_index: usize,
    pub path: Vec<(i32, i32)>,
}
