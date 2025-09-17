//!
//! /Users/noah/My/Rust/rust_sandbox/saha_sinha_pi/src/main.rs
//!
//! 標準ライブラリだけ で動く、シンプルな Rust 実装です。級数の式は Saha–Sinha によるもの。
//!
//! 出典：MathOverflow の要約ページ
//!     新しい円周率の級数についての質問とその導出の数式要約
//!     https://mathoverflow.net/questions/473931/possible-new-series-for-pi?utm_source=chatgpt.com
//!
//! How to use:
//!     cd /Users/noah/My/Rust/rust_sandbox/saha_sinha_pi/
//!     cargo build
//!     cargo run
//!

use std::f64::consts::PI;

/// n! を f64 で計算（小さめの n を想定）
fn factorial(n: u32) -> f64 {
    (1..=n).fold(1.0, |acc, k| acc * (k as f64))
}

/// ポックハマー記号 (a)_{m} = a(a+1)...(a+m-1)
fn pochhammer(a: f64, m: u32) -> f64 {
    (0..m).fold(1.0, |acc, k| acc * (a + k as f64))
}

/// Saha–Sinha の π 級数
/// π = 4 + sum_{n=1..∞} [ 1/n! * ( 1/(n+λ) - 4/(2n+1) ) * ( ((2n+1)^2)/(4(n+λ)) - n )_{n-1} ]
fn pi_saha_sinha(lambda: f64, max_terms: u32, eps: f64) -> (f64, u32) {
    let mut sum = 0.0;
    let mut used = 0;

    for n in 1..=max_terms {
        let nf = n as f64;

        let a = (1.0 / (nf + lambda)) - (4.0 / (2.0 * nf + 1.0));
        let b = ((2.0 * nf + 1.0).powi(2)) / (4.0 * (nf + lambda)) - nf;

        let poch = if n == 0 { 1.0 } else { pochhammer(b, n - 1) };
        let term = (1.0 / factorial(n)) * a * poch;

        sum += term;
        used = n;

        if term.is_nan() || term.is_infinite() {
            break;
        }
        if term.abs() < eps {
            break;
        }
    }

    (4.0 + sum, used)
}

fn main() {
    // λ は収束性を左右するパラメータ。まずは 10.0 を試す
    let lambda = 10.0;
    // 打ち切り上限、しきい値
    let max_terms = 60;
    let eps = 1e-16;

    let (approx, used) = pi_saha_sinha(lambda, max_terms, eps);

    println!("Saha–Sinha series with λ={lambda}");
    println!("  π ≈ {:.17}", approx);
    println!("  terms used: {}", used);
    println!("  |error|    : {:.3e}", (approx - PI).abs());

    // いくつか λ を変えて比較したい場合
    for &lam in &[3.0_f64, 5.0, 10.0, 20.0] {
        let (v, k) = pi_saha_sinha(lam, max_terms, eps);
        println!("λ={:<4} -> π≈{:.15}  terms={}  |err|={:.3e}", lam, v, k, (v - PI).abs());
    }
}

