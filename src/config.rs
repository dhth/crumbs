use clap::ValueEnum;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum ThemeName {
    #[value(name = "catppuccin-mocha")]
    CatppuccinMocha,
    #[value(name = "dracula")]
    Dracula,
    #[value(name = "github-dark")]
    GithubDark,
    #[default]
    #[value(name = "gruvbox-dark")]
    GruvboxDark,
    #[value(name = "monokai-classic")]
    MonokaiClassic,
    #[value(name = "onedark")]
    OneDark,
    #[value(name = "rose-pine-moon")]
    RosePineMoon,
    #[value(name = "tokyonight")]
    Tokyonight,
    #[value(name = "xcode-dark")]
    XcodeDark,
}

impl ThemeName {
    pub(crate) const ALL: [Self; 9] = [
        Self::CatppuccinMocha,
        Self::Dracula,
        Self::GithubDark,
        Self::GruvboxDark,
        Self::MonokaiClassic,
        Self::OneDark,
        Self::RosePineMoon,
        Self::Tokyonight,
        Self::XcodeDark,
    ];

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::CatppuccinMocha => "catppuccin-mocha",
            Self::Dracula => "dracula",
            Self::GithubDark => "github-dark",
            Self::GruvboxDark => "gruvbox-dark",
            Self::MonokaiClassic => "monokai-classic",
            Self::OneDark => "onedark",
            Self::RosePineMoon => "rose-pine-moon",
            Self::Tokyonight => "tokyonight",
            Self::XcodeDark => "xcode-dark",
        }
    }

    pub(crate) fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|theme| *theme == self)
            .unwrap_or(0);
        let next_index = (index + 1) % Self::ALL.len();

        Self::ALL[next_index]
    }

    pub(crate) fn previous(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|theme| *theme == self)
            .unwrap_or(0);
        let previous_index = if index == 0 {
            Self::ALL.len() - 1
        } else {
            index - 1
        };

        Self::ALL[previous_index]
    }
}
