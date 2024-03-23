use crate::consts::ButtonStyle;

use super::{
    components::{ButtonObject, Component, SelectObject},
    emoji::Emoji,
};

pub struct ComponentBuilder;

impl ComponentBuilder {
    pub fn button(button: ButtonObject) -> Result<Component, &'static str> {
        button.verify()?;

        let ButtonObject {
            style,
            label,
            emoji,
            custom_id,
            url,
            disabled,
        } = button;

        Ok(Component {
            type_: 2,
            label,
            emoji,
            custom_id,
            url,
            disabled,
            style: Some(style),
            components: None,
            options: None,
            channel_types: None,
            placeholder: None,
            default_values: None,
            min_values: None,
            max_values: None,
        })
    }

    pub fn select(select: SelectObject) -> Result<Component, &'static str> {
        select.verify()?;

        let SelectObject {
            select_type,
            custom_id,
            options,
            channel_types,
            placeholder,
            default_values,
            min_values,
            max_values,
            disabled,
        } = select;

        Ok(Component {
            type_: select_type as _,
            components: None,
            style: None,
            label: None,
            emoji: None,
            custom_id: Some(custom_id),
            url: None,
            disabled,
            options,
            channel_types: channel_types.map(|i| i.into_iter().map(|i| i as u32).collect()),
            placeholder,
            default_values,
            min_values,
            max_values,
        })
    }
}
