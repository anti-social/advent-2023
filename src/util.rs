use std::env::var;

use anyhow::Context;

use dotenvy::dotenv;

pub fn day_from_filename(filename: &str) -> Result<u8, anyhow::Error> {
    let day = filename
        .strip_suffix(".rs").context("Expected .rs file extension")?
        .rsplit_once('_').context("Expected _ in filename")?
        .1
        .parse()?;
    Ok(day)
}

pub fn get_aoc_session() -> Result<String, anyhow::Error> {
    dotenv()?;
    if let Ok(aoc_session) = var("AOC_SESSION") {
        Ok(aoc_session)
    } else {
        anyhow::bail!("Missing AOC_SESSION environment variable")
    }
}

pub fn fetch_user_input(day: u8) -> Result<Option<String>, anyhow::Error> {
    let aoc_session = if let Ok(sess) = get_aoc_session() {
        sess
    } else {
        return Ok(None);
    };

    let client = reqwest::blocking::Client::new();
    let input = client.get(format!("https://adventofcode.com/2023/day/{day}/input"))
        .header(reqwest::header::COOKIE, format!("session={aoc_session}"))
        .send()?
        .text()?;
    if input.is_empty() {
        anyhow::bail!("Empty user input");
    }
    Ok(Some(input))
}
