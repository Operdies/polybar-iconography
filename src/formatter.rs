use crate::colors::Color;

#[derive(Default, Clone)]
pub struct Formatter {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub underline: Option<Color>,
    pub overline: Option<Color>,
    /// Inverts foreground / background colors
    pub highlight: bool,
}

impl Formatter {
    pub fn with_background(&self, bg: Option<Color>) -> Self {
        Self {
            background: bg,
            ..self.clone()
        }
    }
    pub fn with_underline(&self, fg: Option<Color>) -> Self {
        Self {
            underline: fg,
            ..self.clone()
        }
    }
    pub fn with_overline(&self, fg: Option<Color>) -> Self {
        Self {
            overline: fg,
            ..self.clone()
        }
    }
    pub fn with_foreground(&self, fg: Option<Color>) -> Self {
        Self {
            foreground: fg,
            ..self.clone()
        }
    }

    pub fn with_highlight(&self, highlight: bool) -> Self {
        Self {
            highlight,
            ..self.clone()
        }
    }
}

impl Formatter {
    pub fn format<S>(&self, text: S, link: Option<&str>) -> String
    where
        S: Into<String>,
    {
        let mut beg = vec![];
        let mut end = vec![];

        if let Some(bg) = self.background {
            let color_str = bg.to_string();
            beg.push(format!("%{{B{color_str}}}"));
            end.push("%{B-}")
        }

        if let Some(fg) = self.foreground {
            let color_str = fg.to_string();
            beg.push(format!("%{{F{color_str}}}"));
            end.push("%{F-}")
        }

        if self.highlight {
            beg.push("%{R}".to_string());
            end.push("%{R}");
        }

        if let Some(ul) = self.underline {
            let color_str = ul.to_string();
            beg.push(format!("%{{u{color_str}}}%{{+u}}"));
            end.push("%{-u}")
        }
        if let Some(ol) = self.overline {
            let color_str = ol.to_string();
            beg.push(format!("%{{o{color_str}}}%{{+o}}"));
            end.push("%{-o}")
        }

        if let Some(link) = link {
            beg.push(format!("%{{A1:{}:}}", link));
            end.push("%{A}");
        }

        format!(
            "{}{}{}",
            beg.join(""),
            text.into(),
            end.into_iter().rev().collect::<Vec<_>>().join("")
        )
    }
}
