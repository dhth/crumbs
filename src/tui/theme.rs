use crate::config::ThemeName;
use ratatui::style::Color;

pub(crate) const CATPPUCCIN_MOCHA: Theme = Theme {
    name: "catppuccin-mocha",
    background: Color::from_u32(0x1e1e2e),
    foreground: Color::from_u32(0xcdd6f4),
    muted: Color::from_u32(0xa6adc8),
    accent: Color::from_u32(0xf37799),
    accent_alt: Color::from_u32(0x74a8fc),
    success: Color::from_u32(0x89d88b),
    success_alt: Color::from_u32(0xa6e3a1),
    warning: Color::from_u32(0xebd391),
    error: Color::from_u32(0xf37799),
    info: Color::from_u32(0x94e2d5),
    neutral_1: Color::from_u32(0xbac2de),
    neutral_2: Color::from_u32(0x9399b2),
};

pub(crate) const DRACULA: Theme = Theme {
    name: "dracula",
    background: Color::from_u32(0x282a36),
    foreground: Color::from_u32(0xf8f8f2),
    muted: Color::from_u32(0x6272a4),
    accent: Color::from_u32(0xff6e6e),
    accent_alt: Color::from_u32(0xbd93f9),
    success: Color::from_u32(0x69ff94),
    success_alt: Color::from_u32(0x50fa7b),
    warning: Color::from_u32(0xf1fa8c),
    error: Color::from_u32(0xff5555),
    info: Color::from_u32(0x8be9fd),
    neutral_1: Color::from_u32(0xcaa9fa),
    neutral_2: Color::from_u32(0x7d8cc4),
};

pub(crate) const GITHUB_DARK: Theme = Theme {
    name: "github-dark",
    background: Color::from_u32(0x101216),
    foreground: Color::from_u32(0xc9d1d9),
    muted: Color::from_u32(0x8b949e),
    accent: Color::from_u32(0xf78166),
    accent_alt: Color::from_u32(0x6ca4f8),
    success: Color::from_u32(0x56d364),
    success_alt: Color::from_u32(0x82bc56),
    warning: Color::from_u32(0xe3b341),
    error: Color::from_u32(0xdb61a2),
    info: Color::from_u32(0x56a4bf),
    neutral_1: Color::from_u32(0xc9d1d9),
    neutral_2: Color::from_u32(0x8b949e),
};

pub(crate) const GRUVBOX_DARK: Theme = Theme {
    name: "gruvbox-dark",
    background: Color::from_u32(0x282828),
    foreground: Color::from_u32(0xebdbb2),
    muted: Color::from_u32(0x928374),
    accent: Color::from_u32(0xfabd2f),
    accent_alt: Color::from_u32(0x83a598),
    success: Color::from_u32(0xb8bb26),
    success_alt: Color::from_u32(0x8ec07c),
    warning: Color::from_u32(0xfe8019),
    error: Color::from_u32(0xfb4934),
    info: Color::from_u32(0x83a598),
    neutral_1: Color::from_u32(0xa89984),
    neutral_2: Color::from_u32(0x7c6f64),
};

pub(crate) const MONOKAI_CLASSIC: Theme = Theme {
    name: "monokai-classic",
    background: Color::from_u32(0x272822),
    foreground: Color::from_u32(0xfdfff1),
    muted: Color::from_u32(0x75715e),
    accent: Color::from_u32(0x66d9ef),
    accent_alt: Color::from_u32(0xae81ff),
    success: Color::from_u32(0xa6e22e),
    success_alt: Color::from_u32(0x92d26a),
    warning: Color::from_u32(0xe6db74),
    error: Color::from_u32(0xf92672),
    info: Color::from_u32(0x66d9ef),
    neutral_1: Color::from_u32(0xf2a2c1),
    neutral_2: Color::from_u32(0x939179),
};

