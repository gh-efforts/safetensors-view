fn main() {
    let mut args = std::env::args();
    args.next();
    let fpath = args.next().expect("requires safetensors path");
    let f = std::fs::File::open(&fpath).expect("file not found");
    let f = unsafe {memmap2::Mmap::map(&f).expect("failed to map file")};

    let (_header_size, metadata) = safetensors::SafeTensors::read_metadata(&f).expect("failed to read metadata");
    let mut outf = std::fs::File::create("output.json").expect("failed to create output file");
    serde_json::to_writer_pretty(&mut outf, &metadata).expect("failed to serialize metadata");
}
