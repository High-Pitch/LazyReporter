# Lazy Reporter

I was bored and wanted to create a low power weather reporter to hang in my closet

## Description

Previously was designed to push notifications with a bash script within linux. I changed it to being a device, as I wanted to continue working on it.

## To do
 * Add username autodetect
 * Make case

## Getting Started

### Dependencies

* Rust
* Linux
* Raspberry Pi (Made on a Pi Zero 2 W)
* Display (Made for Inky pHAT)

### Installing

* Fresh linux install on the Pi
* Set up wifi to Pi
* Download the git repo and export everything into you file system 
* Rust
1. ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh ```
2. ```cd /rust/weather_reporter/```
3. ```cargo build --release```
4. move the executable to the home directory

### Executing program

* How to run the program
1. Enable permissions for the program
```sudo chmod 700 weather-reporter-app```
2. Run the data gatherer
```./weather-reporter-data.sh```
3. Run the app
```./weather-reporter.app```

## Help

* If you use a virtual environment ensure you assign the proper paths to the commands.
* If you don't name the device reporter. Change the hard links (to be fixed)

## Authors

### Me: High-Pitch

## Version History
* 1
    * Personal one in usage
    * Temporary case
* 0.4
    * Added Logging
    * Created automation
* 0.3
    * Added UI for screen
    * Screen Choosen
* 0.2
    * Converted to Rust
* 0.1
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

