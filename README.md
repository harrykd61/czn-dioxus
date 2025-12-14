# CZN Dioxus - Electronic Signatures Management

A Dioxus application for managing electronic signatures and certificates on Windows platforms.

## Features

- Browse and search Windows certificate store
- View certificate details (subject, serial number, issuer)
- Electronic signature operations
- Modern UI with Tailwind CSS
- Desktop application with Dioxus

## Installation

### Prerequisites

- Rust (latest stable version)
- Dioxus CLI tools

### Build from source

```bash
git clone https://github.com/yourusername/czn-dioxus.git
cd czn-dioxus
cargo build --release
```

### Run the application

```bash
cargo run --release
```

Or use Dioxus serve:

```bash
dx serve --platform desktop
```

## Usage

1. Launch the application
2. The app will automatically load certificates from your Windows certificate store
3. Use the search box to filter certificates
4. Click on a certificate to view details and perform signature operations

## Configuration

Edit `Dioxus.toml` to customize application settings:

```toml
[application]
[bundle]
identifier = "com.chzn.dioxus"
publisher = "xCompany"
name = "CZN Dioxus Signer"
version = "1.0.0"
```

## Development

### Automatic Tailwind

The project uses automatic Tailwind CSS support in Dioxus 0.7+. Simply run:

```bash
dx serve
```

### Manual Tailwind Setup

For advanced Tailwind customization:

1. Install Node.js and npm
2. Install Tailwind CLI: `npm install -g tailwindcss`
3. Run: `npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch`

## Dependencies

- Dioxus 0.7.1 (desktop + router features)
- Windows API for certificate access
- Chrono for date/time handling
- RFD for file dialogs
- Hex encoding utilities

## License

MIT License

## Contributing

Contributions are welcome! Please open issues and pull requests on GitHub.

## Support

For support, please contact: d_in@rambler.ru
