#![doc(issue_tracker_base_url = "https://github.com/sbruton/aws-route53-update-record/issues/")]
#![deny(
    missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications, unused_variables, unreachable_code, unused_comparisons, unused_imports,
    unused_must_use
)]

//! Updates an A record hosted on AWS Route53 with the specified IP address

#[macro_use]
extern crate clap;
extern crate rusoto_core;
extern crate rusoto_route53;

use clap::App;
use rusoto_core::Region;
use rusoto_route53::{
    Change, ChangeBatch, ChangeResourceRecordSetsRequest, ResourceRecord, ResourceRecordSet,
    Route53, Route53Client,
};

fn main() {
    // load the cli arg parsing configuration
    let yaml = load_yaml!("cli.yml");
    // parse cli arguments
    let matches = App::from_yaml(yaml).get_matches();
    // get our DNS record FQDN, safe to unwrap since marked required in cli config
    let hostname = String::from(matches.value_of("hostname").unwrap());
    // get our IP address, safe to unwrap since marked required in cli config
    let ip = String::from(matches.value_of("ip").unwrap());
    // get our hosed zone id, safe to unwarp since marked required in cli config
    let hosted_zone_id = String::from(matches.value_of("hosted_zone_id").unwrap());
    // get route53 client for US-East-1 (R53 is global, so region is only for API endpoint)
    let client = Route53Client::simple(Region::UsEast1);
    // create our change batch
    let change_batch = ChangeBatch {
        comment: None,
        changes: vec![Change {
            action: String::from("UPSERT"),
            resource_record_set: ResourceRecordSet {
                alias_target: None,
                geo_location: None,
                failover: None,
                health_check_id: None,
                multi_value_answer: None,
                region: None,
                set_identifier: None,
                traffic_policy_instance_id: None,
                ttl: Some(300),
                weight: None,
                name: hostname,
                type_: String::from("A"),
                resource_records: Some(vec![ResourceRecord { value: ip }]),
            },
        }],
    };
    // envelope change batch addressed to hosted zone id
    let change_request = ChangeResourceRecordSetsRequest {
        hosted_zone_id,
        change_batch,
    };
    // implement the change
    match client.change_resource_record_sets(&change_request).sync() {
        Ok(output) => println!("{:?}", output),
        Err(err) => eprintln!("ERR: {:?}", err),
    };
}
