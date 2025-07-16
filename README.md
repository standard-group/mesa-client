![Start background image for GitHub's README](public/startbgforgithub.jpg)

<div align="center">
    <img alt="Desktop Build Mesa (nightly)" src="https://github.com/standard-group/mesa-client/actions/workflows/desktop.yml/badge.svg?branch=main" href="https://github.com/standard-group/mesa-client/actions/workflows/desktop.yml">
    <img alt="Mobile Build Mesa (nightly)" src="https://github.com/standard-group/mesa-client/actions/workflows/mobile.yml/badge.svg?branch=main" href="https://github.com/standard-group/mesa-client/actions/workflows/mobile.yml">
    <img alt="Matrix" src="https://img.shields.io/matrix/project-mesa-room%3Amatrix.org?style=flat&logo=matrix" href="https://matrix.to/#/#project-mesa-room:matrix.org">
    <img alt="Discord" src="https://img.shields.io/badge/Discord-7289DA?style=flat&logo=discord&logoColor=white" href="https://sdiscord.gg/Z997UBvFJ4">
</div>

# Project Mesa Client

This is the client-side code for the Project Mesa desktop application. It is built using Vue.js + Vite and Tauri.

> Why you choose Tauri?

Tauri offers almos the same functionality as Electron, but with a more modern, faster and low-memory eater. It also has a better developer experience, as it is written in Rust and uses the WebView2 API for rendering.

## Getting Started

To install and run the client, go grab our latest release from [here](https://github.com/standard-group/mesa-client/releases/latest) for your platform. 

If you want nightly builds, you can find them [here for desktop](https://github.com/standard-group/mesa-client/actions/workflows/desktop.yml) and [here for mobile (Android)](https://github.com/standard-group/mesa-client/actions/workflows/mobile.yml).

## Contributing

We welcome contributions to the project. If you want to contribute, please follow steps in [CONTRIBUTING.md](CONTRIBUTING.md).

## Project Structure

The project is structured as follows:

- `src-tauri`: This directory contains the Tauri configuration files, including the `tauri.conf.json` file, which is used to configure the Tauri application.
- `src`: This directory contains the source code for the client-side application. (frontend in Vue)

## Building

Please read [BUILDING.md](BUILDING.md) for more information.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Donate

[![Donate using DonationAlerts](public/donate.svg)](https://www.donationalerts.com/r/standardgroup)

> *why our donations via donationalerts?*

cuz we are two dumb teenagers and we are from different countries, so we cant send money to each other. donationalerts is only a one way for us.

*donations are appreciated, but not required.*