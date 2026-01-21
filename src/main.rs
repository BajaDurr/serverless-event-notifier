use lambda_http::{run, service_fn, Body, Error, Request, Response};

use chrono::{
    NaiveDate, NaiveDateTime, NaiveTime, Local, Duration,
};

use pi_test::EventsResponse;
use aws_sdk_sns::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let html = get_todays_events_html().await;

    //send_email("Test message from Lambda").await;

    let response = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::Text(html))
        .expect("failed to render response");

    Ok(response)
}
async fn get_todays_events_html() -> String {
    let api_key = std::env::var("TICKETMASTER_API_KEY")
        .expect("TICKETMASTER_API_KEY not set");

    let url = format!(
        "https://app.ticketmaster.com/discovery/v2/events.json?venueId=KovZpZA6AJdA&apikey={}",
        api_key
    );

    let response = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(_) => return simple_page("Ticket service unavailable"),
    };

    //NON-200 HANDLING
    if !response.status().is_success() {
        return simple_page("Ticket service temporarily unavailable");
    }

    let body_text = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let body = serde_json::from_str::<EventsResponse>(&body_text).unwrap();

    let today = Local::now().date_naive();
    let now = Local::now().naive_local();

let mut events: Vec<_> = body
    .embedded
    .map(|e| e.events)          // extract events if Some
    .unwrap_or_default()        // use empty Vec if None
    .into_iter()
    .filter(|e| {
        let is_today = NaiveDate::parse_from_str(
            &e.dates.start.localDate,
            "%Y-%m-%d",
        )
        .map(|d| d == today)
        .unwrap_or(false);

        let not_suites = !e.name.starts_with("Suites");
        let not_voucher = !e.name.starts_with("Recovery");
        let not_guestpass = !e.name.starts_with("Wild");
        let not_int_fee = !e.name.starts_with("Int Fee");


        is_today && not_suites && not_voucher && not_guestpass && not_int_fee
    })
    .collect();

    events.sort_by(|a, b| {
        let a_dt = make_naive_datetime(
            &a.dates.start.localDate,
            a.dates.start.localTime.as_deref(),
        );
        let b_dt = make_naive_datetime(
            &b.dates.start.localDate,
            b.dates.start.localTime.as_deref(),
        );
        a_dt.cmp(&b_dt)
    });

    let date_display = today.format("%A • %m-%d-%Y").to_string();

    let mut html = String::from(
        r#"<html>
<head>
<meta charset="utf-8">
<title>Today's Events</title>
<style>
body {
    background: #000;
    color: white;
    font-family: Arial, sans-serif;
    margin: 0;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
}
.container {
    width: 100%;
    text-align: center;
}
.header {
    font-size: 26px;
    color: #bbb;
    margin-bottom: 10px;
}
.slide {
    display: none;
    padding: 20px;
    animation: fade 0.8s ease-in-out;
}
@keyframes fade {
    from { opacity: 0; }
    to { opacity: 1; }
}
.title {
    font-size: 36px;
    font-weight: bold;
    color: #00e5ff;
    line-height: 1.2;
}
.time-line {
    font-size: 34px;
    margin-top: 14px;
}
.time-upcoming { color: #ffd54f; }
.time-later { color: #64b5f6; }
.time-live {
    color: #ff5252;
    font-weight: bold;
}
.live-badge {
    display: inline-block;
    margin-left: 12px;
    padding: 6px 14px;
    border-radius: 8px;
    background: #ff1744;
    color: white;
    font-size: 18px;
    animation: pulse 1.2s infinite;
}
@keyframes pulse {
    0% { opacity: 1; }
    50% { opacity: 0.6; }
    100% { opacity: 1; }
}
.empty { color: #777; }
</style>
</head>
<body>
<div class="container">
<div class="header">"#,
    );

    html.push_str(&date_display);
    html.push_str(r#"</div><div id="slides">"#);

    if events.is_empty() {
        html.push_str(
            r#"<div class="slide" style="display:block;">
                <div class="title empty">No events today</div>
            </div>"#,
        );
    } else {
        for event in events {
            let start_dt = make_naive_datetime(
                &event.dates.start.localDate,
                event.dates.start.localTime.as_deref(),
            );

            let end_dt = start_dt + Duration::hours(2);

            let start_time = start_dt.time().format("%-I:%M %p").to_string();
            let end_time = end_dt.time().format("%-I:%M %p").to_string();

            let (time_class, live_badge) = if now >= start_dt && now <= end_dt {
                ("time-live", r#"<span class="live-badge">LIVE</span>"#)
            } else if start_dt - now <= Duration::hours(2) {
                ("time-upcoming", "")
            } else {
                ("time-later", "")
            };

            html.push_str(&format!(
                r#"<div class="slide">
                    <div class="title">{}</div>
                    <div class="time-line {}">{} – {} {}</div>
                </div>"#,
                event.name,
                time_class,
                start_time,
                end_time,
                live_badge
            ));
        }
    }

    html.push_str(
        r#"</div>
<script>
let slides = document.querySelectorAll(".slide");
let index = 0;
function showSlide(i) {
    slides.forEach(s => s.style.display = "none");
    slides[i].style.display = "block";
}
if (slides.length > 0) {
    showSlide(0);
    setInterval(() => {
        index = (index + 1) % slides.length;
        showSlide(index);
    }, 10000);
}
</script>
</div>
</body>
</html>"#,
    );

    html
}

fn simple_page(msg: &str) -> String {
    format!(
        "<html><body style=\"background:black;color:white;font-family:sans-serif;text-align:center;padding-top:40vh;\"><h2>{}</h2></body></html>",
        msg
    )
}

fn make_naive_datetime(
    date_str: &str,
    time_str: Option<&str>,
) -> NaiveDateTime {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let time = match time_str {
        Some(t) => NaiveTime::parse_from_str(t, "%H:%M:%S").unwrap(),
        None => NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
    };
    NaiveDateTime::new(date, time)
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

    match client
        .publish()
        .topic_arn(topic_arn)
        .message(msg)
        .send()
        .await
    {
        Ok(out) => {
            println!("SNS publish OK. message_id={:?}", out.message_id());
        }
        Err(e) => {
            eprintln!("SNS publish FAILED: {:?}", e);
        }
    }
}
