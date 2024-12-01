use std::path::Path;

pub struct Input {
    session_cookie: Option<String>,
    day: u8,
    data: Option<String>,
}

impl Input {
    pub fn new(day: u8) -> Self {
        Self {
            session_cookie: Self::get_cookie(),
            day,
            data: Self::fech_from_file(day),
        }
    }

    fn get_url(&self) -> String {
        format!("https://adventofcode.com/2024/day/{}/input", self.day)
    }

    pub async fn get_data(&mut self) -> Option<String> {
        self.session_cookie.as_ref()?;
        if self.data.is_none() {
            let url = self.get_url();
            let client = reqwest::Client::new();
            let response = client
                .get(&url)
                .header(
                    "Cookie",
                    format!("session={}", self.session_cookie.as_ref().unwrap()),
                )
                .send()
                .await
                .expect("Request failed");
            self.data = Some(response.text().await.expect("Failed to read input"));
        }

        self.save_to_file();

        self.data.clone()
    }

    fn get_cookie() -> Option<String> {
        if std::env::var("AOC_SESSION").is_err() && Path::exists(Path::new("./.env")) {
            let contents = std::fs::read_to_string("./.env").expect("Failed to read .env");
            for line in contents.lines() {
                if line.starts_with("AOC_SESSION=") {
                    return Some(line.trim_start_matches("AOC_SESSION=").to_string());
                }
            }
            return None;
        }

        std::env::var("AOC_SESSION").ok()
    }

    fn fech_from_file(day: u8) -> Option<String> {
        let path_str = format!("./inputs/day{}.txt", &day);
        let path = Path::new(&path_str);
        if path.exists() {
            return Some(std::fs::read_to_string(path).expect("Failed to read input"));
        }
        None
    }

    fn save_to_file(&self) {
        if self.data.is_none() {
            println!(
                "No data to save for day {} (probably because you haven't fetched it yet)",
                &self.day
            );
            return;
        }
        let path_str = format!("./inputs/day{}.txt", &self.day);
        let path = Path::new(&path_str);
        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, self.data.as_ref().unwrap()).unwrap();
            println!("Saved input for day {} to file", &self.day);
        }
    }
}

pub async fn get_data(day: u8) -> Option<String> {
    let mut input = Input::new(day);
    input.get_data().await
}
