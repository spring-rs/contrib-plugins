- [opendal](#spring-opendal)
  [![crates.io](https://img.shields.io/crates/v/spring-opendal.svg)](https://crates.io/crates/spring-opendal)
  [![Documentation](https://docs.rs/spring-opendal/badge.svg)](https://docs.rs/spring-opendal):
  OpenDAL offers a unified data access layer, empowering users to seamlessly and
  efficiently retrieve data from diverse storage services.
- [utoipa](#spring-utoipa)
  [![crates.io](https://img.shields.io/crates/v/spring-utoipa.svg)](https://crates.io/crates/spring-utoipa)
  [![Documentation](https://docs.rs/spring-utoipa/badge.svg)](https://docs.rs/spring-utoipa):
  Utoipa offers compile time generated OpenAPI documentation for Rust.

## Spring OpenDAL

[spring-opendal](spring-opendal) integrates
[Apache OpenDAL™](https://opendal.apache.org/) into spring-rs, providing native
support for all types of storage systems, including object storage services,
file storage services, and many more.

For specific examples, please refer to the
[with-spring-web](https://github.com/spring-rs/contrib-plugins/tree/master/spring-opendal/examples/with-spring-web)
project.

- Run the example

```shell
cargo run --color=always --package spring-opendal --example with-spring-web --features=services-fs
```

- Run the blocking test

```shell
cargo test --test blocking --features="services-memory layers-blocking test-layers" -- --nocapture
```

## Spring utoipa

[spring-utoipa](spring-utoipa) ingegrates
[utoipa](https://github.com/juhaku/utoipa) into spring-web, providing
auto-generated OpenAPI documentation.

For specific examples, please refer to the
[simple](https://github.com/spring-rs/contrib-plugins/tree/master/spring-utoipa/examples/simple),
[with-rapidoc](https://github.com/spring-rs/contrib-plugins/tree/master/spring-utoipa/examples/with-rapidoc),
[with-redoc](https://github.com/spring-rs/contrib-plugins/tree/master/spring-utoipa/examples/with-redoc),
[with-scalar](https://github.com/spring-rs/contrib-plugins/tree/master/spring-utoipa/examples/with-scalar)
or
[with-swagger-ui](https://github.com/spring-rs/contrib-plugins/tree/master/spring-utoipa/examples/with-swagger-ui)
project.

- Run the example

```shell
cargo run --color=always --package spring-utoipa --example with-scalar --features=scalar
```
