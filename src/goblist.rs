// Define the graphic object list

#[derive(Serialize, Deserialize, Debug)]
enum GOBtype {
    Vertex,
    Edge,
    Property,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GOB {
    id: u64,
    gobtype: GOBtype,
    offset: u64, // Where on the disk this is
    size: u64,   // Size of the slot
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GOBList {
    goblist: Vec<GOB>,
}

impl GOBList {
    pub fn new() -> GOBList {
        let mut goblist = Vec::new();

        GOBList { goblist }
    }
}
