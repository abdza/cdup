use std::fs::File;
use std::io;
use sha2::{Sha256, Digest};
use std::env;
use walkdir::WalkDir;
// use std::collections::HashMap;


fn main() -> std::io::Result<()> {
    // let mut file_hashes = HashMap::new();
    // let mut duplicates = HashMap::new();

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        for entry in WalkDir::new(&args[1]).into_iter().filter_map(|e| e.ok()) {
            let filepath = entry.path();
            println!("{:#?}",filepath);
            println!("{:#?}",entry.file_type().is_dir());
            if !entry.file_type().is_dir() {
                let mut file = File::open(filepath)?;
                let mut hasher = Sha256::new();
                let _n = io::copy(&mut file, &mut hasher)?;
                let hash = hasher.finalize().clone();
                println!("hash:{:#?}",hash);

                // match file_hashes.get(&hash) {
                //     Some(_file_hash) => {
                //         duplicates.insert(hash,filepath);
                //     },
                //     None => {
                //         file_hashes.insert(hash,filepath);
                //     }
                // }
                println!("File: {} got sha256: {:x}", filepath.display(), hash);
            }
        }
    }
    Ok(())
}

