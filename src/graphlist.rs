// Define the graph list

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    name: String,
    offset: u64, // Where on the disk this is
    size: u64,   // Size of the slot
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphList {
    graphs: Vec<Graph>,
}

impl GraphList {
    pub fn new() -> GraphList {
        let mut graphs = Vec::new();

        GraphList { graphs }
    }

    pub fn create_graph(&self, name: String) -> Graph {
        let offset = 0;
        let size = 0;
        let g = Graph { name, offset, size };
        g
    }
}
