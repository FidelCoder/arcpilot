use anyhow::Result;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use tracing::{debug, error};
use std::time::Duration;

use crate::opportunities::Opportunity;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self {
            producer,
            topic: "opportunities".to_string(),
        })
    }

    pub async fn publish_opportunity(&self, opportunity: &Opportunity) -> Result<()> {
        let payload = serde_json::to_string(opportunity)?;

        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&opportunity.id);

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => {
                debug!("Published opportunity {} to Kafka", opportunity.id);
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to publish opportunity to Kafka: {}", e);
                Err(anyhow::anyhow!("Kafka publish error: {}", e))
            }
        }
    }
}

