#[derive(Default)]
pub struct ServerState {
    url: Option<tauri::Url>,
}

impl ServerState {
    pub fn set_url(&mut self, url: Option<tauri::Url>) {
        self.url = url
    }

    pub fn upload_url(&self) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("https").unwrap();
            url.set_path("upload");
            url
        })
    }

    pub fn ws_url(&self) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("wss").unwrap();
            url.set_path("ws");
            url
        })
    }

    pub fn remote_media(&self, remote_path: String) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("https").unwrap();
            url.set_path(format!("uploads/{}", remote_path).as_str());
            url
        })
    }
}
