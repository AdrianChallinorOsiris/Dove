// Define the free list and the free slots

#[derive(Serialize, Deserialize, Debug)]
pub struct FreeSlot {
    offset: u64, // Where on the disk this is
    size: u64,   // Size of the slot
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreeList {
    slots: Vec<FreeSlot>,
}

impl FreeList {
    pub fn new(diskheadersize: u64, disksize: u64, headerspace: u64) -> FreeList {
        let mut slots = Vec::new();

        let offset = diskheadersize;
        let size = disksize - diskheadersize - headerspace;

        let fs = FreeSlot { offset, size };
        slots.push(fs);
        FreeList { slots }
    }
}
