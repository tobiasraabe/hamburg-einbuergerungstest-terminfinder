# A finder for Einbürgerungstest appointments at VHS in Hamburg

Appointments for citizenship tests (Einbürgerungstest) in Hamburg are hard to get
because many people want to become citizens under the new legislation and the limited
number of appointments for tests.

Most appointments in Hamburg are available at the
[website of the VHS (Volkshochschule) Hamburg](https://www.vhs-hamburg.de/deutsch/einbuergerungstest-1058?o=date_asc),
but they offer no reminder service that would notify you when new appointments are
available.

To overcome the problem of sitting in front of the screen 24/7 and constantly hitting F5
on the keyboard to refresh the page, here is a little program that is easy to install
and run.

How does it work? The program runs in the background and frequently checks the VHS
webpage. As soon as appointments are available it will open the VHS webpage in your
browser thereby interrupting whatever you are doing.

Follow the instructions for your operating system.

## Linux / macOS

### Installation

Open a terminal
([macOS](https://support.apple.com/de-de/guide/terminal/trmlb20c7888/mac),
[Linux](https://www.youtube.com/watch?v=dHjWNcYT9vo)).

Then, install the latest release on your computer by running the following command.

```console
curl -fsSL https://raw.githubusercontent.com/tobiasraabe/hamburg-einbuergerungstest-terminfinder/main/install/install.sh | bash
```

### Usage

Open the terminal in the folder where you installed the application. Then, run

```console
finder
```

The finder will check every five minutes if new appointments are available. If
appointments are available, it will open the webpage of VHS for the appointments.
Quickly book an appointment!

After the appointment is booked, close the terminal by clicking on the X button or pressing
Ctrl+C or Cmd+C.

Good luck with your citizenship test!

### Deinstallation

When you do not need the program anymore, delete it with

```console
rm finder
```

## Windows

### Installation

Open a terminal in a folder of your choice which is explained
[here](https://johnwargo.com/posts/2024/launch-windows-terminal/).

Then, install the latest release on your computer by running the following command.

```console
iwr -useb hhttps://raw.githubusercontent.com/tobiasraabe/hamburg-einbuergerungstest-terminfinder/main/install/install.ps1 | iex
```

### Usage

Open the terminal in the folder where you installed the application. Then, run

```console
finder.exe
```

The finder will check every five minutes if new appointments are available. If
appointments are available, it will open the webpage of VHS for the appointments.
Quickly book an appointment!

After the appointment is booked, close the terminal by clicking on the X button or pressing
Ctrl+C.

Good luck with your citizenship test!

### Deinstallation

When you do not need the program anymore, delete it with

```console
rm finder.exe
```
