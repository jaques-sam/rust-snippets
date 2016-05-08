// #![feature(test)]
// extern crate uap_rust;
// extern crate woothee;
// use uap_rust::parser::Parser;
// use woothee::parse;
//
// pub fn b_uap() {
//    let parser = Parser::new().unwrap();
//    let _ = parser.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)"
//                             .to_string());
//    let _ = parser.parse("Twitterbot/1.0".to_string());
//    let _ = parser.parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)"
//                             .to_string());
// }
//
// pub fn b_woothee() {
//    let _ = parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)");
//    let _ = parse("Twitterbot/1.0");
//    let _ = parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)");
// }
//
// extern crate test;
// use test::Bencher;
// #[bench]
// fn bench_uap(b: &mut Bencher) {
//    b.iter(|| b_uap());
// }
//
// #[bench]
// fn bench_woothee(b: &mut Bencher) {
//    b.iter(|| b_woothee());
// }