pub(crate) const ONE_DARK: Theme = Theme {
    name: "onedark",
    background: Color::from_u32(0x282c34),
    foreground: Color::from_u32(0xabb2bf),
    muted: Color::from_u32(0x7f848e),
    accent: Color::from_u32(0x61afef),
    accent_alt: Color::from_u32(0xc678dd),
    success: Color::from_u32(0x98c379),
    success_alt: Color::from_u32(0x56b6c2),
    warning: Color::from_u32(0xe5c07b),
    error: Color::from_u32(0xe06c75),
    info: Color::from_u32(0x61afef),
    neutral_1: Color::from_u32(0xc4ccd8),
    neutral_2: Color::from_u32(0x9098a6),
};

pub(crate) const ROSE_PINE_MOON: Theme = Theme {
    name: "rose-pine-moon",
    background: Color::from_u32(0x232136),
    foreground: Color::from_u32(0xe0def4),
    muted: Color::from_u32(0x6e6a86),
    accent: Color::from_u32(0xc4a7e7),
    accent_alt: Color::from_u32(0x9ccfd8),
    success: Color::from_u32(0x9ccfd8),
    success_alt: Color::from_u32(0x88c0d0),
    warning: Color::from_u32(0xf6c177),
    error: Color::from_u32(0xeb6f92),
    info: Color::from_u32(0xc4a7e7),
    neutral_1: Color::from_u32(0xd8c3f0),
    neutral_2: Color::from_u32(0x908caa),
};

pub(crate) const TOKYONIGHT: Theme = Theme {
    name: "tokyonight",
    background: Color::from_u32(0x1a1b26),
    foreground: Color::from_u32(0xc0caf5),
    muted: Color::from_u32(0x6b73a0),
    accent: Color::from_u32(0x7aa2f7),
    accent_alt: Color::from_u32(0xbb9af7),
    success: Color::from_u32(0x9ece6a),
    success_alt: Color::from_u32(0x73daca),
    warning: Color::from_u32(0xe0af68),
    error: Color::from_u32(0xf7768e),
    info: Color::from_u32(0x7dcfff),
    neutral_1: Color::from_u32(0xa9b1d6),
    neutral_2: Color::from_u32(0x787c99),
};

pub(crate) const XCODE_DARK: Theme = Theme {
    name: "xcode-dark",
    background: Color::from_u32(0x292a30),
    foreground: Color::from_u32(0xdfdfe0),
    muted: Color::from_u32(0x7f8c98),
    accent: Color::from_u32(0x4eb0cc),
    accent_alt: Color::from_u32(0xb281eb),
    success: Color::from_u32(0x78c2b3),
    success_alt: Color::from_u32(0x93c278),
    warning: Color::from_u32(0xd9c97c),
    error: Color::from_u32(0xff7ab2),
    info: Color::from_u32(0x81b1eb),
    neutral_1: Color::from_u32(0x98b8f0),
    neutral_2: Color::from_u32(0x8b99a5),
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Theme {
    pub(crate) name: &'static str,
    pub(crate) background: Color,
    pub(crate) foreground: Color,
    pub(crate) muted: Color,
    pub(crate) accent: Color,
    pub(crate) accent_alt: Color,
    pub(crate) success: Color,
    pub(crate) success_alt: Color,
    pub(crate) warning: Color,
    pub(crate) error: Color,
    pub(crate) info: Color,
    pub(crate) neutral_1: Color,
    pub(crate) neutral_2: Color,
}

pub fn get_theme(name: ThemeName) -> Theme {
    match name {
        ThemeName::CatppuccinMocha => CATPPUCCIN_MOCHA,
        ThemeName::Dracula => DRACULA,
        ThemeName::GithubDark => GITHUB_DARK,
        ThemeName::GruvboxDark => GRUVBOX_DARK,
        ThemeName::MonokaiClassic => MONOKAI_CLASSIC,
        ThemeName::OneDark => ONE_DARK,
        ThemeName::RosePineMoon => ROSE_PINE_MOON,
        ThemeName::Tokyonight => TOKYONIGHT,
        ThemeName::XcodeDark => XCODE_DARK,
    }
}
