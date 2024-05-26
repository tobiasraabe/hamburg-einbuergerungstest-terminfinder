use indicatif::ProgressBar;
use scraper::{Html, Selector};
use std::process;
use std::thread;
use std::time::Duration;

struct Appointment {
    is_full: bool,
}

fn main() {
    let url = "https://www.vhs-hamburg.de/deutsch/einbuergerungstest-1058";

    // Create a spinner for waiting.
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Fetching appointments...");

    loop {
        let appointments = match get_appointments(url) {
            Ok(appointments) => appointments,
            Err(e) => {
                eprintln!("Error finding appointments: {}", e);
                process::exit(1);
            }
        };

        let appointment_available = appointments.iter().any(|app| !app.is_full);

        match appointment_available {
            true => {
                pb.finish_with_message(format!(
                    "Found {} appointments. Some are available! Opening the website...",
                    appointments.len()
                ));
                webbrowser::open(url).unwrap();
                println!("Exiting...");
                process::exit(0);
            }
            false => {
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
}

fn get_appointments(url: &str) -> Result<Vec<Appointment>, Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(url)?;
    let body = res.text()?;
    let document = Html::parse_document(&body);

    let selector = Selector::parse(".course-meta__status-content").unwrap();

    let appointments = document
        .select(&selector)
        .map(|node| {
            let is_full = node.text().collect::<String>().contains("voll");
            Appointment { is_full }
        })
        .collect::<Vec<_>>();
    Ok(appointments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_appointments() {
        let mut server = mockito::Server::new();
        server
            .mock("GET", "/")
            .with_status(200)
            .with_body("<div class='course-meta__status-content'>voll</div>")
            .create();
        let appointments = get_appointments(&server.url());
        assert!(appointments.unwrap()[0].is_full);
    }

    #[test]
    fn test_get_appointments_with_available_appointment() {
        let mut server = mockito::Server::new();
        server
            .mock("GET", "/")
            .with_status(200)
            .with_body("<div class='course-meta__status-content'>verfügbar</div>")
            .create();
        let appointments = get_appointments(&server.url());
        assert!(!appointments.unwrap()[0].is_full);
    }
}
