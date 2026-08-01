#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavDestination {
    Hash,
    PasswordHash,
    QrCode,
    Uuid,
    Url,
    Base64,
    Timestamp,
    NumberBase,
    Color,
    TextCase,
}

impl NavDestination {
    pub fn all() -> [Self; 10] {
        [
            Self::Hash,
            Self::PasswordHash,
            Self::QrCode,
            Self::Uuid,
            Self::Url,
            Self::Base64,
            Self::Timestamp,
            Self::NumberBase,
            Self::Color,
            Self::TextCase,
        ]
    }

    pub fn icon_path(&self) -> &'static str {
        match self {
            Self::Hash => "icons/hash.svg",
            Self::PasswordHash => "icons/lock-keyhole.svg",
            Self::QrCode => "icons/qr-code.svg",
            Self::Timestamp => "icons/clock.svg",
            Self::Url => "icons/link.svg",
            Self::NumberBase => "icons/binary.svg",
            Self::Uuid => "icons/id-card.svg",
            Self::Base64 => "icons/code.svg",
            Self::Color => "icons/palette.svg",
            Self::TextCase => "icons/case-sensitive.svg",
        }
    }

    pub fn locale_key<'a>(&self, l10n: &'a crate::locale::L10n) -> &'a str {
        match self {
            Self::Hash => l10n.nav_hash,
            Self::PasswordHash => l10n.nav_password_hash,
            Self::QrCode => l10n.nav_qrcode,
            Self::Timestamp => l10n.nav_timestamp,
            Self::Url => l10n.nav_url,
            Self::NumberBase => l10n.nav_number_base,
            Self::Uuid => l10n.nav_uuid,
            Self::Base64 => l10n.nav_base64,
            Self::Color => l10n.nav_color,
            Self::TextCase => l10n.nav_text_case,
        }
    }
}
