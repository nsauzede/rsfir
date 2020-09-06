use core::f64::consts::PI;

fn w(f: f64) -> f64 {
    2. * PI * f
}

fn s(a: f64, w: f64, t: f64, p: f64) -> f64 {
    a * f64::sin(w * t + p)
}

fn main() {
    let mut t = 0.;
    let (fa, fb, fc) = (400., 150., 30.);
    let (aa, ab, ac) = (2., 1., 0.8);
    let (pa, pb, pc) = (0., 0., 0.4);
    let taps = [
        0.00365183969,
        0.004210430471,
        0.01093157989,
        0.006581967296,
        -0.008013225645,
        -0.02522248066,
        -0.03471236554,
        -0.0272794953,
        -0.002839227321,
        0.02573920741,
        0.04044389884,
        0.03256605667,
        0.01112466844,
        -0.003294306054,
        0.00382161187,
        0.02572949982,
        0.03608495343,
        0.009443684783,
        -0.05381141657,
        -0.1193094492,
        -0.1378136762,
        -0.08072093534,
        0.03403087491,
        0.1472212013,
        0.1942009684,
        0.1472212013,
        0.03403087491,
        -0.08072093534,
        -0.1378136762,
        -0.1193094492,
        -0.05381141657,
        0.009443684783,
        0.03608495343,
        0.02572949982,
        0.00382161187,
        -0.003294306054,
        0.01112466844,
        0.03256605667,
        0.04044389884,
        0.02573920741,
        -0.002839227321,
        -0.0272794953,
        -0.03471236554,
        -0.02522248066,
        -0.008013225645,
        0.006581967296,
        0.01093157989,
        0.004210430471,
        0.00365183969,
    ];

    let (wa, wb, wc) = (w(fa), w(fb), w(fc));
    let mut samples = vec![0.; taps.len()];
    for i in 5 - 5..=1001 - 5 {
        let sa = s(aa, wa, t, pa);
        let sb = s(ab, wb, t, pb);
        let sc = s(ac, wc, t, pc);
        let s = sa + sb + sc;
        let mut last = 0.;
        samples = samples[1..].to_vec();
        samples.push(s);
        if i >= taps.len() {
            last = taps.iter().zip(&samples).map(|(t, s)| t * s).sum();
        }
        println!(
            "t={:.4} sA={:.9} sB={:.10} sC={:.10} s={:.10} res={}",
            t, sa, sb, sc, s, last
        );
        t += 0.0005;
    }
}
