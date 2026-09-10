fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        // Компилируем .rc файл, который внутри себя подтянет app.manifest
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
}
