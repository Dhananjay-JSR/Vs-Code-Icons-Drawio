use std::{fs::DirEntry, vec, fmt::format};
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

fn main() {
    let mut accepted_image_file :Vec<DirEntry> = vec![];
    let mut  MxLIb:Vec<SVGFile> = vec![];
    const REPO_URL: &'static str = "https://github.com/vscode-icons/vscode-icons";
    const INPUT_FDR: &'static str = "input";
    const OUTPUT_FDR: &'static str = "output";
    const OUTPUT_FILE_NAME: &'static str = "Vs-Icon-By-Dhananjay.xml";
    let input_folder_path = std::path::Path::new(INPUT_FDR);
    let output_folder_path = std::path::Path::new(OUTPUT_FDR);
    
    if input_folder_path.is_dir(){
        std::fs::remove_dir_all(input_folder_path).unwrap_or_else(|err|{
            // println!("Some Error Occured")
            println!("{}",err)
        });
        println!("Removing Folder");
        std::fs::create_dir(input_folder_path).unwrap_or_else(|_|{
            println!("Error Occured in Creating Folder")
        });
        println!("Folder Creation Successful");



    }else {
        println!("Folder Doesn't Exits");
        std::fs::create_dir(input_folder_path).unwrap_or_else(|_|{
            println!("Error Occured in Creating Folder")
        });
        println!("Folder Creation Successful");
    }
    println!("Cloning File");
    match Repository::clone(REPO_URL, input_folder_path){
        Ok(repo)=>{
            println!("Clone Complete");
            // for Files in FileIter{
        
                    // }
                },
                Err(e)=>panic!("Unable to Clone Repo Panic !!!")
            } 
        let FileIter: std::fs::ReadDir = std::fs::read_dir(input_folder_path.join("icons")).unwrap();
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
           
}
println!("Generating Meta Data");
for accepted_file in accepted_image_file{
    let Data = std::fs::read_to_string(accepted_file.path()).unwrap();
    let mut fileEcoded = "data:image/svg+xml;base64,".to_owned();
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
    MxLIb.push(SvgEntry);

}
let FinalData: String = serde_json::to_string(&MxLIb).unwrap();
let OutPutData = format!("{}{}{}","<mxlibrary>",FinalData,"</mxlibrary>");

if !output_folder_path.is_dir(){
    std::fs::create_dir(output_folder_path).unwrap_or_else(|_|{
        println!("Error Occured Output Creating Folder")
    });
}else {
    std::fs::remove_dir_all(output_folder_path).unwrap_or_else(|err|{
        // println!("Some Error Occured")
        println!("{}",err)
    });
    println!("Removing Output Folder");
    std::fs::create_dir(output_folder_path).unwrap_or_else(|_|{
        println!("Error Occured in Creating Folder")
    });
}
println!("Output Folder Creation Successful");
std::fs::write(output_folder_path.join(OUTPUT_FILE_NAME), OutPutData.as_bytes()).unwrap();
println!("File Written to Disk Successfully")
}
