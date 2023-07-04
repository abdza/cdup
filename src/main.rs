use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
struct Duplicates {
    hash: String,
    totalsize: u64,
    paths: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let mut file_hashes: HashMap<_, Vec<_>> = HashMap::new();
    // let mut duplicates = HashMap::new();

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        for entry in WalkDir::new(&args[1]).into_iter().filter_map(|e| e.ok()) {
            let filepath = entry.path().to_owned();
            println!("{:#?}", filepath);
            println!("{:#?}", entry.file_type().is_dir());
            if !entry.file_type().is_dir() {
                let mut file = File::open(filepath.clone())?;
                let mut hasher = Sha256::new();
                let _n = io::copy(&mut file, &mut hasher)?;
                let hash = hasher.finalize().clone();
                println!("hash:{:#?}", hash);

                match file_hashes.get(&hash) {
                    Some(file_hash) => {
                        println!("File hash here: {:#?}", file_hash);
                        // file_hash.to_owned().push(filepath.clone());
                        let new_arr = [
                            file_hash.to_owned(),
                            vec![filepath.clone().into_os_string().into_string().unwrap()],
                        ]
                        .concat();
                        println!("After push File hash here: {:#?}", file_hash);
                        // file_hashes.insert(hash,file_hash.to_vec());
                        file_hashes.insert(hash, new_arr);
                    }
                    None => {
                        file_hashes.insert(
                            hash,
                            vec![filepath.clone().into_os_string().into_string().unwrap()],
                        );
                    }
                }
                println!("File: {} got sha256: {:x}", filepath.display(), hash);
            }
        }
    }
    let output_path = "duplicates.json";
    let vec_dup = file_hashes
        .into_iter()
        .filter(|(_key, value)| value.len() > 1)
        .map(|(key, value)| {
            let hash_string: String = format!("{:x}", key);
            Duplicates {
                hash: hash_string,
                totalsize: (fs::metadata(value.first().unwrap()).unwrap().len())
                    * value.len() as u64,
                paths: value,
            }
        })
        .collect::<Vec<Duplicates>>();
    std::fs::write(output_path, serde_json::to_string_pretty(&vec_dup).unwrap()).unwrap();
    // for (hash,filepaths) in file_hashes {
    //     if filepaths.len()> 1 {
    //         println!("Duplicates File: {:#?} got sha256: {:x}", filepaths, hash);
    //         let hash_string : String = format!("{:x}",hash);
    //         let outdata = Duplicates{ hash: hash_string, paths: filepaths};
    //         // println!("{}",&s);
    //         std::fs::write(
    //             output_path,
    //             serde_json::to_string_pretty(&outdata).unwrap(),
    //         ).unwrap();
    //     }
    // }
    Ok(())
}
