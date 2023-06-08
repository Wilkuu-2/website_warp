use tokio::fs;
use std::path::PathBuf;

pub type SnippetWrapper = fn(id: usize, snip: &String) -> String; 

#[derive(Debug)]
pub struct SnippetPaths {
    pub paths: Vec<String>
}

impl SnippetPaths {
    pub async fn from_folder_async(path: &str, prealoc: usize, reverse: bool) -> SnippetPaths {
       let mut dir = fs::read_dir(PathBuf::from(path)).await.unwrap();
       let mut snip_paths: Vec<String> = Vec::with_capacity(prealoc);

       while let Some(file) = dir.next_entry().await.unwrap() {
           if file.file_type().await.unwrap().is_file() {
              let path = file.path();
              if let Some(extension) = path.extension() {
                if !"html".contains(extension.to_str().unwrap_or("invalid")) {continue;}
                snip_paths.push(path.to_string_lossy().to_string());
              }
           }
       }
       snip_paths.sort_unstable();

       if reverse {
            snip_paths.reverse();
       }
 
       SnippetPaths{paths: snip_paths}
    }

    pub fn _from_folder(path: &str, prealoc: usize, reverse: bool) -> SnippetPaths {
       let dir = std::fs::read_dir(PathBuf::from(path)).unwrap();
       let mut snip_paths: Vec<String> = Vec::with_capacity(prealoc);

       for file in dir{
           let file = file.unwrap();
           if file.file_type().unwrap().is_file() {
              let path = file.path();
              if let Some(extension) = path.extension() {
                if !"html".contains(extension.to_str().unwrap_or("invalid")) {continue;}
                snip_paths.push(path.to_string_lossy().to_string());
              }
           }
       }
       
       snip_paths.sort_unstable();

       if reverse {
           snip_paths.reverse();
       } 

       SnippetPaths { paths: snip_paths } 
    }
}

#[derive(Debug)]
pub struct Snippets{
    pub content: Vec<String>
}

impl Snippets {
    pub fn _with_capacity(cap: usize) -> Snippets{
        Snippets { content: Vec::with_capacity(cap)}
    }

    pub async fn _from_paths_async(paths: &SnippetPaths) -> Snippets {
       let mut snips = Snippets::_with_capacity(paths.paths.len());
       for path in &paths.paths {
           snips.content.push(fs::read_to_string(path).await.unwrap().to_string());
       }
       snips
    }

    pub async fn _from_folder_async(path: &str, len: usize) -> Snippets{
       let paths = SnippetPaths::from_folder_async(path,len,true).await;
       Snippets::_from_paths_async(&paths).await
    }
}

