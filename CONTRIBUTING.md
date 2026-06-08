# Contributing to Conservation Composer

Thank you for your interest! Here's how to get started.

## Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/SuperInstance/conservation-composer.git
   cd conservation-composer
   ```

2. **Open locally**
   Simply open `index.html` in your browser, or serve it:
   ```bash
   python -m http.server 8000
   ```

3. **No build step required** — this is a single-file HTML application with zero dependencies.

## How to Contribute

### Reporting Issues
- Open a GitHub issue with a clear description
- Include browser version and OS
- Screenshots or screen recordings are helpful

### Submitting Changes
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes to `index.html`
4. Test in at least Chrome and Firefox
5. Commit with a descriptive message
6. Open a Pull Request

### Code Style
- Vanilla HTML/CSS/JavaScript (no frameworks)
- Keep everything in `index.html` for simplicity
- Use CSS custom properties for theming
- Comment complex math (eigenvalue computation, spectral analysis)

### Areas for Contribution
- New preset conservation patterns
- Improved spectral visualizations
- Mobile/touch support
- Accessibility improvements
- Performance optimizations for large graphs
