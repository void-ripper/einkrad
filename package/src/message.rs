pub enum Message {
    CreateScene(String),
    CreatedScene(u32),
    LoadDrawable(u32, String),
    LoadedDrawable(u32),
}
