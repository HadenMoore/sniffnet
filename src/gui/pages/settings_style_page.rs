use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::{button, horizontal_space, lazy, vertical_space, Rule, TextInput};
use iced::widget::{Button, Column, Container, Row, Scrollable, Space, Text};
use iced::Length::Fixed;
use iced::{Alignment, Element, Font, Length, Renderer};

use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{
    get_font, get_font_headers, BORDER_WIDTH, FONT_SIZE_SUBTITLE,
};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::styles::types::custom_palette::{CustomPalette, ExtraStyles};
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::translations::translations::{
    appearance_title_translation, deep_sea_translation, mon_amour_translation,
    yeti_day_translation, yeti_night_translation,
};
use crate::translations::translations_2::color_gradients_translation;
use crate::translations::translations_3::custom_style_translation;
use crate::utils::types::icon::Icon;
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{Language, Sniffer, StyleType};

pub fn settings_style_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let style_path = &sniffer.settings.style_path;
    let color_gradient = sniffer.settings.color_gradient;
    let language = sniffer.settings.language;
    let font = get_font(style);
    let font_headers = get_font_headers(style);

    let mut content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(
            font,
            font_headers,
            color_gradient,
            language,
        ))
        .push(get_settings_tabs(SettingsPage::Appearance, font, language))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(
            appearance_title_translation(language)
                .style(TextType::Subtitle)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Length::Fixed(15.0)))
        .push(gradients_row(font, color_gradient, language))
        .push(vertical_space(Length::Fixed(15.0)));

    let mut styles_col = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(
            Row::new()
                .push(get_palette_container(
                    style,
                    "Yeti Night".to_string(),
                    yeti_night_translation(language).to_string(),
                    Night,
                ))
                .push(horizontal_space(Length::Fixed(15.0)))
                .push(get_palette_container(
                    style,
                    "Yeti Day".to_string(),
                    yeti_day_translation(language).to_string(),
                    Day,
                )),
        )
        .push(vertical_space(Length::Fixed(10.0)))
        .push(
            Row::new()
                .push(get_palette_container(
                    style,
                    "Deep Sea".to_string(),
                    deep_sea_translation(language).to_string(),
                    DeepSea,
                ))
                .push(horizontal_space(Length::Fixed(15.0)))
                .push(get_palette_container(
                    style,
                    "Mon Amour".to_string(),
                    mon_amour_translation(language).to_string(),
                    MonAmour,
                )),
        )
        .push(vertical_space(Length::Fixed(10.0)));
    for children in get_extra_palettes(ExtraStyles::all_styles(), style) {
        styles_col = styles_col.push(children);
    }
    styles_col = styles_col
        .push(lazy((style_path, style), move |_| {
            lazy_custom_style_input(language, font, style_path, style)
        }))
        .push(vertical_space(10));

    let styles_scroll =
        Scrollable::new(styles_col).direction(Direction::Vertical(ScrollbarType::properties()));

    content = content.push(styles_scroll);

    Container::new(content)
        .height(Length::Fixed(400.0))
        .width(Length::Fixed(800.0))
        .style(ContainerType::Modal)
}

fn gradients_row(
    font: Font,
    color_gradient: GradientType,
    language: Language,
) -> Row<'static, Message, Renderer<StyleType>> {
    Row::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .push(Text::new(format!("{}:", color_gradients_translation(language))).font(font))
        .push(
            button(
                Icon::Forbidden
                    .to_text()
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(12),
            )
            .padding(0)
            .height(20.0)
            .width(Fixed(if color_gradient.eq(&GradientType::None) {
                60.0
            } else {
                20.0
            }))
            .style(ButtonType::Gradient(GradientType::None))
            .on_press(Message::GradientsSelection(GradientType::None)),
        )
        .push(
            button(
                Icon::Waves
                    .to_text()
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(13),
            )
            .padding(0)
            .height(20.0)
            .width(Fixed(if color_gradient.eq(&GradientType::Mild) {
                60.0
            } else {
                20.0
            }))
            .on_press(Message::GradientsSelection(GradientType::Mild))
            .style(ButtonType::Gradient(GradientType::Mild)),
        )
        .push(
            button(
                Icon::Lightning
                    .to_text()
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(13),
            )
            .padding(0)
            .height(20.0)
            .width(Fixed(if color_gradient.eq(&GradientType::Wild) {
                60.0
            } else {
                20.0
            }))
            .on_press(Message::GradientsSelection(GradientType::Wild))
            .style(ButtonType::Gradient(GradientType::Wild)),
        )
}

