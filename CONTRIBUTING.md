# Contributing to Spotify Sync CLI

First off, thank you for considering contributing to Spotify Sync CLI! It's people like you that make this tool better for everyone.

## License and Copyright

This project is licensed under the GNU General Public License v3.0 (GPLv3). By contributing to this project, you agree that:

1. You grant us the right to use your contributions under the GPLv3 license
2. You have the right to grant us these rights (i.e., you own the code you're contributing)
3. You understand that your contributions will be publicly available under the GPLv3

## Code Requirements

1. All new files must include the GPLv3 copyright header
2. All modifications to existing files must preserve the copyright header
3. Your code must be well-documented and follow Rust best practices
4. Include tests for new functionality

## Development Process

1. Fork the repository
2. Create your feature branch:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. Make your changes
4. Run the full test suite:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --all -- --check
   ```
5. Update documentation if needed
6. Commit your changes:
   ```bash
   git commit -m 'Add amazing feature'
   ```
7. Push to your branch:
   ```bash
   git push origin feature/amazing-feature
   ```
8. Open a Pull Request

## Pull Request Requirements

1. Clear description of changes
2. Tests for new functionality
3. Documentation updates if needed
4. GPLv3 headers in new files
5. All tests passing
6. Code formatted with `cargo fmt`
7. No `clippy` warnings

## Development Setup

1. Install Rust and Cargo
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/spotify-sync-CLI
   cd spotify-sync-CLI
   ```
3. Set up pre-commit hooks:
   ```bash
   git config core.hooksPath .githooks
   chmod +x .githooks/*
   ```
4. Create your `.env` file:
   ```bash
   cp .env.example .env
   # Edit .env with your Spotify API credentials
   ```

## Code Style

- Follow the Rust API guidelines
- Use meaningful variable names
- Document public APIs
- Keep functions focused and small
- Use error handling appropriately
- Format code with `cargo fmt`

## Questions?

Feel free to:
- Open an issue for discussion
- Ask in the Pull Request
- Contact the maintainers

## License

By contributing, you agree that your contributions will be licensed under the GNU General Public License v3.0.