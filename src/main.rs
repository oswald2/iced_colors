use iced::widget::{Column, Row, Container, Text, PickList, Space};
use iced::{Color, Element, Length, Theme};
use iced::alignment::Horizontal;

pub fn main() -> iced::Result {
    iced::application(PaletteViewer::default, update, view)
        .theme(|state: &PaletteViewer| state.theme.clone())
        .run()
}

struct PaletteViewer {
    theme: Theme,
}

impl Default for PaletteViewer {
    fn default() -> Self {
        PaletteViewer {
            theme: Theme::Light,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ThemeSelected(Theme),
}

fn update(state: &mut PaletteViewer, message: Message) {
    match message {
        Message::ThemeSelected(theme) => state.theme = theme,
    }
}

fn view(state: &PaletteViewer) -> Element<'_, Message> {
    // Create a pick list for theme switching
    let theme_picker = PickList::new(Theme::ALL, Some(state.theme.clone()), Message::ThemeSelected)
        .placeholder("Select a theme...");

    // Helper function to create a labeled color swatch
    fn swatch(color: Color, text_color: Color, label: &'_ str) -> Element<'_, Message> {
        Container::new(Text::new(label).size(14).color(text_color))
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(80.0))
            .center_x(Length::Fixed(120.0))
            .center_y(Length::Fixed(80.0))
            .style(move |_theme: &Theme| iced::widget::container::background(color))
            .into()
    }

    let palette = state.theme.extended_palette();

    // Create rows for each color category
    let primary_row = Row::new()
        .push(swatch(
            palette.primary.base.color,
            palette.primary.base.text,
            "Primary Base",
        ))
        .push(swatch(
            palette.primary.strong.color,
            palette.primary.strong.text,
            "Primary Strong",
        ))
        .push(swatch(
            palette.primary.weak.color,
            palette.primary.weak.text,
            "Primary Weak",
        ))
        .spacing(10);

    let success_row = Row::new()
        .push(swatch(
            palette.success.base.color,
            palette.success.base.text,
            "Success Base",
        ))
        .push(swatch(
            palette.success.strong.color,
            palette.success.strong.text,
            "Success Strong",
        ))
        .push(swatch(
            palette.success.weak.color,
            palette.success.weak.text,
            "Success Weak",
        ))
        .spacing(10);

    let danger_row = Row::new()
        .push(swatch(
            palette.danger.base.color,
            palette.danger.base.text,
            "Danger Base",
        ))
        .push(swatch(
            palette.danger.strong.color,
            palette.danger.strong.text,
            "Danger Strong",
        ))
        .push(swatch(
            palette.danger.weak.color,
            palette.danger.weak.text,
            "Danger Weak",
        ))
        .spacing(10);

    let warning_row = Row::new()
        .push(swatch(
            palette.warning.base.color,
            palette.warning.base.text,
            "Warning Base",
        ))
        .push(swatch(
            palette.warning.strong.color,
            palette.warning.strong.text,
            "Warning Strong",
        ))
        .push(swatch(
            palette.warning.weak.color,
            palette.warning.weak.text,
            "Warning Weak",
        ))
        .spacing(10);

    let background_row = Row::new()
        .push(swatch(
            palette.background.base.color,
            palette.background.base.text,
            "Background Base",
        ))
        .push(swatch(
            palette.background.neutral.color,
            palette.background.neutral.text,
            "Background Neutral",
        ))
        .push(swatch(
            palette.background.strong.color,
            palette.background.strong.text,
            "Background Strong",
        ))
        .push(swatch(
            palette.background.stronger.color,
            palette.background.stronger.text,
            "Background Stronger",
        ))
        .push(swatch(
            palette.background.strongest.color,
            palette.background.strongest.text,
            "Background Strongest",
        ))
        .push(swatch(
            palette.background.weak.color,
            palette.background.weak.text,
            "Background Weak",
        ))
        .push(swatch(
            palette.background.weaker.color,
            palette.background.weaker.text,
            "Background Weaker",
        ))
        .push(swatch(
            palette.background.weakest.color,
            palette.background.weakest.text,
            "Background Weakest",
        ));


    // Combine all rows into a column
    let content = Column::new()
        .push(theme_picker)
        .push(Space::new().height(Length::Fixed(20.0)))
        .push(Text::new("Extended Palette").size(20))
        .push(primary_row)
        .push(success_row)
        .push(danger_row)
        .push(warning_row)
        .push(background_row)
        .spacing(10)
        .align_x(Horizontal::Center);

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
        .padding(20)
        .into()
}
