use std::fs;
use std::io;

use crate::utils::file_check::file_check;
pub fn read_directory() -> Result<Vec<String>, io::Error> {
    // Pull everything in cucrent pwd into a vector of strings
    let paths: Result<fs::ReadDir, io::Error> = fs::read_dir("./");
    let mut valid_files: Vec<String> = Vec::new();
    if paths.is_ok() {
        let files: fs::ReadDir = paths.unwrap();
        for file in files {
            let f = file?;
            let dir: bool = f.file_type().unwrap().is_dir();
            let mut name = f.path().to_str().unwrap().to_string();
            if dir {
                let mut tmp = name.to_string();
                tmp.push_str("/");
                name = tmp;
            }
            let res = file_check(name);
            match res {
                Some(res) => {
                    valid_files.push(res);
                }
                None => {}
            }
        }
    }
    // let paths: Vec<String> = fs::read_dir("./")?
    //     .map(|res| res.map(|e| file_check(e.path().to_string_lossy().into_owned())))
    //     .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(valid_files)
}
