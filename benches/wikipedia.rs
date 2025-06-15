use criterion::{criterion_group, criterion_main, Criterion};
use kuchikiki::traits::*;
use std::{hint::black_box, iter, time::Duration};

use lightningcss::{
	printer::PrinterOptions,
	rules::CssRule,
	stylesheet::{ParserOptions, StyleSheet},
	traits::ToCss,
};

fn rust_wikipedia(c: &mut Criterion) {
	let html = include_str!("../test_data/rust_wikipedia.html");
	let css = include_str!("../test_data/rust_wikipedia.css");

	let stylesheet = StyleSheet::parse(css, ParserOptions::default()).unwrap();
	let selectors: Vec<String> =
		stylesheet
			.rules
			.0
			.iter()
			.flat_map(|rule| -> Box<dyn Iterator<Item = String>> {
				match rule {
					CssRule::Style(style) => Box::new(style.selectors.0.iter().map(|selector| {
						selector.to_css_string(PrinterOptions::default()).unwrap()
					})),
					_ => Box::new(iter::empty::<String>()),
				}
			})
			.collect();

	c.bench_function("select", move |b| {
		b.iter(|| {
			let document = kuchikiki::parse_html().one(black_box(html));

			for selector in black_box(&selectors) {
				match document.select(selector) {
					Ok(iter) => {
						for item in iter {
							black_box(item);
						}
					}
					Err(_) => {}
				}
			}
		})
	});
}

criterion_group! {
	name = benches;
	config = Criterion::default().measurement_time(Duration::from_secs(25));
	targets = rust_wikipedia
}
criterion_main!(benches);
