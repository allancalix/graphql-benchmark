use criterion::*;

fn async_graphql_parser<'a>(query: &'a str) {
    async_graphql_parser::parse_query(query).unwrap();
}

fn graphql_parser<'a>(query: &'a str) {
    graphql_parser::query::parse_query::<&'a str>(query).unwrap();
}

fn apollo_2_parser(query: &str) {
    let parser = apollo_parser_02::Parser::new(query);
    let tree = parser.parse();

    if tree.errors().len() != 0 {
        panic!("error parsing query: {:?}", tree.errors());
    }
}

fn apollo_parser(query: &str) {
    let parser = apollo_parser::Parser::new(query);
    let tree = parser.parse();

    if tree.errors().len() != 0 {
        panic!("error parsing query: {:?}", tree.errors());
    }
}

fn bench_parser_simple(c: &mut Criterion) {
    let query = "{artists{name,origin,songs{name,duration,likes}},__schema{types{name,fields{name}}}}";

    c.bench_function("async_graphql_parser", move |b| b.iter(|| async_graphql_parser(query)));
    c.bench_function("apollo_graphql_parser", move |b| b.iter(|| apollo_parser(query)));
    c.bench_function("apollo_fork_graphql_parser", move |b| b.iter(|| apollo_2_parser(query)));
    c.bench_function("graphql_parser", move |b| b.iter(|| graphql_parser(query)));
}

fn bench_parser_convoluted(c: &mut Criterion) {
    let query = include_str!("tes.graphql");

    c.bench_function("async_graphql_parser", move |b| b.iter(|| async_graphql_parser(query)));
    c.bench_function("apollo_graphql_parser", move |b| b.iter(|| apollo_parser(query)));
    c.bench_function("apollo_fork_graphql_parser", move |b| b.iter(|| apollo_2_parser(query)));
    c.bench_function("graphql_parser", move |b| b.iter(|| graphql_parser(query)));
}

criterion_group!(benches, bench_parser_simple, bench_parser_convoluted);
criterion_main!(benches);
