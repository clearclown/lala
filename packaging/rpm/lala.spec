Name:           lala
Version:        0.1.0
Release:        1%{?dist}
Summary:        Modern text editor with multi-format preview support

License:        MIT OR Apache-2.0
URL:            https://github.com/yourusername/lala
Source0:        https://github.com/yourusername/lala/archive/v%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  libxcb-devel
BuildRequires:  gcc

Requires:       libxcb

%description
Lala is a modern, lightweight text editor with both GUI and CLI interfaces.
It supports previewing Markdown, HTML, Mermaid diagrams, and LaTeX documents
with beautiful terminal formatting.

Features:
- File tree view with async loading
- Syntax highlighting for multiple languages
- Multi-format preview (Markdown, HTML, Mermaid, LaTeX)
- Advanced search and replace with regex
- Both CLI and GUI modes

%prep
%setup -q

%build
cargo build --release --locked

%install
rm -rf $RPM_BUILD_ROOT
install -D -m755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -D -m644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -D -m644 docs/en/install.md %{buildroot}%{_docdir}/%{name}/install.md
install -D -m644 docs/en/cli-usage.md %{buildroot}%{_docdir}/%{name}/cli-usage.md
install -D -m644 LICENSE-MIT %{buildroot}%{_licensedir}/%{name}/LICENSE-MIT
install -D -m644 LICENSE-APACHE %{buildroot}%{_licensedir}/%{name}/LICENSE-APACHE

%check
cargo test --release --locked

%files
%license LICENSE-MIT LICENSE-APACHE
%doc README.md docs/en/install.md docs/en/cli-usage.md
%{_bindir}/%{name}

%changelog
* Thu Nov 14 2025 Lala Contributors <your-email@example.com> - 0.1.0-1
- Initial RPM release
- Multi-format preview support (Markdown, HTML, Mermaid, LaTeX)
- GUI and CLI modes
- Syntax highlighting
- Advanced search and replace
