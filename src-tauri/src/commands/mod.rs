use polars::prelude::*;

#[tauri::command]
pub fn upload_file(path: &str) {
    println!("@celled");
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(path.into()))
        .unwrap()
        .finish()
        .unwrap();

    println!("{}", df);



    println!("{}", path)
}