use crate::consts::ButtonStyle;

use super::{
    components::{ButtonObject, Component, SelectObject},
    emoji::Emoji,
};

/// A builder for creating components.
pub struct ComponentBuilder;

impl ComponentBuilder {
    /// Creates a button component.
    ///
    /// # Arguments
    ///
    /// * `button` - The button object.
    ///
    /// # Examples
    ///
    /// ```
    /// let button = ButtonObject {
    ///     style: ButtonStyle::Primary,
    ///     label: Some("Click me".to_string()),
    ///     emoji: None,
    ///     custom_id: Some("button1".to_string()),
    ///     url: None,
    ///     disabled: false,
    /// };
    /// let component = ComponentBuilder::button(button).unwrap();
    /// ```
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
            label,
            emoji,
            custom_id,
            url,
            disabled,

            type_: 2,
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

    /// Creates a select component.
    ///
    /// # Arguments
    ///
    /// * `select` - The select object.
    ///
    /// # Examples
    ///
    /// ```
    /// let select = SelectObject {
    ///     select_type: 3,
    ///     custom_id: "select1".to_string(),
    ///     options: Some(vec![]),
    ///     channel_types: None,
    ///     placeholder: Some("Choose an option".to_string()),
    ///     default_values: None,
    ///     min_values: None,
    ///     max_values: None,
    ///     disabled: false,
    /// };
    /// let component = ComponentBuilder::select(select).unwrap();
    /// ```
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
