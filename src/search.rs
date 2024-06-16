use indicatif::ProgressBar;
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use std::error::Error;
use std::process;
use std::thread;
use std::time::Duration;

struct Appointment {
    is_full: bool,
    course_id: String,
}

pub fn search() {
    // Create a spinner for waiting.
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Fetching appointments...");

    loop {
        let documents = match get_documents() {
            Ok(documents) => documents,
            Err(error) => panic!("Error fetching appointments: {error:?}"),
        };
        let appointments = match parse_appointments(documents) {
            Ok(appointments) => appointments,
            Err(error) => panic!("Error parsing appointments: {error:?}"),
        };

        let appointment_available = appointments.iter().any(|app| !app.is_full);

        if appointment_available {
            pb.finish_with_message(format!(
                "Found {} appointments. Some are available! Opening the website...",
                appointments.len()
            ));

            let url = "https://www.vhs-hamburg.de/deutsch/einbuergerungstest-1058";
            if webbrowser::open(url).is_err() {
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

fn parse_appointments(documents: Vec<String>) -> Result<Vec<Appointment>, Box<dyn Error>> {
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

    Ok(appointments)
}
