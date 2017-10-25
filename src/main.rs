// Attempt to estimate entropy obtained from system timers

extern crate gnuplot;

fn get_input() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64
}

const LOW_BITS: usize = 10;
const DIFF_BITS: usize = 10;
const LOW_DIFF: usize = 1 << 5;

const N_LOW_BITS: usize = 1 << LOW_BITS;
const LOW_MASK: i64 = (N_LOW_BITS - 1) as i64;

const MAX_DIFF: usize = 1 << DIFF_BITS;

struct DataOut {
    low_bits: [u32; N_LOW_BITS],
    diffs: [u32; MAX_DIFF],
}

fn collect() -> DataOut {
    let mut out = DataOut { low_bits: [0; N_LOW_BITS], diffs: [0; MAX_DIFF] };
    
    let mut ignored = 0u32;
    let mut last = get_input() as i64;
    for _ in 0..100000 {
        let sample = get_input() as i64;
        
        out.low_bits[(sample & LOW_MASK) as usize] += 1;
        
        let diff = sample - last;
        if diff < MAX_DIFF as i64 {
            out.diffs[diff as usize] += 1;
        } else {
            ignored += 1;
        }
        
        last = sample;
    }
    
    println!("Diffs over maximum: {}", ignored);
    
    out
}

use gnuplot::{Figure, AxesCommon, Caption, AutoOption, DataType};

fn plot_hist<I: IntoIterator>(n: usize, iter: I, caption: &str)
    where <I as IntoIterator>::Item: DataType
{
    let mut fg = Figure::new();
    fg.axes2d()
        .set_y_range(AutoOption::Fix(0.0), AutoOption::Auto)
        .boxes(0..n, iter, &[Caption(caption)]);
    fg.show();
}

fn main() {
    let data = collect();
    
    // low bits are well distributed, so presumably no excessive rounding
//     plot_hist(N_LOW_BITS, data.low_bits.iter(), "low bits");
    
    // diffs are *not at all* well distributed, so not so much entropy available!
    // on my system, most calls are either 175 or 200 ns; does this imply little
    // more than 1 bit per call?
    plot_hist(MAX_DIFF, data.diffs.iter(), "diffs");
    
    // looking at the low bits of the diff, there may be 2-4 bits of entropy,
    // but this is hard to estimate due to bias
    let mut low_diff = [0u32; LOW_DIFF];
    for i in 0..MAX_DIFF {
        low_diff[i & (LOW_DIFF - 1)] += data.diffs[i];
    }
    plot_hist(LOW_DIFF, low_diff.iter(), "low diff bits");
    
    let mut diff_bit_freq = [0u32; DIFF_BITS];
    let mut total = 0;
    for i in 0..MAX_DIFF {
        let n = data.diffs[i];
        if n > 0 {
            for bit in 0..DIFF_BITS {
                if (i >> bit) & 1 == 1 {
                    diff_bit_freq[bit] += n;
                }
            }
            total += n;
        }
    }
    let mut diff_bit_p = [0.0f64; DIFF_BITS];
    let mut diff_bit_entropy = [0.0f64; DIFF_BITS];
    for bit in 0..DIFF_BITS {
        let p = diff_bit_freq[bit] as f64 / total as f64;
        
        diff_bit_p[bit] = p;
        diff_bit_entropy[bit] = if p == 0.0 || p == 1.0 {
            0.0
        } else { 
            // Shannon entropy:
            p * -p.log2() +
            (1.0 - p) * -(1.0 - p).log2()
        };
    }
    println!("low diff bits:");
    println!("freq:\t{:?}", diff_bit_freq);
    println!("p:\t{:?}", diff_bit_p);
    println!("entropy:\t{:?}", diff_bit_entropy);
    println!("Total entropy: {}", diff_bit_entropy.iter().sum::<f64>());
}
