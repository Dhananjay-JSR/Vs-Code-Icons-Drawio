use std::{fs::DirEntry, vec};
use git2::Repository;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SVGFile {
    data:String,
    w: i32,
    h: i32,
    title:String,
    aspect:String
}

#[derive(Serialize, Deserialize, Debug)]
struct FileStruct{
    data:Vec<SVGFile>
}

fn main() {
    let mut accepted_image_file :Vec<DirEntry> = vec![];
    let mut  MxLIb:Vec<SVGFile> = vec![];
    const REPO_URL: &'static str = "https://github.com/vscode-icons/vscode-icons";
    const INPUT_FDR: &'static str = "input";
    let folder_path = std::path::Path::new(INPUT_FDR);
    if folder_path.is_dir(){
        // std::fs::remove_dir_all(folder_path).unwrap_or_else(|err|{
        //     // println!("Some Error Occured")
        //     println!("{}",err)
        // });
        // println!("Removing Folder");
        // std::fs::create_dir(folder_path).unwrap_or_else(|_|{
        //     println!("Error Occured in Creating Folder")
        // });
        println!("Folder Creation Successful");



    }else {
        println!("Folder Doesn't Exits");
        std::fs::create_dir(folder_path).unwrap_or_else(|_|{
            println!("Error Occured in Creating Folder")
        });
        println!("Folder Creation Successful");
    }
    println!("Cloning File");
    // match Repository::clone(REPO_URL, folder_path){
    //     Ok(repo)=>{
    //         println!("Clone Complete");
    //         // for Files in FileIter{
        
        //             // }
        //         },
        //         Err(e)=>todo!()
        //     } 
        let FileIter: std::fs::ReadDir = std::fs::read_dir(folder_path.join("icons")).unwrap();
        for images_files in FileIter{
            match images_files {
                Ok(image)=>{
                    if !(image.file_name().to_str().unwrap().contains("folder_type")){
                        accepted_image_file.push(image)
                    }
                    
                }
                Err(Eer)=>{
                    println!("Error Occured {} in Extracting FIle",Eer)
                }
            }
            // let ImagesLoc = images_files.unwrap_or_else(|_|{
            //     println!("Images")
            // })
    // std::fs::read_dir(path)
    // println!("{}",REPO_URL)
    // match Repository::clone(REPO_URL, os::)
}

println!("{}",accepted_image_file.len());

for accepted_file in accepted_image_file{
    let Data = std::fs::read_to_string(accepted_file.path()).unwrap();
    let mut fileEcoded = "data:image/svg+xml;base64,".to_owned();
    // println!("{}",Data)
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(Data);
    fileEcoded.push_str(&encoded);
    let DataNew = accepted_file.file_name();
    let NewFile = DataNew.to_str().unwrap().strip_prefix("file_type_").unwrap_or_else(||{
        DataNew.to_str().unwrap()
    });
    let NewFileName = NewFile.strip_suffix(".svg").unwrap();
    // println!("{:?}",NewFile);
    let SvgEntry: SVGFile = SVGFile{
        data:fileEcoded,
        w:32,
        h:32,
        title:NewFileName.to_string(),
        aspect:"fixed".to_string()
    };
    // let j = serde_json::to_string(&SvgEntry).unwrap();
    MxLIb.push(SvgEntry);
    // println!("{}",j);

    // Print, write to a file, or send to an HTTP server.
    // println!("{}", j);


}
let FinalData: String = serde_json::to_string(&MxLIb).unwrap();
let data = "Some data!";
// std::fs::write("dhan.json", FinalData.as_bytes()).unwrap();
}
