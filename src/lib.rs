use serde::Deserialize;

/* =========================
   Top-level response
   ========================= */

#[derive(Debug, Deserialize)]
pub struct EventsResponse {
    #[serde(rename = "_embedded", default)]
    pub embedded: Option<EmbeddedEvents>,

    #[serde(rename = "_links")]
    pub links: Option<Links>,

    pub page: Page,
}


/* =========================
   Embedded events
   ========================= */

#[derive(Debug, Deserialize)]
pub struct EmbeddedEvents {
    pub events: Vec<Event>,
}

/* =========================
   Event
   ========================= */

#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,

    #[serde(rename = "type")]
    pub event_type: String,

    pub url: Option<String>,
    pub locale: Option<String>,
    pub test: bool,

    pub images: Option<Vec<Image>>,
    pub sales: Option<Sales>,
    pub dates: Dates,

    pub classifications: Option<Vec<Classification>>,
    pub promoter: Option<Promoter>,
    pub promoters: Option<Vec<Promoter>>,

    #[serde(rename = "_embedded")]
    pub embedded: Option<EventEmbedded>,
}

/* =========================
   Images
   ========================= */

#[derive(Debug, Deserialize)]
pub struct Image {
    pub ratio: Option<String>,
    pub url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fallback: Option<bool>,
}

/* =========================
   Sales
   ========================= */

#[derive(Debug, Deserialize)]
pub struct Sales {
    pub public: PublicSales,
    pub presales: Option<Vec<Presale>>,
}

#[derive(Debug, Deserialize)]
pub struct PublicSales {
    #[serde(rename = "startDateTime")]
    pub start_date_time: Option<String>,

    #[serde(rename = "endDateTime")]
    pub end_date_time: Option<String>,

    pub startTBD: bool,
    pub startTBA: bool,
}

#[derive(Debug, Deserialize)]
pub struct Presale {
    #[serde(rename = "startDateTime")]
    pub start_date_time: Option<String>,

    #[serde(rename = "endDateTime")]
    pub end_date_time: Option<String>,

    pub name: String,
}

/* =========================
   Dates
   ========================= */

#[derive(Debug, Deserialize)]
pub struct Dates {
    pub start: EventStart,
    pub timezone: Option<String>,
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct EventStart {
    pub localDate: String,
    pub localTime: Option<String>,
    pub dateTime: Option<String>,

    pub dateTBD: bool,
    pub dateTBA: bool,
    pub timeTBA: bool,
    pub noSpecificTime: bool,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub code: String,
}

/* =========================
   Classification
   ========================= */

#[derive(Debug, Deserialize)]
pub struct Classification {
    pub primary: bool,
    pub segment: Category,
    pub genre: Category,
    pub subGenre: Option<Category>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

/* =========================
   Promoters
   ========================= */

#[derive(Debug, Deserialize)]
pub struct Promoter {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

/* =========================
   Embedded sub-resources
   ========================= */

#[derive(Debug, Deserialize)]
pub struct EventEmbedded {
    pub venues: Option<Vec<Venue>>,
    pub attractions: Option<Vec<Attraction>>,
}

#[derive(Debug, Deserialize)]
pub struct Venue {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Attraction {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub locale: Option<String>,
}

/* =========================
   Pagination
   ========================= */

#[derive(Debug, Deserialize)]
pub struct Page {
    pub size: u32,
    pub totalElements: u32,
    pub totalPages: u32,
    pub number: u32,
}

/* =========================
   Links
   ========================= */

#[derive(Debug, Deserialize)]
pub struct Links {
    pub first: Option<Link>,
    #[serde(rename = "self")]
    pub self_: Option<Link>,
    pub next: Option<Link>,
    pub last: Option<Link>,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub href: Option<String>,
}