fn get_palette_container(
    style: StyleType,
    name: String,
    description: String,
    on_press: StyleType,
) -> Button<'static, Message, Renderer<StyleType>> {
    let font = get_font(style);

    let is_custom = matches!(on_press, StyleType::Custom(_));

    let mut content = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(name).font(font))
        .push(get_palette(on_press, is_custom));

    if !is_custom {
        content = content.push(Text::new(description).font(font));
    }

    Button::new(content)
        .height(Length::Fixed(if is_custom { 75.0 } else { 110.0 }))
        .width(Length::Fixed(380.0))
        .padding(5)
        .style(if on_press.eq(&style) {
            ButtonType::BorderedRoundSelected
        } else {
            ButtonType::BorderedRound
        })
        .on_press(Message::Style(on_press))
}

fn get_palette(
    style: StyleType,
    is_custom: bool,
) -> Container<'static, Message, Renderer<StyleType>> {
    let height = if is_custom { 25.0 } else { 40.0 };

    Container::new(
        Row::new()
            .push(
                Row::new()
                    .width(Length::Fixed(120.0))
                    .push(Rule::horizontal(height).style(RuleType::PalettePrimary(style))),
            )
            .push(
                Row::new()
                    .width(Length::Fixed(80.0))
                    .push(Rule::horizontal(height).style(RuleType::PaletteSecondary(style))),
            )
            .push(
                Row::new()
                    .width(Length::Fixed(60.0))
                    .push(Rule::horizontal(height).style(RuleType::PaletteOutgoing(style))),
            )
            .push(
                Row::new()
                    .width(Length::Fixed(40.0))
                    .push(Rule::horizontal(height).style(RuleType::PaletteButtons(style))),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .width(300.0 + 2.0 * BORDER_WIDTH)
    .height(height + 1.7 * BORDER_WIDTH)
    .style(ContainerType::Palette)
}

// Buttons for each extra style arranged in rows of two
fn get_extra_palettes(
    styles: &[ExtraStyles],
    current_style: StyleType,
) -> Vec<Element<'static, Message, Renderer<StyleType>>> {
    // Map each extra style into a palette container
    let mut styles = styles.iter().map(|&style| {
        let name = style.to_string();
        let description = String::new();
        let style = StyleType::Custom(style);
        get_palette_container(current_style, name, description, style)
    });

    // The best way to do this would be with itertools, but that would introduce another dependency.
    let mut children = Vec::with_capacity(styles.len());

    // This handles the case where there aren't an even number of styles.
    // [Iterator::zip] drops remainders. Itertools' `zip_longest` and the unstable array chunks API
    // are both better solutions.
    while let (Some(first), second) = (styles.next(), styles.next()) {
        // Add both styles and the vertical space if there are two styles.
        if let Some(second) = second {
            children.extend([
                Row::new()
                    .push(first)
                    .push(horizontal_space(Length::Fixed(15.0)))
                    .push(second)
                    .into(),
                <Space as Into<Element<Message, Renderer<StyleType>>>>::into(vertical_space(
                    Length::Fixed(10.0),
                )),
            ]);
        } else {
            children.extend([
                Row::new().push(first).into(),
                <Space as Into<Element<Message, Renderer<StyleType>>>>::into(vertical_space(
                    Length::Fixed(10.0),
                )),
            ]);
        }
    }

    children
}

fn lazy_custom_style_input(
    language: Language,
    font: Font,
    custom_path: &str,
    style: StyleType,
) -> Button<'static, Message, Renderer<StyleType>> {
    let is_custom_toml_style_set = matches!(style, StyleType::Custom(ExtraStyles::CustomToml(_)));

    let custom_palette = CustomPalette::from_file(custom_path);
    let is_error = if custom_path.is_empty() {
        false
    } else {
        custom_palette.is_err()
    };

    let input = TextInput::new("-", custom_path)
        .on_input(Message::LoadStyle)
        .on_submit(Message::LoadStyle(custom_path.to_string()))
        .padding([0, 5])
        .font(font)
        .width(Length::Fixed(300.0))
        .style(if is_error {
            TextInputType::Error
        } else {
            TextInputType::Standard
        });

    let mut content = Column::new()
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(custom_style_translation(language)).font(font))
        .push(input);

    if is_custom_toml_style_set {
        content = content.push(get_palette(style, true));
    } else if let Ok(palette) = custom_palette {
        content = content.push(get_palette(
            StyleType::Custom(ExtraStyles::CustomToml(palette)),
            true,
        ));
    }

    Button::new(content)
        .height(Length::Fixed(
            if custom_palette.is_ok() || is_custom_toml_style_set {
                110.0
            } else {
                75.0
            },
        ))
        .width(Length::Fixed(380.0))
        .padding([10, 0, 5, 0])
        .style(if is_custom_toml_style_set {
            ButtonType::BorderedRoundSelected
        } else {
            ButtonType::BorderedRound
        })
        .on_press(Message::LoadStyle(custom_path.to_string()))
}
