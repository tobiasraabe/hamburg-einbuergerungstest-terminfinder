"""Contains the finder for appointments."""

import sys
import time
import webbrowser
from importlib.resources import files

import httpx
from bs4 import BeautifulSoup

_URL = "https://www.vhs-hamburg.de/deutsch/einbuergerungstest-1058?o=date_asc"


def main() -> int:
    """Start the finder."""
    while True:
        is_available = _available_appointments_exist()
        if is_available:
            print("Appointments are available! Opening the website...")  # noqa: T201
            webbrowser.open(_URL)
            if sys.platform == "win32":
                import winsound

                winsound.PlaySound(files("finder") / "alert.wav", 0)
            print("Exiting...")  # noqa: T201
            sys.exit(0)
        else:
            print("No appointments available. Retrying in 5 minutes...")  # noqa: T201
            time.sleep(300)


def _available_appointments_exist() -> bool:
    """Check if there are available appointments."""
    response = httpx.get(_URL)
    course_statuses = BeautifulSoup(response.text, "html.parser").find_all(
        "div", class_="course-meta__status-content"
    )
    return any("voll" not in course_status.text for course_status in course_statuses)
