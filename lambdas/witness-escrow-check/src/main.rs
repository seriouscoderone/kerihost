//! Lambda handler for scheduled escrow processing
//!
//! This handler is triggered by CloudWatch Events to process escrowed events.
//! It checks if escrow conditions are now satisfied and promotes events accordingly.

use aws_lambda_events::cloudwatch_events::CloudWatchEvent;
use kerihost_db::DynamoDbDatabase;
use kerihost_witness::{Witness, WitnessConfig};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::{info, warn, error};

/// Global witness instance
static WITNESS: OnceCell<Witness<DynamoDbDatabase>> = OnceCell::const_new();

/// Initialize the witness
async fn init_witness() -> Witness<DynamoDbDatabase> {
    let db = Arc::new(DynamoDbDatabase::from_env().await);
    let config = WitnessConfig::from_env();
    Witness::new(None, db, config)
}

/// Get or initialize the witness
async fn get_witness() -> &'static Witness<DynamoDbDatabase> {
    WITNESS.get_or_init(init_witness).await
}

/// Lambda handler
async fn handler(
    _event: LambdaEvent<CloudWatchEvent>,
) -> Result<(), Error> {
    let witness = get_witness().await;

    info!("Starting escrow check");

    // Get all escrowed events
    let escrowed = match witness.get_all_escrowed().await {
        Ok(e) => e,
        Err(e) => {
            error!(error = %e, "Failed to get escrowed events");
            return Err(e.into());
        }
    };

    info!(count = escrowed.len(), "Found escrowed events");

    let mut promoted = 0;
    let mut expired = 0;
    let mut kept = 0;

    for item in escrowed {
        // Check if expired
        if item.is_expired() {
            info!(
                digest = %item.event.event.digest,
                reason = %item.reason,
                "Removing expired escrow"
            );

            if let Err(e) = witness.remove_escrowed(&item.event.event.digest).await {
                warn!(error = %e, "Failed to remove expired escrow");
            } else {
                expired += 1;
            }
            continue;
        }

        // Check if can be promoted
        match witness.can_promote(&item).await {
            Ok(true) => {
                info!(
                    digest = %item.event.event.digest,
                    reason = %item.reason,
                    "Promoting escrowed event"
                );

                match witness.promote_escrowed(&item).await {
                    Ok(_) => {
                        promoted += 1;
                    }
                    Err(e) => {
                        warn!(
                            error = %e,
                            digest = %item.event.event.digest,
                            "Failed to promote escrowed event"
                        );
                    }
                }
            }
            Ok(false) => {
                kept += 1;
            }
            Err(e) => {
                warn!(
                    error = %e,
                    digest = %item.event.event.digest,
                    "Failed to check escrow promotion"
                );
                kept += 1;
            }
        }
    }

    info!(
        promoted = promoted,
        expired = expired,
        kept = kept,
        "Escrow check completed"
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .json()
        .init();

    lambda_runtime::run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
    // Unit tests are limited since this is primarily integration logic
}
