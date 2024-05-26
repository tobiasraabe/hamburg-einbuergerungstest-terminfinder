# A finder for Einbürgerungstest appointments at VHS in Hamburg

Appointments for citizenship tests (Einbürgerungstest) in Hamburg are hard to get
because many people want to become citizens under the new legislation and the limited
number of appointments for tests.

Most appointments in Hamburg are available at the
[website of the VHS (Volkshochschule) Hamburg](https://www.vhs-hamburg.de/deutsch/einbuergerungstest-1058?o=date_asc),
but they offer no reminder service that would notify you when new appointments are
available.

To overcome the problem of sitting in front of the screen 24/7 and constantly hitting F5
on the keyboard to refresh the page, here is a little script that you can run on your PC
that informs you about new appointments.

Follow the instructions to set everything up.

## Installation

> [!WARNING]
>
> The executable does not work properly on every system. Windows seems to work, Linux
> and MacOS probably not. I am working on it.

1. Go to the
   [release page](https://github.com/tobiasraabe/hamburg-einbuergerungstest-terminfinder/releases)
   and select the latest release.
2. Download the correct binary for your OS.
3. Run the binary.

The finder will check every five minutes if new appointments are available. If
appointments are available, it will open the webpage of VHS for the appointments.
Quickly book an appointment!

After the appointment is booked, close the terminal by clicking on the X button or press
Ctrl+C or Cmd+C.

Good luck with your citizenship test!
