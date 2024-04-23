# A finder for Einbürgerungstest appointments at VHS in Hamburg

Appointments for citizenship tests (Einbürgerunstest) in Hamburg are hard to get since
many people want to become citizens due to the new legislation and the limited number of
appointments available.

Most appointments are available at the website of the VHS (Volkshochschule) Hamburg, but
they offer no reminder service that would notify you when new appointments are
available.

To overcome the problem of hitting constantly F5 on the keyboard to refresh the page,
here is a little script that you can run on your PC that plays an alarm when new dates
are published.

Follow the instructions to set everything up.

## Installation

1. Install rye. Choose the suitable installer for your OS from https://rye-up.com/.

1. Download this repository.

1. Open a terminal in this folder.

1. Start the finder by executing the following command in the terminal.

   ```console
   finder
   ```

The finder will check every five minutes if new appointments are available. If an
appointment is available, it will play alarm music. Close the terminal by clicking on
the X button or press Ctrl+C or Cmd+C and quickly go to the
[webpage of VHS](https://www.vhs-hamburg.de/deutsch/einbuergerungstest-1058?o=date_asc)
to book your appointment.

Good luck with your citizenship test!
