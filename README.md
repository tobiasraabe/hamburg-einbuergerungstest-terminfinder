# A finder for Einbürgerungstest appointments at VHS in Hamburg

Appointments for citizenship tests (Einbürgerungstest) in Hamburg are hard to get
because many people want to become citizens under the new legislation and the limited
number of appointments for tests.

Most appointments in Hamburg are available at the website of the VHS (Volkshochschule)
Hamburg, but they offer no reminder service that would notify you when new appointments
are available.

To overcome the problem of sitting in front of the screen 24/7 and constantly hitting F5
on the keyboard to refresh the page, here is a little script that you can run on your PC
that informs you about new appointments.

Follow the instructions to set everything up.

## Installation

1. Install rye. Choose a suitable installer for your OS from https://rye-up.com/ and
   install the tool.

1. Download this repository and unzip it. ([Instructions](https://sites.northwestern.edu/researchcomputing/resources/downloading-from-github/))

1. Open a terminal in the unzipped folder so you can see the folder `src` and `tests`.
   (Instructions: [Windows](https://johnwargo.com/posts/2024/launch-windows-terminal/),
   [MacOS](https://support.apple.com/guide/terminal/open-new-terminal-windows-and-tabs-trmlb20c7888/mac#:~:text=On%20your%20Mac%2C%20open%20a,window%3A%20Choose%20Open%20in%20Terminal.),
   [Linux](https://www.youtube.com/watch?v=dHjWNcYT9vo))

2. Start the finder by executing the following commands in the terminal.

   ```console
   rye sync --all-features
   rye run finder
   ```

The finder will check every five minutes if new appointments are available. If
appointments are available, it will open the webpage of VHS for the appointments.
Quickly book an appointment!

After the appointment is booked, close the terminal by clicking on the X button or press
Ctrl+C or Cmd+C. Then, delete the folder and uninstall rye.

Good luck with your citizenship test!
