# Image Viewer

This is a minimal image viewer desktop native app built with the following technology:

![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white) ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![TailwindCSS](https://img.shields.io/badge/tailwindcss-%2338B2AC.svg?style=for-the-badge&logo=tailwind-css&logoColor=white) ![Svelte](https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white) ![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF)

## User Stories

### ✅ As a user, I would like to browse my computer for an image file

- ✅ opening the app should present the user with a file browse input field
- ✅ clicking on the file browse button should display a native file selection dialogue box
- ✅ choosing an image file and confirming the selected file should close the dialogue box
- ✅ the file browse input field should display the selected file path

### ✅ As a user, I would like to view the image file selected

- ✅ after choosing an image file using the file browse feature, a "preview" button should appear
- ✅ clicking the preview button should route the user to display the image selected
- ✅ from the preview route, clicking on the image should route the user to the main page

## Architecture

Being minimal, the architecture is also minimal.

- The main directory represents a `SvelteKit` project + Rust `Cargo` _workspace_.
- The `Svelte` _frontend_ is found within the `src` directory.
- The `Tauri` _backend_ is found within the `src-tauri` directory.
- `Svelte` components live under the `src/lib/components` directory.
- `SvelteKit` routes live under the `src/routes` directory.
- `Rust` crate modules live under the `src-tauri/src` directory.
- `src-tauri/src/main.rs` is the `Rust` entry-point of the `Tauri` application.

## Application Features

The _User Stories_ previously mentioned are the specifications by which the application has been designed.

- Browse for an image file on the user's filesystem.
- Preview the selected image in the app.

## Development

While this is a minimal app, you are free to fork the repository and extend/enhance things however you like.

You can build more on the frontend by working with `TypeScript`, `TailwindCSS` and `Svelte` in the `src` directory.

You can build more on the backend by working with `Rust`, and `Tauri` in the `src-tauri` directory.

#### Prerequisites

- Install Rust [See Official Rust Installation Guide](https://www.rust-lang.org/tools/install)
- Follow the Tauri Setup [See Official Tauri Getting Started Guide](https://tauri.app/v1/guides/getting-started/prerequisites)

```bash
pnpm dev
```

## Building

```bash
pnpm build
```

## Production

This project is not production-ready in that application binary code signing and icons, manifests, etc... have not been prepared. In order to take this project to production, there are several steps needed and are outside of the scope of this document.

## License

© 2023 SYS 64738 CONSULTING SERVICES LLC, Richard Marks

See [LICENSE](./LICENSE.md) for details
