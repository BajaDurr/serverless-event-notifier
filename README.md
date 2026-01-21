# 📣 Serverless Event Notifier (AWS, Rust, Docker)



This project is a **production-style, serverless notification system** built in Rust and deployed on AWS. It runs on a scheduled basis using **EventBridge** and **AWS Lambda**, fetches daily events from the Ticketmaster API, and sends email notifications using **Amazon SNS**.

The system is designed as a **fault-tolerant, event-driven** worker with reproducible builds using Docker and minimal operational overhead.

### This project was intended for displaying the daily events at the [Grand Casino Arena](https://www.grandcasinoarena.com) in St Paul, MN


## 🏗️ Architecture
- **AWS Lambda** --Executes the notfifier as a serverless worker
- **Amazon EventBridge** --Triggers the Lambda on a daily schedule
- **Amazon SNS** --Sends email notifications
- **Docker** --Provides deterministic, reproducible Linux builds for Lambda
- **Rust** --High-performance, memory-safe runtime

### Flow:

```text
EventBridge (cron) → Lambda (Rust) → Ticketmaster API → SNS (Email)
```

## ✅ Features
- Scheduled, automated daily execution
- Resilient external API integration
- Graceful handling of empty or missing event data
- Fault-tolerant retries via EventBridge
- Reproducible Docker-based build pipeline
- Fully serverless, zero server management

## 📁 Repository Structure

 ```text
.
├── src/
│   ├── bin/
│   │   └── notifier.rs      # Lambda worker entrypoint
│   ├── lib.rs               # Shared API models
│   └── main.rs              # (legacy / local testing)
├── output/
│   ├── bootstrap            # Lambda executable
│   ├── function.zip         # Deployment package
│   └── notifier.zip         # Deployment package
├── Dockerfile               # Build container
├── Cargo.toml
└── Cargo.lock
```
## 🔐 Configuration

This project requires the following environment variable in AWS Lambda:

API Key is acquired by applying to be a [ticketmaster developer](https://developer.ticketmaster.com)

```text
TICKETMASTER_API_KEY=<your_api_key>
```
## 🐳 Build & Package (Docker)
This project uses Docker to ensure a Linux-compatible Lambda binary:

```bash
docker build -t notifier-build .
```
Then extract the compiled binary and zip it:

```bash
# (example – depends on your container workflow)
cp bootstrap output/bootstrap
zip function.zip bootstrap
```

## ☁️ Deployment

1. Upload ```function.zip``` to AWS Lambda
2. Set runtime to **Custom Runtime (provided.al2)**
3. Set handler to: ```bootstrap```
4. Set environment variable: ```TICKETMASTER_API_KEY```
5. Create an **EventBridge schedule** to trigger the function
6. Configure **SNS** email subscription

## 🧠 Design Goals
- Decoupled, event-driven architecture
- Minimal operational complexity
- High reliablilty and fault tolerance
- Deterministic builds and easy redeployments
- Cloud-native scheduling and execution

## 📌 Why This Project Exists

#### Professional/ Techincal:
- Practice **production-style serverless architecture**
- Learn **Rust in real cloud workloads**
- Design **reliable automation systems**
- Implement **cloud-natice scheduling and messaging pipelines**
  
#### Personal/ Real-World Use
- I live near the Grand Casino Areana
- Whenever there is an event it effects travel, costs, and time
- I wanted a way to view events daily to understand how my day will be affected
- Was orginally running on a Raspberry Pi to run locally but that had to be done manually
