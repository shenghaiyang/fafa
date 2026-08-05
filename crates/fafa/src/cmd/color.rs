use anyhow::{Result, bail};

use crate::pretty::{print_footer, print_header, println_key_value, swatch};

#[derive(Clone, Copy)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: f64,
}

#[derive(clap::Args)]
pub struct Args {
    color: String,
}

const MAX_LEN: usize = 12;

pub fn run(args: Args) -> Result<()> {
    let rgba = parse(&args.color)?;
    let alpha_byte = (rgba.a * 255.0).round() as u8;
    let (h, s, l) = rgb_to_hsl(rgba.r, rgba.g, rgba.b);
    let color_swatch = swatch(rgba.r, rgba.g, rgba.b);
    let header = "Color";
    print_header(header);
    println_key_value(
        "hex",
        format!("#{:02X}{:02X}{:02X} {color_swatch}", rgba.r, rgba.g, rgba.b),
        MAX_LEN,
    );
    println_key_value(
        "rgba",
        format!(
            "rgba({}, {}, {}, {:.2}) {color_swatch}",
            rgba.r, rgba.g, rgba.b, rgba.a
        ),
        MAX_LEN,
    );
    println_key_value(
        "rgba hex",
        format!(
            "#{:02X}{:02X}{:02X}{:02X} {color_swatch}",
            rgba.r, rgba.g, rgba.b, alpha_byte
        ),
        MAX_LEN,
    );
    println_key_value(
        "argb hex",
        format!(
            "#{:02X}{:02X}{:02X}{:02X} {color_swatch}",
            alpha_byte, rgba.r, rgba.g, rgba.b
        ),
        MAX_LEN,
    );
    println_key_value(
        "hsla",
        format!(
            "hsla({h:.0}, {s:.0}%, {l:.0}%, {:.2}) {color_swatch}",
            rgba.a
        ),
        MAX_LEN,
    );
    print_footer(header.len());
    Ok(())
}

fn parse(input: &str) -> Result<Rgba> {
    let trimmed = input.trim();
    if let Some(hex) = trimmed.strip_prefix('#') {
        parse_hex(hex)
    } else if trimmed.starts_with("rgb") || trimmed.starts_with("RGB") {
        parse_rgb(trimmed)
    } else if trimmed.starts_with("hsl") || trimmed.starts_with("HSL") {
        parse_hsl(trimmed)
    } else {
        bail!("unsupported color format; expected #RRGGBB[AA], rgb()/rgba(), or hsl()/hsla()")
    }
}

fn parse_hex(s: &str) -> Result<Rgba> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() != 6 && chars.len() != 8 {
        bail!("hex color must have 6 or 8 hex digits");
    }
    let pair = |c1: char, c2: char| -> Result<u8> {
        let hi = c1
            .to_digit(16)
            .ok_or_else(|| anyhow::anyhow!("invalid hex digit"))?;
        let lo = c2
            .to_digit(16)
            .ok_or_else(|| anyhow::anyhow!("invalid hex digit"))?;
        Ok((hi * 16 + lo) as u8)
    };
    let r = pair(chars[0], chars[1])?;
    let g = pair(chars[2], chars[3])?;
    let b = pair(chars[4], chars[5])?;
    let a = if chars.len() == 8 {
        pair(chars[6], chars[7])? as f64 / 255.0
    } else {
        1.0
    };
    Ok(Rgba { r, g, b, a })
}

fn parse_rgb(s: &str) -> Result<Rgba> {
    let inner = s
        .strip_prefix("rgba(")
        .or_else(|| s.strip_prefix("RGBA("))
        .or_else(|| s.strip_prefix("rgb("))
        .or_else(|| s.strip_prefix("RGB("))
        .and_then(|x| x.strip_suffix(')'))
        .ok_or_else(|| anyhow::anyhow!("invalid rgb format"))?;
    let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
    if parts.len() != 3 && parts.len() != 4 {
        bail!("rgb requires 3 or 4 components");
    }
    let component = |v: &str| -> Result<u8> {
        v.parse::<u8>()
            .map_err(|_| anyhow::anyhow!("invalid rgb component: {v}"))
    };
    let r = component(parts[0])?;
    let g = component(parts[1])?;
    let b = component(parts[2])?;
    let a = if parts.len() == 4 {
        let v: f64 = parts[3]
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid alpha value"))?;
        if v <= 1.0 { v } else { v / 255.0 }
    } else {
        1.0
    };
    Ok(Rgba { r, g, b, a })
}

fn parse_hsl(s: &str) -> Result<Rgba> {
    let inner = s
        .strip_prefix("hsla(")
        .or_else(|| s.strip_prefix("HSLA("))
        .or_else(|| s.strip_prefix("hsl("))
        .or_else(|| s.strip_prefix("HSL("))
        .and_then(|x| x.strip_suffix(')'))
        .ok_or_else(|| anyhow::anyhow!("invalid hsl format"))?;
    let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
    if parts.len() != 3 && parts.len() != 4 {
        bail!("hsl requires 3 or 4 components");
    }
    let hue: f64 = parts[0]
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid hue value"))?;
    if hue < 0.0 {
        bail!("hue must be non-negative");
    }
    let h = hue % 360.0;
    let s: f64 = parts[1]
        .trim_end_matches('%')
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid saturation value"))?;
    let s = s / 100.0;
    let l: f64 = parts[2]
        .trim_end_matches('%')
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid lightness value"))?;
    let l = l / 100.0;
    let a = if parts.len() == 4 {
        let v: f64 = parts[3]
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid alpha value"))?;
        v.clamp(0.0, 1.0)
    } else {
        1.0
    };
    let (r, g, b) = hsl_to_rgb(h, s, l)?;
    Ok(Rgba { r, g, b, a })
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Result<(u8, u8, u8)> {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r1, g1, b1) = match hp as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    let conv = |v: f64| -> u8 { ((v + m) * 255.0).round() as u8 };
    Ok((conv(r1), conv(g1), conv(b1)))
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = f64::from(r) / 255.0;
    let g = f64::from(g) / 255.0;
    let b = f64::from(b) / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let l = (max + min) / 2.0;
    let (h, s) = if delta == 0.0 {
        (0.0, 0.0)
    } else {
        let saturation = delta / (1.0 - (2.0 * l - 1.0).abs());
        let hue = if max == r {
            ((g - b) / delta) % 6.0
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        } * 60.0;
        let hue = if hue < 0.0 { hue + 360.0 } else { hue };
        (hue, saturation)
    };
    (h.round(), (s * 100.0).round(), (l * 100.0).round())
}
