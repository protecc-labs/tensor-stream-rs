use anyhow::Result;
use graphql_client::GraphQLQuery;
use tokio_stream::StreamExt;
use url::Url;

extern crate tensor_stream;

use tensor_stream::{
    connect_subscription_client,
    graphql::{
        subscription::BoxedSubscription,
        types::{tswap_order_update_all, TswapOrderUpdateAll},
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let subscription_client =
        connect_subscription_client(Url::parse("wss://api.tensor.so/graphql")?).await?;

    let request_body = TswapOrderUpdateAll::build_query(tswap_order_update_all::Variables {});

    let mut stream: BoxedSubscription<TswapOrderUpdateAll> =
        subscription_client.start::<TswapOrderUpdateAll>(&request_body);

    loop {
        let message = stream.next().await;

        if let Some(Some(response)) = message {
            if let Some(response_data) = response.data {
                println!("{:?}", response_data);
            }
        }
    }
}
