// The graph header

use freelist::FreeList;
use graphlist::{Graph, GraphList};
use goblist::GOBList;

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    freelist: FreeList,
    graphlist: GraphList,
    goblist: GOBList,
}

impl Header {
    pub fn new(diskheadersize: u64, disksize: u64, headerspace: u64) -> Header {
        let freelist = FreeList::new(diskheadersize, disksize, headerspace);
        let graphlist = GraphList::new();
        let goblist = GOBList::new();

        Header {
            freelist,
            graphlist,
            goblist,
        }
    }

    pub fn create_graph(&self, name: String) -> Graph {
        let g = self.graphlist.create_graph(name);
        g
    }
}
