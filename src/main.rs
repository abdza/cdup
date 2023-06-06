use std::fs::File;
use std::io;
use sha2::{Sha256, Digest};
use std::env;
use walkdir::WalkDir;
use std::collections::HashMap;


fn main() -> std::io::Result<()> {
    let mut file_hashes : HashMap<_,Vec<_>>= HashMap::new();
    // let mut duplicates = HashMap::new();

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        for entry in WalkDir::new(&args[1]).into_iter().filter_map(|e| e.ok()) {
            let filepath = entry.path().to_owned();
            println!("{:#?}",filepath);
            println!("{:#?}",entry.file_type().is_dir());
            if !entry.file_type().is_dir() {
                let mut file = File::open(filepath.clone())?;
                let mut hasher = Sha256::new();
                let _n = io::copy(&mut file, &mut hasher)?;
                let hash = hasher.finalize().clone();
                println!("hash:{:#?}",hash);

                match file_hashes.get(&hash) {
                    Some(file_hash) => {
                        println!("File hash here: {:#?}",file_hash);
                        // file_hash.to_owned().push(filepath.clone());
                        let new_arr = [file_hash.to_owned(), vec![filepath.clone()]].concat();
                        println!("After push File hash here: {:#?}",file_hash);
                        // file_hashes.insert(hash,file_hash.to_vec());
                        file_hashes.insert(hash,new_arr);
                    },
                    None => {
                        file_hashes.insert(hash,vec![filepath.clone()]);
                    }
                }
                println!("File: {} got sha256: {:x}", filepath.display(), hash);
            }
        }
    }
    for (hash,filepaths) in file_hashes {
        if filepaths.len()> 1 {
            println!("Duplicates File: {:#?} got sha256: {:x}", filepaths, hash);
        }
    }
    Ok(())
}

