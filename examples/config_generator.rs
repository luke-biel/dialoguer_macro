use serde::Serialize;
use std::fs::File;
use std::io::Write;
use yaga::Dialogue;

#[derive(Dialogue, Serialize)]
struct Wrapper {
    #[dialogue(prompt = "Generate some random configuration")]
    config: Config,
    #[dialogue(prompt = "Output filename")]
    output_filename: String,
}

#[derive(Dialogue, Serialize)]
struct Config {
    #[dialogue(prompt = "Ingestion method")]
    ingestion: PrometheusIngestion,
    #[dialogue(prompt = "Rest port")]
    port: u16,
    #[dialogue(prompt = "Allow unauthorized users")]
    force_authentication: bool,
}

#[derive(Dialogue, Serialize)]
enum PrometheusIngestion {
    #[dialogue(prompt = "Configure Telegraf ingestion")]
    Telegraf(TelegrafConfig),
    #[dialogue(prompt = "Configure Elasticsearch ingestion")]
    Elasticsearch(ElasticsearchConfig),
}

#[derive(Dialogue, Serialize)]
struct TelegrafConfig {
    #[dialogue(prompt = "Metrics endpoint")]
    metrics_endpoint: String,
    #[dialogue(prompt = "Scrape frequency (in seconds)")]
    scrape_frequency: u64,
}

#[derive(Dialogue, Serialize)]
struct ElasticsearchConfig {
    #[dialogue(prompt = "Metrics endpoint")]
    metrics_endpoint: String,
    #[dialogue(prompt = "Elasticsearch port")]
    port: u16,
    #[dialogue(prompt = "Scrape frequency (in seconds)")]
    scrape_frequency: u64,
}

fn main() {
    let Wrapper {
        config,
        output_filename,
    } = Wrapper::compose("Compose configuration").unwrap();

    let json = serde_json::to_string_pretty(&config).unwrap();

    let mut file = File::create(&output_filename).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
