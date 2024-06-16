use indicatif::ProgressBar;
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use std::error::Error;
use std::process;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Appointment {
    is_full: bool,
    course_id: String,
}

/// Searches for available appointments and prints them.
///
/// This function fetches documents containing appointment information in a loop.
/// It parses the appointments from the documents and filters out the full ones.
/// The number of available courses is then printed.
/// If there are available courses, the function finishes the progress bar,
/// prints a message, and attempts to open the website.
///
/// # Panics
///
/// This function will panic if there is an error fetching or parsing the appointments.
pub fn search() {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Fetching appointments...");

    loop {
        let documents = match get_documents() {
            Ok(documents) => documents,
            Err(error) => panic!("Error fetching appointments: {error:?}"),
        };
        let appointments = parse_appointments(documents);

        let available_courses: Vec<_> = appointments.iter().filter(|app| !app.is_full).collect();
        let n_available_courses = available_courses.len();

        if n_available_courses > 0 {
            pb.finish_and_clear();
            println!(
                "{n_available_courses} of {} courses are available! Opening the website...",
                appointments.len()
            );

            let url = if n_available_courses == 1 {
                format!(
                    "https://www.vhs-hamburg.de/kurs/einburgerungstest/{}",
                    available_courses[0].course_id
                )
            } else {
                "https://www.vhs-hamburg.de/deutsch/einbuergerungstest-1058".to_string()
            };

            if webbrowser::open(&url).is_err() {
                println!("Failed to open website. Please open {url} manually.");
            }
            println!("Exiting...");
            process::exit(0);
        } else {
            for i in (1..=300).rev() {
                let minutes = i / 60;
                let seconds = i % 60;
                pb.set_message(format!(
                    "Found {} appointments. None are available. Retrying in {}m{}s.",
                    appointments.len(),
                    minutes,
                    seconds
                ));
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn get_documents() -> Result<Vec<String>, Box<dyn Error>> {
    let base_url = "https://www.vhs-hamburg.de/deutsch/einbuergerungstest-1058";
    let client = Client::new();
    let mut documents = Vec::new();
    let mut page = 1;

    loop {
        let url = Url::parse_with_params(base_url, &[("p", page.to_string())])?;

        let response = client.get(url.clone()).send()?;

        // Check if the request was redirected to the first page
        if response.url() != &url {
            break;
        }

        let text = response.text()?;
        documents.push(text);

        page += 1;
    }

    Ok(documents)
}

fn parse_appointments(documents: Vec<String>) -> Vec<Appointment> {
    let mut appointments = Vec::new();

    let course_card_selector = Selector::parse("article.course-card").unwrap();
    let course_link_selector = Selector::parse(".course-card__title a").unwrap();
    let course_status_selector = Selector::parse(".course-meta__status--danger span").unwrap();

    for document in documents {
        let html_doc = Html::parse_document(&document);

        for course_card in html_doc.select(&course_card_selector) {
            if let Some(course_link_element) = course_card.select(&course_link_selector).next() {
                if let Some(href) = course_link_element.value().attr("href") {
                    if let Some(course_id) = href.split('/').nth(3) {
                        let is_full = course_card.select(&course_status_selector).next().is_some();
                        appointments.push(Appointment {
                            is_full,
                            course_id: course_id.to_string(),
                        });
                    }
                }
            }
        }
    }

    appointments
}
