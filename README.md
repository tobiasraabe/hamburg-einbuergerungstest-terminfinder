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

1. Install rye. Choose the suitable installer for your OS from https://rye-up.com/.

1. Download this repository.

1. Open a terminal in this folder.

1. Start the finder by executing the following command in the terminal.

   ```console
   finder
   ```

The finder will check every five minutes if new appointments are available. If
appointments are available, it will open the webpage of VHS for the appointments.
Quickly book an appointment!

After the appointment is booked, close the terminal by clicking on the X button or press
Ctrl+C or Cmd+C. Then, delete the folder and uninstall rye.

Good luck with your citizenship test!
