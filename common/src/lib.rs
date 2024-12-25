use std::{fs::{File, OpenOptions}, io::{Cursor, Seek, BufReader, BufRead}, env};

use reqwest::{header::COOKIE};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn load_env_file() -> Option<std::path::PathBuf> {
    dotenv::from_filename("../.env").ok()
}

pub async fn aoc_fetch_input(file_name: &'static str, day: u8) -> Result<File> {
    let url = reqwest::Url::parse(&format!("https://adventofcode.com/2022/day/{day}/input"))?;

    load_env_file();
    let session_id = env::var("AOC_SESSION").expect("Downloading input.txt requires the AOC_SESSION env var to be set");

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let response = client.get(url).header(COOKIE, &format!("session={session_id}")).send().await?;

    let mut file = OpenOptions::new().read(true).write(true).create(true).open(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    
    Ok(file)
}

pub async fn aoc_get_file(file_name: &'static str, day: u8) -> Option<File> {
    let mut file = std::fs::File::open(file_name);

    if file.is_err() {
        file = Ok(aoc_fetch_input(file_name, day).await.expect(&format!("Could not fetch day {day}'s input")));
    }

    let file = file.unwrap_or_else(|err| panic!("Could not open file '{file_name}': {err}"));

    Some(file)
}

pub fn for_each_line<F>(file: &File, mut func: F)
where
    F: FnMut(usize, &str)
{
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).enumerate().for_each(|(i, line)| func(i, line.as_str()));
}
