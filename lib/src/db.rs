//! The types and structs to represent the tables in the database, as
//! well as basic database interaction functions

use ratatui::style::Color;
use chrono::{DateTime, TimeDelta, Local};

/// Tags are used to group data by subject - for example Maths or Chores.
/// These can be represented in the TUI using different colours. The colours
/// used are `ratatui::style::Color`, which use the ANSI colour table.
/// The theming of the colours can be changed by using different terminal
/// themes. 
/// A Tag in the database could be constructed through:
/// ```
/// let maths_tag: Tag { 
///     id: 0, 
///     name: String::from("Maths"), 
///     border: None,
///     fill: Some(Color::White),
///     color: Color::Black
/// };
/// 
/// assert!(maths_tag.color == Color::Black);
/// ```
pub struct Tag {
    /// The ID of the relevant Tag.
    pub(crate) id: u8, // We should never need more than u8 max tags.
    /// The Tag name
    pub(crate) name: String,
    /// The border colour for the Tag's blocks
    pub(crate) border: Option<Color>,
    /// The block fill colour for the Tag
    pub(crate) fill: Option<Color>,
    /// The foreground (Text) colour for the tag
    pub(crate) color: Color,
}

/// The plan table for the database. Has all of the relevant information needed.
/// In the actual database, `tag` will be the `Tag.id`. It is just simpler for
/// the sake of abstraction here to use the whole struct as we will be fetching
/// it whenever we want to fetch the tag anyway.
pub struct Plan {
    pub(crate) id: u8,
    pub(crate) description: String,
    pub(crate) start: DateTime<Local>,
    pub(crate) until: DateTime<Local>,
    pub(crate) advance: Option<TimeDelta>,
    pub(crate) done: bool,
    pub(crate) tag: Option<Tag>,
    pub(crate) notify: bool,
    pub(crate) porsmo: bool,
}
