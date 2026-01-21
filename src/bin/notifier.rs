use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::Value;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Local, Duration};

use aws_sdk_sns::Client;
use pi_test::EventsResponse;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(_event: LambdaEvent<Value>) -> Result<(), Error> {
    // Build the daily message and send it
    let msg = build_daily_message().await;
    send_email(&msg).await;
    Ok(())
}

async fn send_email(msg: &str) {
    let topic_arn = match std::env::var("SNS_TOPIC_ARN") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("SNS_TOPIC_ARN missing: {:?}", e);
            return;
        }
    };

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    match client.publish().topic_arn(topic_arn).message(msg).send().await {
        Ok(out) => println!("SNS publish OK. message_id={:?}", out.message_id()),
        Err(e) => eprintln!("SNS publish FAILED: {:?}", e),
    }
}

async fn build_daily_message() -> String {
    let api_key = match std::env::var("TICKETMASTER_API_KEY") {
        Ok(v) => v,
        Err(_) => return "Daily Events: API key missing".to_string(),
    };

    let url = format!(
        "https://app.ticketmaster.com/discovery/v2/events.json?venueId=KovZpZA6AJdA&apikey={}",
        api_key
    );

    let response = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(_) => return "Daily Events: ticket service unreachable".to_string(),
    };

    if !response.status().is_success() {
        return format!("Daily Events: Ticketmaster returned {}", response.status());
    }

    let body_text = match response.text().await {
        Ok(t) => t,
        Err(_) => return "Daily Events: failed reading response".to_string(),
    };

    let body: EventsResponse = match serde_json::from_str(&body_text) {
        Ok(b) => b,
        Err(_) => return "Daily Events: failed parsing event data".to_string(),
    };

    let today = Local::now().date_naive();
    let now = Local::now().naive_local();

    let events_iter = body.embedded.map(|e| e.events).unwrap_or_default();

    let mut events: Vec<_> = events_iter
        .into_iter()
        .filter(|e| {
            let is_today = NaiveDate::parse_from_str(&e.dates.start.localDate, "%Y-%m-%d")
                .map(|d| d == today)
                .unwrap_or(false);

            let not_suites = !e.name.starts_with("Suites");
            let not_voucher = !e.name.starts_with("Recovery");
            let not_guestpass = !e.name.starts_with("Wild");

            is_today && not_suites && not_voucher && not_guestpass
        })
        .collect();

    events.sort_by(|a, b| {
        let a_dt = make_naive_datetime(&a.dates.start.localDate, a.dates.start.localTime.as_deref());
        let b_dt = make_naive_datetime(&b.dates.start.localDate, b.dates.start.localTime.as_deref());
        a_dt.cmp(&b_dt)
    });

    if events.is_empty() {
        return format!("{}: No events today.", today.format("%A %m-%d-%Y"));
    }

    let mut lines = Vec::new();
    lines.push(format!("{} Events:", today.format("%A %m-%d-%Y")));

    for e in events {
        let start_dt = make_naive_datetime(&e.dates.start.localDate, e.dates.start.localTime.as_deref());
        let end_dt = start_dt + Duration::hours(2);

        let start_time = start_dt.time().format("%-I:%M %p").to_string();
        let end_time = end_dt.time().format("%-I:%M %p").to_string();

        let live = if now >= start_dt && now <= end_dt { " (LIVE)" } else { "" };

        lines.push(format!("- {}: {}–{}{}", e.name, start_time, end_time, live));
    }

    lines.join("\n")
}

fn make_naive_datetime(date_str: &str, time_str: Option<&str>) -> NaiveDateTime {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let time = match time_str {
        Some(t) => NaiveTime::parse_from_str(t, "%H:%M:%S").unwrap(),
        None => NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
    };
    NaiveDateTime::new(date, time)
}
