use super::{
    entities::*,
    error::RepoError,
    util::geo::{MapBbox, MapPoint},
};

use failure::Fallible;
use std::result;

type Result<T> = result::Result<T, RepoError>;

pub trait EntryGateway {
    fn create_entry(&self, _: Entry) -> Result<()>;
    fn get_entry(&self, _: &str) -> Result<Entry>;
    fn all_entries(&self) -> Result<Vec<Entry>>;
    fn count_entries(&self) -> Result<usize>;
    fn update_entry(&self, _: &Entry) -> Result<()>;
    fn import_multiple_entries(&mut self, _: &[Entry]) -> Result<()>;
}

pub trait EventGateway {
    fn create_event(&mut self, _: Event) -> Result<()>;
    fn get_event(&self, _: &str) -> Result<Event>;
    fn all_events(&self) -> Result<Vec<Event>>;
    fn update_event(&mut self, _: &Event) -> Result<()>;
    fn delete_event(&mut self, _: &str) -> Result<()>;
}

pub trait UserGateway {
    fn create_user(&mut self, user: User) -> Result<()>;
    fn update_user(&mut self, user: &User) -> Result<()>;
    fn get_user(&self, username: &str) -> Result<User>;
    // TODO: fn get_user_by_email(&self, email: &str) -> Result<User>;
    fn all_users(&self) -> Result<Vec<User>>;
    fn delete_user(&mut self, username: &str) -> Result<()>;
}

pub trait CommentGateway {
    fn create_comment(&self, _: Comment) -> Result<()>;
    fn all_comments(&self) -> Result<Vec<Comment>>;
}

pub trait OrganizationGateway {
    fn create_org(&mut self, _: Organization) -> Result<()>;
    fn get_org_by_api_token(&self, token: &str) -> Result<Organization>;
    fn get_all_tags_owned_by_orgs(&self) -> Result<Vec<String>>;
}

pub trait RatingRepository {
    fn get_rating(&self, id: &str) -> Result<Rating>;
    fn get_ratings(&self, ids: &[String]) -> Result<Vec<Rating>>;

    fn add_rating_for_entry(&self, rating: Rating) -> Result<()>;

    fn all_ratings_for_entry_by_id(&self, entry_id: &str) -> Result<Vec<Rating>>;
}

//TODO:
//  - TagGeatway
//  - CategoryGateway
//  - SubscriptionGateway

pub trait Db:
    EntryGateway + UserGateway + CommentGateway + EventGateway + OrganizationGateway + RatingRepository
{
    fn create_tag_if_it_does_not_exist(&self, _: &Tag) -> Result<()>;
    fn create_category_if_it_does_not_exist(&mut self, _: &Category) -> Result<()>;
    fn create_bbox_subscription(&mut self, _: &BboxSubscription) -> Result<()>;

    fn all_categories(&self) -> Result<Vec<Category>>;
    fn all_tags(&self) -> Result<Vec<Tag>>;
    fn count_tags(&self) -> Result<usize>;
    fn all_bbox_subscriptions(&self) -> Result<Vec<BboxSubscription>>;

    fn delete_bbox_subscription(&mut self, _: &str) -> Result<()>;
}

#[derive(Debug, Default, Clone)]
pub struct IndexedEntry {
    pub id: String,
    pub pos: MapPoint,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub ratings: AvgRatings,
}

#[derive(Debug, Clone)]
pub struct EntryIndexQuery {
    pub bbox: Option<MapBbox>,
    pub text: Option<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

pub trait EntryIndex {
    fn query_entries(&self, query: &EntryIndexQuery, limit: usize) -> Fallible<Vec<IndexedEntry>>;
}

pub trait EntryIndexer: EntryIndex {
    fn add_or_update_entry(&mut self, entry: &Entry, ratings: &AvgRatings) -> Fallible<()>;
    fn remove_entry_by_id(&mut self, id: &str) -> Fallible<()>;
    fn flush(&mut self) -> Fallible<()>;
}
