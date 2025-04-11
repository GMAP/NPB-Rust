mod common;
use crate::common::print_results::*;
use crate::common::randdp::*;
use crate::common::timers::*;

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::env;
use std::sync::{Arc, Mutex};

#[cfg(class = "S")]
mod params {
    pub const M: u8 = 24;
    pub const CLASS: char = 'S';
    pub const SX_VERIFY_VALUE: f64 = -3.247834652034740e+3;
    pub const SY_VERIFY_VALUE: f64 = -6.958407078382297e+3;
}

#[cfg(class = "W")]
mod params {
    pub const M: u8 = 25;
    pub const CLASS: char = 'W';
    pub const SX_VERIFY_VALUE: f64 = -2.863319731645753e+3;
    pub const SY_VERIFY_VALUE: f64 = -6.320053679109499e+3;
}

#[cfg(class = "A")]
mod params {
    pub const M: u8 = 28;
    pub const CLASS: char = 'A';
    pub const SX_VERIFY_VALUE: f64 = -4.295875165629892e+3;
    pub const SY_VERIFY_VALUE: f64 = -1.580732573678431e+4;
}

#[cfg(class = "B")]
mod params {
    pub const M: u8 = 30;
    pub const CLASS: char = 'B';
    pub const SX_VERIFY_VALUE: f64 = 4.033815542441498e+4;
    pub const SY_VERIFY_VALUE: f64 = -2.660669192809235e+4;
}

#[cfg(class = "C")]
mod params {
    pub const M: u8 = 32;
    pub const CLASS: char = 'C';
    pub const SX_VERIFY_VALUE: f64 = 4.764367927995374e+4;
    pub const SY_VERIFY_VALUE: f64 = -8.084072988043731e+4;
}

#[cfg(class = "D")]
mod params {
    pub const M: u8 = 36;
    pub const CLASS: char = 'D';
    pub const SX_VERIFY_VALUE: f64 = 1.982481200946593e+5;
    pub const SY_VERIFY_VALUE: f64 = -1.020596636361769e+5;
}

#[cfg(class = "E")]
mod params {
    pub const M: u8 = 40;
    pub const CLASS: char = 'E';
    pub const SX_VERIFY_VALUE: f64 = -5.319717441530e+05;
    pub const SY_VERIFY_VALUE: f64 = -3.688834557731e+05;
}

#[cfg(not(any(
    class = "S",
    class = "W",
    class = "A",
    class = "B",
    class = "C",
    class = "D",
    class = "E"
)))]
mod params {
    // Never used
    pub const M: u8 = 16;
    pub const CLASS: char = 'U';
    pub const SX_VERIFY_VALUE: f64 = 1.0;
    pub const SY_VERIFY_VALUE: f64 = 1.0;
    compile_error!(
        "\n\n\
        Must set a class at compilation time by setting RUSTFLAGS\n\
        class options for EP are: {S, W, A, B, C, D, E}\n\
        For example:\n\
        RUSTFLAGS='--cfg class=\"A\" ' cargo build --release --bin ep\n\n\n\
    "
    );
}

#[cfg(safe = "true")]
pub const UNSAFE: bool = false;
#[cfg(not(safe = "true"))]
pub const UNSAFE: bool = true;

#[cfg(timers = "true")]
pub const TIMERS: bool = true;
#[cfg(not(timers = "true"))]
pub const TIMERS: bool = false;

use params::*;

pub const MK: i32 = 16;
pub const MM: i32 = M as i32 - MK;
pub const NN: i32 = 1 << MM;
pub const NK: i32 = 1 << MK;
pub const NQ: usize = 10;
pub const EPSILON: f64 = 1.0e-8;
pub const A: f64 = 1220703125.0;
pub const S: f64 = 271828183.0;
pub const NK_PLUS: usize = (NK << 1) as usize + 1;

/*
* each instance of the main loop may be performed independently. we compute
* the k offsets separately to take into account the fact that some nodes
* have more numbers to generate than others
*/
pub const K_OFFSET: i32 = -1;

