use std::{
    fs::{self},
    str::FromStr,
};

use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

use crate::repository::chatgpt::send_message;

use super::utils::get_file_path;

#[derive(Debug, Serialize)]
pub struct Folder {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
}

pub fn list_folders() -> Result<Json<Vec<Folder>>, ()> {
    let mut folders: Vec<Folder> = Vec::new();
    let paths = fs::read_dir("./media").unwrap();

    for path in paths {
        let path = path.unwrap();
        let _metadata = fs::metadata(path.path()).unwrap();
        let folder = Folder {
            id: path.file_name().into_string().unwrap(),
            file_url: None,
        };
        folders.push(folder);
    }

    Ok(Json(folders))
}

pub async fn save_file(file: TempFile) -> String {
    let file_name = file.file_name.clone().unwrap();
    let name_without_ext = file_name.split('.').collect::<Vec<&str>>()[0];
    let dir = format!("./media/{}", name_without_ext);
    let file_path = format!("{}/{}", dir, file_name);

    println!("dir: {}", dir);
    println!("file_path: {}", file_path);

    // create folder if not exists
    fs::create_dir_all(dir).unwrap();

    // copia file.file para o path

    match file.file.persist(&file_path) {
        Ok(_) => println!("file persistido com sucesso"),
        Err(e) => {
            println!("erro ao persistir file, err: {}", e);
            return String::from("");
        }
    }

    let pdf_content = match read_pdf_content(&file_path) {
        Ok(pdf_content) => pdf_content,
        Err(e) => {
            println!("erro ao ler pdf, err: {}", e);
            return String::from("");
        }
    };

    let pergunta = String::from(
        "esse é um extraido de um pdf, guarde ele, vou fazer perguntas a seguir: \n\n",
    ) + pdf_content.as_str();

    // let rt = tokio::runtime::Runtime::new().unwrap();

    send_message(pergunta, String::from_str(name_without_ext).unwrap()).await;

    file_path
}

pub fn get_file(id: String) -> Result<Json<Folder>, ()> {
    let file_url = get_file_path(&id, "pdf");

    let folder = Folder {
        id,
        file_url: Some(file_url),
    };

    Ok(Json(folder))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub role: String,
    pub content: String,
}

pub fn get_conversations(id: String) -> Vec<Conversation> {
    let conversations: Vec<Conversation> = Vec::new();

    let path_json = get_file_path(&id, "json");

    println!("path_json: {}", path_json);

    let json_data = match fs::read_to_string(path_json) {
        Ok(path_json) => path_json,
        Err(e) => {
            println!("erro ao ler json 11, err: {}", e);
            return conversations;
        }
    };

    println!("json_data: {}", json_data);

    let conversations: Vec<Conversation> = match serde_json::from_str(&json_data) {
        Ok(json_data) => json_data,
        Err(e) => {
            println!("erro ao ler json 11, err: {}", e);
            return conversations;
        }
    };

    conversations
}

pub fn read_pdf_content(path: &str) -> Result<String, String> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("erro ao ler pdf, err: {}", e);
            return Err(e.to_string());
        }
    };

    match pdf_extract::extract_text_from_mem(&bytes) {
        Ok(text) => Ok(text),
        Err(e) => Err(e.to_string()),
    }
}
