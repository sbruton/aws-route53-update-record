# aws-route53-update-record

## Building
* Install the [Rust](https://rust-lang.org) toolchain
```sh
git clone https://github.com/sbruton/aws-route53-update-record.git
cd aws-route53-update-record
cargo build
```

## Usage
```sh
aws-route53-update-record --zone SOME-HOSTED-ZONE-ID --hostname some.host.tld --ip 1.2.3.4
```
