use git2::Repository;

fn main() {
    // let imageFile :Vec<str>;
    const REPO_URL: &'static str = "https://github.com/vscode-icons/vscode-icons";
    let folder_path = std::path::Path::new("input");
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

                        println!("{:?}",image.path())
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
}
