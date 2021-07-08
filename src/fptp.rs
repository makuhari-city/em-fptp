use async_trait::async_trait;
use serde_json::Value;
use vote::{AggregationRule, VoteData};
use std:::collections::HashMap;
use uuid::Uuid;

struct FPTP {}

#[async_trait]
impl AggregationRule for FPTP {
    async fn calculate(data: VoteData) -> Value {
        let mut result: HashMap<Uuid, u32> = HashMap::new();
        let stripped = info.only_policy_voting();
        for (_, vote) in stripped {
            let max = vote.iter().reduce(|a, b| if a.1 > b.1 { a } else { b });
            if max.is_some() {
                let (to, _) = max.unwrap();
                *result.entry(to.to_owned()).or_insert(0) += 1;
            }
        }

        let mut sorted = result.into_iter().collect::<Vec<(Uuid, u32)>>();

        sorted.sort_by(|(_, a), (_, b)| b.cmp(a));

        serde_json::to_value(sorted).expect("this should be serializable")
    }
}
