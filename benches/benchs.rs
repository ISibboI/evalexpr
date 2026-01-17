#![feature(test)]
#![cfg(not(tarpaulin_include))]

extern crate rand;
extern crate rand_pcg;
extern crate test;

use evalexpr::{build_operator_tree, combine_trees, DefaultNumericTypes, Operator};
use rand::{distributions::Uniform, seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{fmt::Write, hint::black_box};
use test::Bencher;

const BENCHMARK_LEN: usize = 100_000;
const EXPONENTIAL_TUPLE_ITERATIONS: usize = 12;

fn generate_expression<Gen: Rng>(len: usize, gen: &mut Gen) -> String {
    let int_distribution = Uniform::new_inclusive(1, 100);
    let whitespaces = [" ", "", "", "  ", " \n", "       "];
    let operators = ["+", "-", "*", "/", "%", "^"];
    let mut result = String::new();
    write!(result, "{}", gen.sample(int_distribution)).unwrap();

    while result.len() < len {
        result.push_str(whitespaces.choose(gen).unwrap());
        result.push_str(operators.choose(gen).unwrap());
        result.push_str(whitespaces.choose(gen).unwrap());
        write!(result, "{}", gen.sample(int_distribution)).unwrap();
    }

    result
}

fn generate_expression_chain<Gen: Rng>(len: usize, gen: &mut Gen) -> String {
    let mut chain = generate_expression(10, gen);
    while chain.len() < len {
        chain.push_str("; ");
        chain.push_str(&generate_expression(10, gen));
    }
    chain
}

fn generate_small_expressions<Gen: Rng>(len: usize, gen: &mut Gen) -> Vec<String> {
    let mut result = Vec::new();
    let mut result_len = 0;
    while result_len < len {
        let expression = generate_expression(10, gen);
        result_len += expression.len();
        result.push(expression);
    }
    result
}

fn generate_large_tuple_expression<Gen: Rng>(iterations: usize, gen: &mut Gen) -> String {
    let mut result = String::from("a=(");
    result.push_str(&generate_expression(0, gen));
    result.push(',');
    result.push_str(&generate_expression(0, gen));
    result.push(')');
    for _ in 0..iterations {
        result.push_str(";a=(a,a)")
    }
    dbg!(result)
}

#[bench]
fn bench_parse_long_expression_chains(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(0);
    let long_expression_chain = generate_expression_chain(BENCHMARK_LEN, &mut gen);

    bencher.iter(|| build_operator_tree::<DefaultNumericTypes>(&long_expression_chain).unwrap());
}

#[bench]
fn bench_parse_deep_expression_trees(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(15);
    let deep_expression_tree = generate_expression(BENCHMARK_LEN, &mut gen);

    bencher.iter(|| build_operator_tree::<DefaultNumericTypes>(&deep_expression_tree).unwrap());
}

#[bench]
fn bench_parse_many_small_expressions(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(33);
    let small_expressions = generate_small_expressions(BENCHMARK_LEN, &mut gen);

    bencher.iter(|| {
        for expression in &small_expressions {
            black_box(build_operator_tree::<DefaultNumericTypes>(expression).unwrap());
        }
    });
}

#[bench]
fn bench_evaluate_long_expression_chains(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(0);
    let long_expression_chain = build_operator_tree::<DefaultNumericTypes>(
        &generate_expression_chain(BENCHMARK_LEN, &mut gen),
    )
    .unwrap();

    bencher.iter(|| long_expression_chain.eval().unwrap());
}

#[bench]
fn bench_evaluate_deep_expression_trees(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(15);
    let deep_expression_tree =
        build_operator_tree::<DefaultNumericTypes>(&generate_expression(BENCHMARK_LEN, &mut gen))
            .unwrap();

    bencher.iter(|| deep_expression_tree.eval().unwrap());
}

#[bench]
fn bench_evaluate_many_small_expressions(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(33);
    let small_expressions: Vec<_> = generate_small_expressions(BENCHMARK_LEN, &mut gen)
        .iter()
        .map(|expression| build_operator_tree::<DefaultNumericTypes>(expression).unwrap())
        .collect();

    bencher.iter(|| {
        for expression in &small_expressions {
            black_box(expression.eval().unwrap());
        }
    });
}

#[bench]
fn bench_evaluate_large_tuple_expression(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(44);
    let large_tuple_expression = build_operator_tree::<DefaultNumericTypes>(
        &generate_large_tuple_expression(EXPONENTIAL_TUPLE_ITERATIONS, &mut gen),
    )
    .unwrap();
    dbg!(&large_tuple_expression);

    bencher.iter(|| large_tuple_expression.eval().unwrap());
}

#[bench]
fn bench_tree_combining(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(33);
    let small_expressions: Vec<_> = generate_small_expressions(BENCHMARK_LEN, &mut gen)
        .iter()
        .zip(&generate_small_expressions(BENCHMARK_LEN, &mut gen))
        .map(|(expression_a, expression_b)| {
            (
                build_operator_tree::<DefaultNumericTypes>(expression_a).unwrap(),
                build_operator_tree::<DefaultNumericTypes>(expression_b).unwrap(),
            )
        })
        .collect();

    bencher.iter(|| {
        for expression in &small_expressions {
            black_box(
                combine_trees(expression.0.clone(), expression.1.clone(), Operator::Add).unwrap(),
            );
        }
    });
}
