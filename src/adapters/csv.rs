use core::entities::*;

#[derive(Debug, Serialize)]
pub struct CsvRecord {
    pub id: String,
    pub osm_node: Option<u64>,
    pub created: u64,
    pub version: u64,
    pub title: String,
    pub description: String,
    pub lat: f64,
    pub lng: f64,
    pub street: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub homepage: Option<String>,
    pub categories: String,
    pub tags: String,
    pub license: Option<String>,
    pub avg_rating: f64,
}

impl From<(Entry, Vec<Category>, f64)> for CsvRecord {
    fn from(t: (Entry, Vec<Category>, f64)) -> Self {
        let (e, categories, avg_rating) = t;

        let Entry {
            id,
            osm_node,
            created,
            version,
            title,
            description,
            lat,
            lng,
            street,
            zip,
            city,
            country,
            homepage,
            license,
            ..
        } = e.clone();

        let categories = categories
            .into_iter()
            .map(|c| c.name)
            .collect::<Vec<_>>()
            .join(",");

        CsvRecord {
            id,
            osm_node: osm_node.map(|x| x as u64),
            created: created as u64,
            version: version as u64,
            title,
            description,
            lat,
            lng,
            street,
            zip,
            city,
            country,
            homepage,
            license,
            categories,
            tags : e.tags.join(","),
            avg_rating,
        }
    }
}
