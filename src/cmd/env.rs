use std::fs;
use std::fs::File;
use std::path::Path;


/*fn get_epher_symbolic_link() -> Result<File, Err>{
    let file = File::open(EPHER_LINK_PATH) ?;
    let metadata = file.metadata() ?;
    fs::read_link(EPHER_LINK_PATH)?;
    if metadata.is_symlink() && metadata.is_dir()  { 
          
    };
 }*/