#![feature(test)]

extern crate test;

mod macros;

bench_clone_static!(bench_clone_static_00, 0, |fixture| fixture);
bench_clone_static!(bench_clone_static_01, 1, |fixture| fixture);
bench_clone_static!(bench_clone_static_02, 2, |fixture| fixture);
bench_clone_static!(bench_clone_static_03, 3, |fixture| fixture);
bench_clone_static!(bench_clone_static_04, 4, |fixture| fixture);
bench_clone_static!(bench_clone_static_05, 5, |fixture| fixture);
bench_clone_static!(bench_clone_static_06, 6, |fixture| fixture);
bench_clone_static!(bench_clone_static_07, 7, |fixture| fixture);
bench_clone_static!(bench_clone_static_08, 8, |fixture| fixture);
bench_clone_static!(bench_clone_static_09, 9, |fixture| fixture);
bench_clone_static!(bench_clone_static_10, 10, |fixture| fixture);
bench_clone_static!(bench_clone_static_11, 11, |fixture| fixture);
bench_clone_static!(bench_clone_static_12, 12, |fixture| fixture);
bench_clone_static!(bench_clone_static_13, 13, |fixture| fixture);

bench_clone_ref!(bench_clone_ref_00, 0, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_01, 1, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_02, 2, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_03, 3, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_04, 4, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_05, 5, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_06, 6, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_07, 7, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_08, 8, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_09, 9, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_10, 10, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_11, 11, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_12, 12, |fixture| fixture);
bench_clone_ref!(bench_clone_ref_13, 13, |fixture| fixture);

bench_eq_static!(bench_eq_static_00, 0, |fixture| fixture);
bench_eq_static!(bench_eq_static_01, 1, |fixture| fixture);
bench_eq_static!(bench_eq_static_02, 2, |fixture| fixture);
bench_eq_static!(bench_eq_static_03, 3, |fixture| fixture);
bench_eq_static!(bench_eq_static_04, 4, |fixture| fixture);
bench_eq_static!(bench_eq_static_05, 5, |fixture| fixture);
bench_eq_static!(bench_eq_static_06, 6, |fixture| fixture);
bench_eq_static!(bench_eq_static_07, 7, |fixture| fixture);
bench_eq_static!(bench_eq_static_08, 8, |fixture| fixture);
bench_eq_static!(bench_eq_static_09, 9, |fixture| fixture);
bench_eq_static!(bench_eq_static_10, 10, |fixture| fixture);
bench_eq_static!(bench_eq_static_11, 11, |fixture| fixture);
bench_eq_static!(bench_eq_static_12, 12, |fixture| fixture);
bench_eq_static!(bench_eq_static_13, 13, |fixture| fixture);

bench_eq_ref!(bench_eq_ref_00, 0, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_01, 1, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_02, 2, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_03, 3, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_04, 4, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_05, 5, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_06, 6, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_07, 7, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_08, 8, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_09, 9, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_10, 10, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_11, 11, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_12, 12, |fixture| fixture);
bench_eq_ref!(bench_eq_ref_13, 13, |fixture| fixture);
