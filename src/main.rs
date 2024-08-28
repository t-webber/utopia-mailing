use reqwest::blocking::get;
use std::fs;
use std::io::{copy, Read};

fn url2html(url: &str) -> Result<String, String> {
    match get(url) {
        Ok(mut response) => {
            if response.status().is_success() {
                let mut body = String::new();
                match response.read_to_string(&mut body) {
                    Err(err) => Err(format!("Error reading response body: {err}.")),
                    Ok(_) => Ok(body),
                }
            } else {
                Err(format!(
                    "Error: Request was not successful (status code: {}).",
                    response.status()
                ))
            }
        }
        Err(err) => Err(format!("Error making request: {err}.")),
    }
}

fn max_number_in_line(seq: &str) -> usize {
    let mut result = Vec::new();
    result.push(String::new());
    seq.chars().for_each(|ch| {
        if ch.is_ascii_digit() {
            match result.last_mut() {
                Some(last) => (*last).push(ch),
                None => result.push(ch.to_string()),
            }
        } else if result.last().map_or(0, |last| last.len()) > 0 {
            result.push(String::new());
        }
    });
    result
        .into_iter()
        .map(|s| {
            // println!(">>>>>>>>{}", s);
            s.parse().unwrap_or_default()
        })
        .max()
        .unwrap_or_default()
}

fn find_latest_line(body: String) -> (String, usize) {
    let mut max = 0;
    let mut max_line = "";
    body.lines().for_each(|line| {
        let digitalised = line
            .split("</a>")
            .next()
            .unwrap_or(line)
            .chars()
            .filter(|ch| !ch.is_alphabetic())
            .collect::<String>();
        let current = max_number_in_line(&digitalised);
        // println!("{}: {}", current, line);
        if current > max {
            max_line = line;
            max = current;
            // print!("max -> {}", max);
            // print!("max_line -> {}", max_line);
        }
    });
    // println!("max_line -> {}", max_line);
    // println!("max -> {}", max);
    (max_line.to_owned(), max)
}

fn max_line_to_pdf_url(url: &str, line: &str) -> String {
    let pdf = line
        .split("href=\"")
        .nth(1)
        .expect("No href found")
        .split("\">")
        .next()
        .expect("href not closed");
    format!("{url}{pdf}")
}

fn download_pdf(url: &str, city: &str, max: usize) -> std::io::Result<()> {
    let pdfname = format!("{city}_{max}.pdf");
    let response = get(url).expect("Failed to make request");

    if response.status().is_success() {
        let mut dest = fs::File::create(pdfname)?;
        let content = response.bytes().expect("Failed to read response bytes");
        copy(&mut content.as_ref(), &mut dest)?;
        // println!("The file has been downloaded successfully.");
    } else {
        // println!("Failed to download the file.");
    }

    Ok(())
}

fn city(url: &str, city: &str) {
    let body = url2html(url).unwrap();
    // print!("{}", body);
    let (max_line, max) = find_latest_line(body);
    let link = max_line_to_pdf_url(url, &max_line);
    // eprintln!("{link}");
    download_pdf(&link, city, max).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"avignon".to_owned()) {
        city(
            "https://www.cinemas-utopia.org/U-blog/avignon/public/GAZETTE_PDF/",
            "avignon",
        );
    };
    if args.contains(&"montpellier".to_owned()) {
        city(
            "https://www.cinemas-utopia.org/U-blog/montpellier/public/Gazettes/",
            "montpellier",
        );
    };
}
