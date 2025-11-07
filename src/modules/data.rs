use crate::{client::AfricasTalkingClient, error::Result};
use serde::{Deserialize, Serialize};

/**
 * Data module struct to contain props.
 */
#[derive(Debug, Clone)]
pub struct DataModule {
    client: AfricasTalkingClient,
}

/**
 * Contains transaction metadata for a recipient/data item.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub happy: String,
    pub name: String,
}

/**
 * Represents a phone number and how much data to allocate to it.
 * @phone_number: The recipient's phone number.
 * @quantity: The amount of data to allocate.
 * @unit: The unit of data (e.g., MB, GB).
 * @validity: The validity period of the data (e.g., 1 day, 7 days).
 * @metadata: Additional metadata associated with the transaction.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    #[serde(rename = "phoneNumber")]
    phone_number: String,
    quantity: String,
    unit: String,
    validity: String,
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataRequestObj {
    pub username: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    pub recipients: Vec<Recipient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientResponseObj {
    #[serde(rename = "phoneNumber")]
    phone_number: String,
    provicder: String,
    status: String,
    #[serde(rename = "transactionId")]
    transaction_id: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct DataResponseObj {
    entries: Vec<RecipientResponseObj>,
}

/**
 * Handles data module business logic.
 */
impl DataModule {
    pub(crate) fn new(client: AfricasTalkingClient) -> Self {
        Self { client }
    }

    pub async fn request_data(&self, recipients: Vec<Recipient>) -> Result<DataResponseObj> {
        let request = DataRequestObj {
            username: self.client.config.username.clone(),
            product_name: "Mobile Data".to_string(),
            recipients,
        };

        self.client.post("/version1/data/request", &request).await?
    }
}
