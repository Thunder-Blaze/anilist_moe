pub mod activity;
pub mod airing;
pub mod forum;
pub mod media;
pub mod notification;
pub mod staff;
pub mod user;

pub use activity::ActivityEndpoint;
pub use airing::AiringEndpoint;
pub use forum::ForumEndpoint;
pub use media::MediaEndpoint;
pub use staff::StaffEndpoint;
pub use user::UserEndpoint;
pub use notification::NotificationEndpoint;
