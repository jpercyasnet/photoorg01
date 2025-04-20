use std::path::Path;
use std::fs;
use rfd::FileDialog;

pub fn todirpressm(dirval: String) -> (u32, String, String) { 
     let errcode: u32;
     let errstring: String;
     let mut new_dir: String;
     if Path::new(&dirval).exists() {
         new_dir = dirval.to_string();
     } else {
         new_dir = "/".to_string();
     }
     let folder = FileDialog::new()
//        .set_location(&new_dir)
//        .show_open_single_dir()
//        .unwrap();
         .set_directory(&new_dir)
         .pick_folder();
     if folder == None {
         errstring = "error getting directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         let mut bolok = true;
         for entry1 in fs::read_dir(&new_dir).unwrap() {
              let entry = entry1.unwrap();
              if let Ok(metadata) = entry.metadata() {
                  if let Ok(_file_name) = entry.file_name().into_string() {
                      if metadata.is_file() {
                          bolok = false;
                      }
                  }
              }
         }
         if bolok {
             errstring = "got to directory".to_string();
             errcode = 0;
         } else {
             errstring = "the to directory has files in it".to_string();
             errcode = 2;
         }
     }
     (errcode, errstring, new_dir)
}

