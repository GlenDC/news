use std::time::SystemTime;

/// API Representation of an Item,
/// containing the info and data of any Post, Question and comment
/// as stored for Plabayo News and shown on the website.
pub struct Item {
    /// The item's unique id.
    pub id: ItemID,
    /// Indicates if the item is alive, deleted or locked.
    /// Locked items are also hidden by default (but can still be accessed using a direct URL).
    pub state: ItemState,
    /// The kind of item, e.g. comment or story.
    pub kind: ItemKind,
    /// The id of the user that created this item.
    pub by: UserID,
    /// Time the item was created.
    pub time: SystemTime,
    /// Time the item was last modified,
    /// and it will equal the .item sibling property's value
    /// in case the item was never modified after its creation.
    pub mod_time: SystemTime,
    /// The HTML-formatted text in case of a comment or question,
    /// not defined in the case of a post.
    pub text: Option<String>,
    /// An optional ID of the parent item,
    /// which could be a parent comment or the story in case of a root comment,
    /// a story has no parent.
    pub parent: Option<ItemID>,
    /// The identifiers of the root comments of a story or direct replies to a comment.
    /// It will be empty in case the item is a story without comments or a comment without replies.
    pub kids: Vec<ItemID>,
    /// If the item is a story it will contain the url of the webpage it refers to.
    pub url: Option<String>,
    /// Title of the post or question, empty in case the item is a comment.
    pub title: Option<String>,
}

/// The possible kinds an Item can be.
pub enum ItemKind {
    Story,
    Question,
    Comment,
}

/// The possible states an item can be in,
/// each item is in exactly one of these states at all times.
pub enum ItemState {
    Alive,
    Deleted,
    Locked,
}

/// The unique ID (identifier) of an item.
pub type ItemID = u64;

/// The unique ID (identifier) of a user.
/// We refer to users always by their unique ID,
/// rather than some kind of username of choice,
/// as to make it easy for the user to change this info,
/// as well as allow the possibility for a user to stay private if desired.
pub type UserID = u64;

pub struct User {
    /// The user's unique ID, auto generated by the system.
    pub id: UserID,
    /// Indicates if the user is public, hidden,
    /// deleted or Locked.
    pub state: UserState,
    /// An optional username of the user's choosing,
    /// only rules are that it isn't taken yet and follows the site's guidelines.
    pub username: Option<String>,
    /// An optional name of the user,
    /// this name is not validated but it should also follow the site's guidelines.
    pub name: Option<String>,
    /// An optional description of the user's location,
    /// could refer to a city, country, combination or other indicative description
    /// of the user's location.
    pub location: Option<String>,
    /// Time the user was created.
    pub create_time: SystemTime,
    /// Time the user was last logged in.
    pub last_login_time: SystemTime,
    /// Karma points collected by the user (can also be negative).
    pub karma: i64,
    /// The user's optional self-description, HTML Formatted.
    pub about: Option<String>,
    /// The posts, questions and comments submitted by the user,
    /// in the order of creation.
    pub items: Vec<ItemID>,
    /// A list of unique IPs with which the user has accessed this website.
    /// kept only to allow the possibility of adding it to a ban list,
    /// if ever required. Let's hope not.
    pub ips: Vec<String>,
    /// All authentication forms that can be used by a user
    /// to identify itself and proof their authority, as part
    /// of their login procedure. A user requires at least one
    /// form of UserAuthentication.
    pub authentications: Vec<Box<dyn UserAuthentication>>,
    /// Optional preferences that can be configured by the user.
    pub preferences: Option<UserPreferences>,
}

/// The possible states a User can be in,
/// each user is in exactly one of these states at all times.
pub enum UserState {
    /// Default state of a User. It indicates the user is active,
    /// and allows other users to check this user's profile page/info.
    Public,
    /// Optional state of a User, it indicates the user marked itself
    /// as hidden ensuring others cannot access its profile page/info.
    Hidden,
    /// A state triggered by the user when removing its account.
    /// By default we do not delete any items of the user as it would
    /// create a ripple effect. The bio and optional info of the user
    /// would be deleted however.
    Deleted,
    /// A state triggered by an admin in case the account is Locked
    /// for reasons such as violating the site's guidelines despite
    /// multiple warnings.
    Locked,
}

/// The possible kinds a user can be. The user is only on of these.
pub enum UserKind {
    /// Authorizes the User as a (regular) member,
    /// allowing the user to submit items and store
    /// preferences.
    Member,
    /// Authorizes the User with moderator privileges,
    /// on top of the privileges given to a member.
    /// A moderator is allowed to manage items from other users.
    Moderator,
    /// Authorizes the User with admin privileges
    /// and all privileges given to a moderator.
    /// An admin is allowed to modify other users.
    Admin,
}

/// A UserAuthentication is used to identify and authenticate the user,
/// the authorization is defined by what kind of user it is.
pub trait UserAuthentication {}

/// Bundles the optional preferences a user can configure.
pub struct UserPreferences {
    /// Defines the language/locale used for the user,
    /// despite what its browser might define, which is the default.
    /// Only languages, supported by Plabayo News can be chosen
    pub language: UserLanguage,
    /// Color schema to be used by the user. By default it is defined
    /// by the info provided by the browser (client).
    pub color_schema: ColorSchema,
}

/// Languages (locales) that can be used by the user for the localization
/// of the website. As such only languages already supported by us can be chosen (found here).
pub enum UserLanguage {
    Auto,
    /// automatically detect the language using the browser's client
    English,
    Spanish,
    Dutch,
}

/// The colorSchema for the website.
pub enum ColorSchema {
    /// Automatically choose the color schema based on the input of the browser client,
    /// this is default behavior.
    Auto,
    /// The default light Plabayo News Color Schema.
    Light,
    /// The default dark Plabayo news Color Schema.
    Dark,
}

/// Used to keep a log of actions happening on the website,
/// to keep track of how karma has been affected, post votes,
/// user and item state.
pub struct Action {} // TODO
