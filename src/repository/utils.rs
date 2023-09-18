pub fn get_file_path(id: &str, ext: &str) -> String {
    format!("./media/{0}/{0}.{1}", id, ext)
}