/* ep */
fn main() {
    if let Ok(ray_num_threads_str) = env::var("RAY_NUM_THREADS") {
        if let Ok(ray_num_threads) = ray_num_threads_str.parse::<usize>() {
            ThreadPoolBuilder::new()
                .num_threads(ray_num_threads)
                .build_global()
                .unwrap();
        } else {
            ThreadPoolBuilder::new().build_global().unwrap();
        }
    } else {
        ThreadPoolBuilder::new().build_global().unwrap();
    }

    let mut x = Vec::with_capacity(NK_PLUS);

    let (mops, mut t1): (f64, f64);
    let (mut tm, an, mut tt, gc): (f64, f64, f64, f64);
    let np: i32;
    let nit: i32;
    let verified: i8;

    let mut t_aux: f64;

    let timers: Arc<Mutex<Timer>> = Arc::new(Mutex::new(Timer::new()));

    print!("\n\n NAS Parallel Benchmarks 4.1 Parallel Rust version with Rayon - EP Benchmark\n\n");
    print!(
        " Number of random numbers generated: {:>20.0}\n",
        2.0f64.powi(M as i32 + 1)
    );

    /*
     * --------------------------------------------------------------------
     * compute the number of "batches" of random number pairs generated
     * per processor. Adjust if the number of processors does not evenly
     * divide the total number
     * --------------------------------------------------------------------
     */
    np = NN;

    /*
     * call the random number generator functions and initialize
     * the x-array to reduce the effects of paging on the timings.
     * also, call all mathematical functions that are used. make
     * sure these initializations cannot be eliminated as dead code.
     */
    vranlc(0, &mut 1.0, 1.0, &mut vec![1.0]);
    x.resize(NK_PLUS, -1.0e99);
    _ = f64::max(1.0, 1.0).abs().sqrt().ln();

    {
        let mut timers = timers.lock().unwrap();
        timers.clear(0);
        timers.clear(1);
        timers.clear(2);
        timers.start(0);
    }

    t1 = A;
    vranlc(0, &mut t1, A, &mut x);

    /* compute AN = A ^ (2 * NK) (mod 2^46) */
    t1 = A;
    for _ in 0..MK + 1 {
        t_aux = t1;
        randlc(&mut t1, t_aux);
    }

    an = t1;

    let (sx, sy, q) = (1..np + 1)
        .into_par_iter()
        .map(|k| {
            let mut x: Vec<f64> = vec![0.0; NK_PLUS];
            let mut thread_q: Vec<f64> = vec![0.0; NQ];
            let mut thread_sx: f64 = 0.0;
            let mut thread_sy: f64 = 0.0;

            let mut kk: i32 = K_OFFSET + k;
            let mut t1: f64 = S;
            let mut t2: f64 = an;
            let mut t3;

            /* find starting seed t1 for this kk */
            for _ in 1..101 {
                let ik = kk >> 1;
                if (ik << 1) != kk {
                    randlc(&mut t1, t2);
                }

                if ik == 0 {
                    break;
                }
                let t_aux = t2;
                randlc(&mut t2, t_aux);

                kk = ik;
            }

            /* compute uniform pseudorandom numbers */
            if TIMERS {
                if rayon::current_thread_index().unwrap() == 0 {
                    timers.lock().unwrap().start(2);
                }
            }
            vranlc(NK << 1, &mut t1, A, &mut x);
            if TIMERS {
                if rayon::current_thread_index().unwrap() == 0 {
                    timers.lock().unwrap().stop(2);
                }
            }

            /*
             * compute gaussian deviates by acceptance-rejection method and
             * tally counts in concentric square annuli. this loop is not
             * vectorizable.
             */
            if TIMERS {
                if rayon::current_thread_index().unwrap() == 0 {
                    timers.lock().unwrap().start(1);
                }
            }
            for i in 0..NK as usize {
                let x1;
                let x2;
                x1 = 2.0 * x[2 * i] - 1.0;
                x2 = 2.0 * x[2 * i + 1] - 1.0;
                t1 = x1 * x1 + x2 * x2;

                if t1 <= 1.0 {
                    t2 = f64::sqrt(-2.0 * f64::ln(t1) / t1);
                    t3 = x1 * t2;
                    let t4 = x2 * t2;
                    let l = f64::max(t3.abs(), t4.abs()) as usize;

                    thread_q[l] += 1.0;
                    thread_sx = thread_sx + t3;
                    thread_sy = thread_sy + t4;
                }
            }
            if TIMERS {
                if rayon::current_thread_index().unwrap() == 0 {
                    timers.lock().unwrap().stop(1);
                }
            }

            (thread_sx, thread_sy, thread_q)
        })
        .reduce(
            || (0.0, 0.0, vec![0.0; NQ]),
            |(mut acc_sx, mut acc_sy, mut acc_q), (thread_sx, thread_sy, thread_q)| {
                acc_sx += thread_sx;
                acc_sy += thread_sy;
                (0..NQ).into_iter().for_each(|i| acc_q[i] += thread_q[i]);

                (acc_sx, acc_sy, acc_q)
            },
        );

    let mut timers = timers.lock().unwrap();

    gc = q.iter().sum();

    timers.stop(0);
    tm = timers.read(0).as_secs_f64();

    nit = 0;
    verified = if CLASS != 'U' {
        let sx_err = ((sx - SX_VERIFY_VALUE) / SX_VERIFY_VALUE).abs();
        let sy_err = ((sy - SY_VERIFY_VALUE) / SY_VERIFY_VALUE).abs();
        if (sx_err <= EPSILON) && (sy_err <= EPSILON) {
            1
        } else {
            0
        }
    } else {
        -1
    };
    mops = 2.0_f64.powf((M + 1) as f64) / tm / 1000000.0;

    println!("\n EP Benchmark Results:\n");
    println!(" CPU Time ={:>10.4}", tm);
    println!(" N = 2^{:>5}", M as i32);
    println!(" No. Gaussian Pairs = {:>15.0}", gc);
    println!(" Sums = {:>25.15e} {:>25.15e}", sx, sy);
    println!(" Counts: ");
    for i in 0..NQ - 1 {
        println!("{:>3}{:>15.0}", i, q[i]);
    }

    let info = PrintInfo {
        name: String::from("EP"),
        class: CLASS.to_string(),
        size: (usize::pow(2, M as u32 + 1), 0, 0),
        num_iter: nit,
        time: tm,
        mops,
        operation: String::from("Random numbers generated"),
        verified,
        num_threads: rayon::current_num_threads() as u32,
        //uns: UNSAFE
    };
    printer(info);

    if TIMERS {
        if tm <= 0.0 {
            tm = 1.0;
        }
        tt = timers.read(0).as_secs_f64();
        println!("\nTotal time:     {:>9.3} ({:>6.2})", tt, tt * 100.0 / tm);
        tt = timers.read(1).as_secs_f64();
        println!("Gaussian pairs: {:>9.3} ({:>6.2})", tt, tt * 100.0 / tm);
        tt = timers.read(2).as_secs_f64();
        println!("Random numbers: {:>9.3} ({:>6.2})", tt, tt * 100.0 / tm);
    }
}
