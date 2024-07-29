use polars::prelude::*;
use serde::Serialize;
use crate::commands::processor;

#[derive(Serialize)]
pub struct Response {
    success: bool,
    message: String,
}

#[tauri::command]
pub async fn upload_file(path: &str) -> Result<Response, Response> {
    let lf: LazyFrame = LazyCsvReader::new(path)
        .finish()
        .unwrap();

    let mut conn = crate::database::establish_connection();

    match processor::create_teachers(&lf, &mut conn).await {
        Ok(message) => {
            println!("{}", message);
        }
        Err(e) => {
            Err(Response {
                success: false,
                message: e.to_string(),
            })?
        }
    }
    match processor::create_divisions(&lf, &mut conn).await {
        Ok(message) => {
            println!("{}", message);
        }
        Err(e) => {
            Err(Response {
                success: false,
                message: e.to_string(),
            })?
        }
    }
    match processor::create_subjects(&lf, &mut conn).await {
        Ok(message) => {
            println!("{}", message);
        }
        Err(e) => {
            Err(Response {
                success: false,
                message: e.to_string(),
            })?
        }
    }
    match processor::create_specialities(&lf, &mut conn).await {
        Ok(message) => {
            println!("{}", message);
        }
        Err(e) => {
            Err(Response {
                success: false,
                message: e.to_string(),
            })?
        }
    }
    match processor::create_students(&lf, &mut conn).await {
        Ok(message) => {
            println!("{}", message);
        }
        Err(e) => {
            Err(Response {
                success: false,
                message: e.to_string(),
            })?
        }
    }
    match processor::create_grades(&lf, &mut conn).await {
        Ok(message) => {
            println!("{}", message);
        }
        Err(e) => {
            Err(Response {
                success: false,
                message: e.to_string(),
            })?
        }
    }


    Ok(Response {
        success: true,
        message: "File uploaded successfully".to_string(),
    })
}
