use anyhow::*;
use graphql_client::{GraphQLQuery, Response};

type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/query.graphql",
    response_derives = "Debug"
)]
struct Resolver;

fn main() -> Result<(), anyhow::Error> {
    let q = Resolver::build_query(resolver::Variables {
        names: vec![resolver::name {
            supplied_id: None,
            value: "Puma concolor".to_string(),
        }],
        sources: Some(vec![1, 11]),
    });
    let client = reqwest::Client::new();

    let mut res = client
        .post("http://index.globalnames.org/api/graphql")
        .json(&q)
        .send()?;

    let response_body: Response<resolver::ResponseData> = res.json()?;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }
    let response_data: resolver::ResponseData = response_body.data.expect("missing response data");
    println!("{:#?}", response_data);

    Ok(())
}
