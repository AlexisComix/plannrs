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
    /// The Id for each entry in the plan table. It would be possible to 
    /// have the primary key be the start datetime as I do not plan to allow 
    /// users to have two study sessions at once (I do not think that it would
    /// be feasible to rewrite all of the other systems just to accomodate it).
    /// However, I think that I may as well just use IDs for it as it will always
    /// be simpler.
    pub(crate) id: u8,
    /// This is the description of the plan or study session. A user could
    /// write a short or a longer description (we would have to have a button to 
    /// toggle an expanded popup for the text, and possibly limit the number 
    /// of characters based off of that larger view. Most people will not see
    /// this part anyway, but it is worth thinking about.)
    pub(crate) description: String,
    /// This is the start time for any given study session. It is calculated
    /// based off of the local time on the relevant system. This should not
    /// cause any issues in future, perhaps unless someone changes timezones
    /// before the notification appears for a given plan.
    pub(crate) start: DateTime<Local>,
    /// Similar to the start time, this should only have issues if the user
    /// changes timezone. This will probably be an unlikely circumstance.
    /// However, I think that when they are in the new timezone, the displayed
    /// time will update to match so the user can change the time in the DB 
    /// manaully. There could also be a warning in the daemon that triggers
    /// a notification to do so if the timezone has changed. 
    pub(crate) until: DateTime<Local>,
    /// This is the amount of time before the start that the notification should
    /// appear. We will only use the seconds for this most likely, in increments
    /// of 60 for minutes.
    pub(crate) advance: Option<TimeDelta>,
    /// This is a flag for if the task has been completed or done. This can
    /// be a user changed checkmark on each task.
    pub(crate) done: bool,
    /// This is the option for the plan to be associated with a tag. If it 
    /// is not associated with a tag, it will have a default grey/black/white
    /// colour scheme in the timeline. 
    pub(crate) tag: Option<Tag>,
    /// This is a flag for if the notification will sound on the desktop
    /// or not. 
    pub(crate) notify: bool,
    /// This is a flag for porsmo integration. Planned, but not likely anytime
    /// soon.
    pub(crate) porsmo: bool,
}
