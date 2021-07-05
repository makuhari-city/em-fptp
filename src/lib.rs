use std::collections::HashMap;
use uuid::Uuid;
use vote::VoteData;

type FPTPResult = Vec<(Uuid, u32)>;

pub async fn calculate(info: &VoteData) -> FPTPResult {
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

    sorted
}

#[cfg(test)]
mod fptp_test {

    use super::*;

    #[actix_rt::test]
    async fn simple() {
        let json_data = br#"{
  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ]
    ],
    "cc652ec5-0a11-48da-9189-4642473bb54e": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ]
    ],
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": [
      [
        "a076bf38-55b3-42c0-8cd5-d89381152e10",
        1
      ]
    ]
  },
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e",
  ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(&info).await;
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }

    #[actix_rt::test]
    async fn dont_include_delegates() {
        let json_data = br#"{
  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ]
    ],
    "cc652ec5-0a11-48da-9189-4642473bb54e": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ],
      [
        "046c12e1-906a-492f-8614-39dfa87d676d",
        1
      ]

    ],
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": [
      [
        "a076bf38-55b3-42c0-8cd5-d89381152e10",
        1
      ],
      [
        "046c12e1-906a-492f-8614-39dfa87d676d",
        1
      ]

    ]
  },
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e",
    ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(&info).await;
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }

    #[actix_rt::test]
    async fn multiple() {
        let json_data = br#"{
  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        0.8
      ],
      [
        "a076bf38-55b3-42c0-8cd5-d89381152e10",
        0.7
      ]
    ],
    "cc652ec5-0a11-48da-9189-4642473bb54e": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ]
    ],
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": [
      [
        "a076bf38-55b3-42c0-8cd5-d89381152e10",
        1
      ]
    ]
  },
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e",
  ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(&info).await;
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }

    #[actix_rt::test]
    async fn empty() {
        let json_data = br#"{

  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {},
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e",
  ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(&info).await;
        assert!(result.is_empty());
    }

    #[actix_rt::test]
    async fn empty_delegates() {
        let json_data = br#"{

  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9":[
        ["cc652ec5-0a11-48da-9189-4642473bb54e", 1.0]
    ]
  },
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e",
      ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(&info).await;
        assert!(result.is_empty());
    }
}
