mod common;
use crate::common::print_results::*;
use crate::common::randdp::*;
use crate::common::timers::*;

#[cfg(class = "S")]
mod params {
    pub const CLASS: char = 'S';
    pub const NA: i32 = 1400;
    pub const NONZER: i32 = 7;
    pub const NITER: i32 = 15;
    pub const SHIFT: f64 = 10.0;
    pub const ZETA_VERIFY: f64 = 8.5971775078648;
}

#[cfg(class = "W")]
mod params {
    pub const CLASS: char = 'W';
    pub const NA: i32 = 7000;
    pub const NONZER: i32 = 8;
    pub const NITER: i32 = 15;
    pub const SHIFT: f64 = 12.0;
    pub const ZETA_VERIFY: f64 = 10.362595087124;
}

#[cfg(class = "A")]
mod params {
    pub const CLASS: char = 'A';
    pub const NA: i32 = 14000;
    pub const NONZER: i32 = 11;
    pub const NITER: i32 = 15;
    pub const SHIFT: f64 = 20.0;
    pub const ZETA_VERIFY: f64 = 17.130235054029;
}

#[cfg(class = "B")]
mod params {
    pub const CLASS: char = 'B';
    pub const NA: i32 = 75000;
    pub const NONZER: i32 = 13;
    pub const NITER: i32 = 75;
    pub const SHIFT: f64 = 60.0;
    pub const ZETA_VERIFY: f64 = 22.712745482631;
}

#[cfg(class = "C")]
mod params {
    pub const CLASS: char = 'C';
    pub const NA: i32 = 150000;
    pub const NONZER: i32 = 15;
    pub const NITER: i32 = 75;
    pub const SHIFT: f64 = 110.0;
    pub const ZETA_VERIFY: f64 = 28.973605592845;
}

#[cfg(class = "D")]
mod params {
    pub const CLASS: char = 'D';
    pub const NA: i32 = 1500000;
    pub const NONZER: i32 = 21;
    pub const NITER: i32 = 100;
    pub const SHIFT: f64 = 500.0;
    pub const ZETA_VERIFY: f64 = 52.514532105794;
}

#[cfg(class = "E")]
mod params {
    pub const CLASS: char = 'E';
    pub const NA: i32 = 9000000;
    pub const NONZER: i32 = 26;
    pub const NITER: i32 = 100;
    pub const SHIFT: f64 = 1500.0;
    pub const ZETA_VERIFY: f64 = 77.522164599383;
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
    pub const CLASS: char = 'U';
    pub const NA: i32 = 1;
    pub const NONZER: i32 = 1;
    pub const NITER: i32 = 1;
    pub const SHIFT: f64 = 1.0;
    pub const ZETA_VERIFY: f64 = 1.0;
    compile_error!(
        "\n\n\
        Must set a class at compilation time by setting RUSTFLAGS\n\
        class options for CG are: {S, W, A, B, C, D, E}\n\
        For example:\n\
        RUSTFLAGS='--cfg class=\"A\"' cargo build --release --bin cg'\n\n\n\
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

/*
 * ---------------------------------------------------------------------
 * note: please observe that in the routine conj_grad three
 * implementations of the sparse matrix-vector multiply have
 * been supplied. the default matrix-vector multiply is not
 * loop unrolled. the alternate implementations are unrolled
 * to a depth of 2 and unrolled to a depth of 8. please
 * experiment with these to find the fastest for your particular
 * architecture. if reporting timing results, any of these three may
 * be used without penalty.
 * ---------------------------------------------------------------------
*/

pub const NZ: usize = NA as usize * (NONZER + 1) as usize * (NONZER + 1) as usize;
pub const NAZ: i32 = NA * (NONZER + 1);
pub const T_INIT: usize = 0;
pub const T_BENCH: usize = 1;
pub const T_CONJ_GRAD: usize = 2;
pub const T_LAST: usize = 3;
pub const FIRSTROW: i32 = 0;
pub const LASTROW: i32 = NA - 1;
pub const FIRSTCOL: i32 = 0;
pub const LASTCOL: i32 = NA - 1;

pub const EPSILON: f64 = 1.0e-10;
pub const AMULT: f64 = 1220703125.0;

/* cg */
fn main() {
    let mut colidx: Vec<i32> = vec![0; NZ];
    let mut rowstr: Vec<i32> = vec![0; (NA + 1) as usize];
    let mut iv: Vec<i32> = vec![0; NA as usize];
    let mut arow: Vec<i32> = vec![0; NA as usize];
    let mut acol: Vec<i32> = vec![0; NAZ as usize];
    let mut aelt: Vec<f64> = vec![0.0; NAZ as usize];
    let mut a: Vec<f64> = vec![0.0; NZ];
    let mut x: Vec<f64> = vec![0.0; NA as usize + 2];
    let mut z: Vec<f64> = vec![0.0; NA as usize + 2];
    let mut p: Vec<f64> = vec![0.0; NA as usize + 2];
    let mut q: Vec<f64> = vec![0.0; NA as usize + 2];
    let mut r: Vec<f64> = vec![0.0; NA as usize + 2];

    let naa: i32 = NA;
    let nzz: usize = NZ;
    let mut tran: f64;

    let mut zeta: f64;
    let mut rnorm: f64 = 0.0;
    let (mut norm_temp1, mut norm_temp2): (f64, f64);
    let (mut t, mops, mut tmax): (f64, f64, f64);
    let verified: i8;

    let mut timers = Timer::new();
    timers.clear(T_INIT);
    timers.clear(T_BENCH);
    timers.clear(T_CONJ_GRAD);

    timers.start(T_INIT);

    print!("\n\n NAS Parallel Benchmarks 4.1 Serial Rust version - CG Benchmark\n\n");
    print!(" Size: {:>11}\n", NA);
    print!(" Iterations: {:>5}\n", NITER);

    /* initialize random number generator */
    tran = 314159265.0;
    zeta = randlc(&mut tran, AMULT);

    makea(
        naa,
        nzz,
        &mut a[..],
        &mut colidx[..],
        &mut rowstr[..],
        &mut arow[..],
        &mut acol[..],
        &mut aelt[..],
        &mut iv[..],
        &mut tran,
    );

    /*
     * ---------------------------------------------------------------------
     * note: as a result of the above call to makea:
     * values of j used in indexing rowstr go from 0 --> lastrow-firstrow
     * values of colidx which are col indexes go from firstcol --> lastcol
     * so:
     * shift the col index vals from actual (firstcol --> lastcol)
     * to local, i.e., (0 --> lastcol-firstcol)
     * ---------------------------------------------------------------------
     */
    (&rowstr[0..(LASTROW - FIRSTROW + 1) as usize])
        .into_iter()
        .zip(&rowstr[1..(LASTROW - FIRSTROW + 2) as usize])
        .for_each(|(j, j1)| {
            for k in *j..*j1 {
                colidx[k as usize] -= FIRSTCOL;
            }
        });

    /* set starting vector to (1, 1, .... 1) */
    x[0..NA as usize + 1].fill(1.0);
    q.fill(0.0);
    z.fill(0.0);
    r.fill(0.0);
    p.fill(0.0);

    /*
     * -------------------------------------------------------------------
     * ---->
     * do one iteration untimed to init all code and data page tables
     * ----> (then reinit, start timing, to niter its)
     * -------------------------------------------------------------------*/
    for _ in 0..1 {
        /* the call to the conjugate gradient routine */
        conj_grad(
            &mut colidx[..],
            &mut rowstr[..],
            &mut x[..],
            &mut z[..],
            &mut a[..],
            &mut p[..],
            &mut q[..],
            &mut r[..],
            &mut rnorm,
        );

        /*
         * --------------------------------------------------------------------
         * zeta = shift + 1/(x.z)
         * so, first: (x.z)
         * also, find norm of z
         * so, first: (z.z)
         * --------------------------------------------------------------------
         */
        norm_temp1 = 0.0;
        norm_temp2 = 0.0;

        for j in 0..(LASTCOL - FIRSTCOL + 1) as usize {
            norm_temp1 = norm_temp1 + x[j] * z[j];
            norm_temp2 = norm_temp2 + z[j] * z[j];
        }
        norm_temp2 = 1.0 / f64::sqrt(norm_temp2);

        /* normalize z to obtain x */
        for j in 0..(LASTCOL - FIRSTCOL + 1) as usize {
            x[j] = norm_temp2 * z[j];
        }
    } /* end of do one iteration untimed */

    /* set starting vector to (1, 1, .... 1) */
    x[0..NA as usize + 1].fill(1.0);

    timers.stop(T_INIT);

    print!(
        " Initialization time = {:>15.3} seconds\n",
        timers.read(T_INIT).as_secs_f64()
    );

    timers.start(T_BENCH);

    /*
     * --------------------------------------------------------------------
     * ---->
     * main iteration for inverse power method
     * ---->
     * --------------------------------------------------------------------
     */
    for it in 1..NITER + 1 {
        /* the call to the conjugate gradient routine */
        if TIMERS {
            timers.start(T_CONJ_GRAD);
        }
        conj_grad(
            &mut colidx[..],
            &mut rowstr[..],
            &mut x[..],
            &mut z[..],
            &mut a[..],
            &mut p[..],
            &mut q[..],
            &mut r[..],
            &mut rnorm,
        );
        if TIMERS {
            timers.stop(T_CONJ_GRAD);
        }

        /*
         * --------------------------------------------------------------------
         * zeta = shift + 1/(x.z)
         * so, first: (x.z)
         * also, find norm of z
         * so, first: (z.z)
         * --------------------------------------------------------------------
         */
        norm_temp1 = 0.0;
        norm_temp2 = 0.0;
        for j in 0..(LASTCOL - FIRSTCOL + 1) as usize {
            norm_temp1 += x[j] * z[j];
            norm_temp2 += z[j] * z[j];
        }
        norm_temp2 = 1.0 / f64::sqrt(norm_temp2);

        zeta = SHIFT + 1.0 / norm_temp1;
        if it == 1 {
            println!("\n   iteration             ||r||                 zeta");
        }
        println!("   {:>5}       {:>20.14e}{:>20.13e}", it, rnorm, zeta);

        /* normalize z to obtain x */
        for j in 0..(LASTCOL - FIRSTCOL + 1) as usize {
            x[j] = norm_temp2 * z[j];
        }
    } /* end of main iter inv pow meth */

    timers.stop(T_BENCH);

    /*
     * --------------------------------------------------------------------
     * end of timed section
     * --------------------------------------------------------------------
     */
    t = timers.read(T_BENCH).as_secs_f64();

    print!(" Benchmark completed\n");

    if CLASS != 'U' {
        let err = f64::abs(zeta - ZETA_VERIFY) / ZETA_VERIFY;
        if err <= EPSILON {
            verified = 1;
            print!(" VERIFICATION SUCCESSFUL\n");
            print!(" Zeta is    {:+20.13e}\n", zeta);
            print!(" Error is   {:+20.13e}\n", err);
        } else {
            verified = 0;
            print!(" VERIFICATION FAILED\n");
            print!(" Zeta                {:+20.13e}\n", zeta);
            print!(" The correct zeta is {:+20.13e}\n", ZETA_VERIFY);
        }
    } else {
        verified = 0;
        print!(" Problem size unknown\n");
        print!(" NO VERIFICATION PERFORMED\n");
    }
    if t != 0.0 {
        mops = ((NITER << 1) * NA) as f64
            * (3.0
                + (NONZER * (NONZER + 1)) as f64
                + 25.0 * (5.0 + (NONZER * (NONZER + 1)) as f64)
                + 3.0)
            / t
            / 1000000.0;
    } else {
        mops = 0.0;
    }

    let info = PrintInfo {
        name: String::from("CG"),
        class: CLASS.to_string(),
        size: (NA as usize, 0, 0),
        num_iter: NITER,
        time: t,
        mops,
        operation: String::from("Floating Point"),
        verified,
        num_threads: 1,
        //uns: UNSAFE
    };
    printer(info);

    /*
     * ---------------------------------------------------------------------
     * more timers
     * ---------------------------------------------------------------------
     */
    if TIMERS {
        let mut t_names: Vec<String> = vec![String::new(); 3];
        t_names[T_INIT] = String::from("init");
        t_names[T_BENCH] = String::from("benchmk");
        t_names[T_CONJ_GRAD] = String::from("conjgd");

        tmax = timers.read(T_BENCH).as_secs_f64();
        if tmax == 0.0 {
            tmax = 1.0;
        }
        print!("  SECTION   Time (secs)\n");
        for i in 0..T_LAST {
            t = timers.read(i).as_secs_f64();
            if i == T_INIT {
                print!("  {:>8}:{:>9.3}\n", t_names[i], t);
            } else {
                print!(
                    "  {:>8}:{:>9.3}  ({:>6.2}%)\n",
                    t_names[i],
                    t,
                    t * 100.0 / tmax
                );
                if i == T_CONJ_GRAD {
                    t = tmax - t;
                    print!(
                        "    --> {:>8}:{:>9.3}  ({:>6.2}%)\n",
                        "rest",
                        t,
                        t * 100.0 / tmax
                    );
                }
            }
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * floating point arrays here are named as in NPB1 spec discussion of
 * CG algorithm
 * ---------------------------------------------------------------------
 */
fn conj_grad(
    colidx: &mut [i32],
    rowstr: &mut [i32],
    x: &mut [f64],
    z: &mut [f64],
    a: &mut [f64],
    p: &mut [f64],
    q: &mut [f64],
    r: &mut [f64],
    rnorm: &mut f64,
) {
    let cgitmax: i32 = 25;
    let (mut d, sum, mut rho, mut rho0, mut alpha, mut beta): (f64, f64, f64, f64, f64, f64);

    /* initialize the CG algorithm */
    q.fill(0.0);
    z.fill(0.0);
    (&mut r[..])
        .into_iter()
        .zip(&mut p[..])
        .zip(&x[..])
        .for_each(|((r, p), x)| {
            *r = *x;
            *p = *r;
        });

    /*
     * --------------------------------------------------------------------
     * rho = r.r
     * now, obtain the norm of r: First, sum squares of r elements locally...
     * --------------------------------------------------------------------
     */
    rho = (&r[0..(LASTCOL - FIRSTCOL + 1) as usize])
        .into_iter()
        .map(|r| *r * r)
        .sum();

    /* the conj grad iteration loop */
    for _ in 1..cgitmax + 1 {
        /*
         * ---------------------------------------------------------------------
         * q = A.p
         * the partition submatrix-vector multiply: use workspace w
         * ---------------------------------------------------------------------
         *
         * note: this version of the multiply is actually (slightly: maybe %5)
         * faster on the sp2 on 16 nodes than is the unrolled-by-2 version
         * below. on the Cray t3d, the reverse is TRUE, i.e., the
         * unrolled-by-two version is some 10% faster.
         * the unrolled-by-8 version below is significantly faster
         * on the Cray t3d - overall speed of code is 1.5 times faster.
         */
        (&rowstr[0..NA as usize])
            .into_iter()
            .zip(&rowstr[1..NA as usize + 1])
            .zip(&mut q[0..(LASTCOL - FIRSTCOL + 1) as usize])
            .for_each(|((j, j1), q)| {
                *q = (&a[*j as usize..*j1 as usize])
                    .into_iter()
                    .zip(&colidx[*j as usize..*j1 as usize])
                    .map(|(a, colidx)| a * p[*colidx as usize])
                    .sum();
            });

        /*
         * --------------------------------------------------------------------
         * obtain p.q
         * --------------------------------------------------------------------
         */
        d = (&p[0..(LASTCOL - FIRSTCOL + 1) as usize])
            .into_iter()
            .zip(&q[0..(LASTCOL - FIRSTCOL + 1) as usize])
            .map(|(p, q)| *p * *q)
            .sum();

        /*
         * --------------------------------------------------------------------
         * obtain alpha = rho / (p.q)
         * -------------------------------------------------------------------
         */
        alpha = rho / d;

        /*
         * --------------------------------------------------------------------
         * save a temporary of rho
         * --------------------------------------------------------------------
         */
        rho0 = rho;

        /*
         * ---------------------------------------------------------------------
         * obtain z = z + alpha*p
         * and    r = r - alpha*q
         * ---------------------------------------------------------------------
         */
        for j in 0..(LASTCOL - FIRSTCOL + 1) as usize {
            z[j] += alpha * p[j];
            r[j] -= alpha * q[j];
        }

        /*
         * ---------------------------------------------------------------------
         * rho = r.r
         * now, obtain the norm of r: first, sum squares of r elements locally...
         * ---------------------------------------------------------------------
         */
        rho = (&r[0..(LASTCOL - FIRSTCOL + 1) as usize])
            .into_iter()
            .map(|r| *r * r)
            .sum();

        /*
         * ---------------------------------------------------------------------
         * obtain beta
         * ---------------------------------------------------------------------
         */
        beta = rho / rho0;

        /*
         * ---------------------------------------------------------------------
         * p = r + beta*p
         * ---------------------------------------------------------------------
         */
        for j in 0..(LASTCOL - FIRSTCOL + 1) as usize {
            p[j] = r[j] + beta * p[j];
        }
    } /* end of do cgit=1, cgitmax */

    /*
     * ---------------------------------------------------------------------
     * compute residual norm explicitly: ||r|| = ||x - A.z||
     * first, form A.z
     * the partition submatrix-vector multiply
     * ---------------------------------------------------------------------
     */
    (&rowstr[0..NA as usize])
        .into_iter()
        .zip(&rowstr[1..NA as usize + 1])
        .zip(&mut r[0..(LASTCOL - FIRSTCOL + 1) as usize])
        .for_each(|((j, j1), r)| {
            *r = (&a[*j as usize..*j1 as usize])
                .into_iter()
                .zip(&colidx[*j as usize..*j1 as usize])
                .map(|(a, colidx)| a * z[*colidx as usize])
                .sum();
        });

    /*
     * ---------------------------------------------------------------------
     * at this point, r contains A.z
     * ---------------------------------------------------------------------
     */
    sum = (&x[0..(LASTCOL - FIRSTCOL + 1) as usize])
        .into_iter()
        .zip(&r[0..(LASTCOL - FIRSTCOL + 1) as usize])
        .map(|(x, r)| {
            let d = *x - *r;
            d * d
        })
        .sum();

    *rnorm = f64::sqrt(sum);
}

/*
 * ---------------------------------------------------------------------
 * scale a double precision number x in (0,1) by a power of 2 and chop it
 * ---------------------------------------------------------------------
 */
const fn icnvrt(x: f64, ipwr2: i32) -> i32 {
    return (x * ipwr2 as f64) as i32;
}

/*
 * ---------------------------------------------------------------------
 * generate the test problem for benchmark 6
 * makea generates a sparse matrix with a
 * prescribed sparsity distribution
 *
 * parameter    type        usage
 *
 * input
 *
 * n            i           number of cols/rows of matrix
 * nz           i           nonzeros as declared array size
 * rcond        r*8         condition number
 * shift        r*8         main diagonal shift
 *
 * output
 *
 * a            r*8         array for nonzeros
 * colidx       i           col indices
 * rowstr       i           row pointers
 *
 * workspace
 *
 * iv, arow, acol i
 * aelt           r*8
 * ---------------------------------------------------------------------
 */
fn makea(
    n: i32,
    nz: usize,
    a: &mut [f64],
    colidx: &mut [i32],
    rowstr: &mut [i32],
    arow: &mut [i32],
    acol: &mut [i32],
    aelt: &mut [f64],
    iv: &mut [i32],
    tran: &mut f64,
) {
    let (mut nzv, mut nn1): (i32, i32);
    let mut ivc = [0; NONZER as usize + 1];
    let mut vc = [0.0; NONZER as usize + 1];

    /*
     * --------------------------------------------------------------------
     * nonzer is approximately  (int(sqrt(nnza /n)));
     * --------------------------------------------------------------------
     * nn1 is the smallest power of two not less than n
     * --------------------------------------------------------------------
     */
    nn1 = 1;
    while nn1 < n {
        nn1 *= 2;
    }

    /*
     * -------------------------------------------------------------------
     * generate nonzero positions and save for the use in sparse
     * -------------------------------------------------------------------
     */
    for iouter in 0..n {
        nzv = NONZER;
        sprnvc(n, nzv, nn1, &mut vc, &mut ivc, tran);
        vecset(&mut vc, &mut ivc, &mut nzv, iouter + 1, 0.5);
        arow[iouter as usize] = nzv;

        for ivelt in 0..nzv {
            acol[(iouter * (NONZER + 1) + ivelt) as usize] = ivc[ivelt as usize] - 1;
            aelt[(iouter * (NONZER + 1) + ivelt) as usize] = vc[ivelt as usize];
        }
    }

    /*
     * ---------------------------------------------------------------------
     * ... make the sparse matrix from list of elements with duplicates
     * (iv is used as  workspace)
     * ---------------------------------------------------------------------
     */
    sparse(a, colidx, rowstr, n, nz, arow, acol, aelt, iv, 0.1);
}

fn sparse(
    a: &mut [f64],
    colidx: &mut [i32],
    rowstr: &mut [i32],
    n: i32,
    nz: usize,
    arow: &mut [i32],
    acol: &mut [i32],
    aelt: &mut [f64],
    nzloc: &mut [i32],
    rcond: f64,
) {
    /*
     * ---------------------------------------------------
     * generate a sparse matrix from a list of
     * [col, row, element] tri
     * ---------------------------------------------------
     */
    let (mut j, mut j1, mut j2, mut nza, mut jcol, mut k_aux): (i32, i32, i32, i32, i32, i32);
    let (mut size, mut scale, ratio, mut va): (f64, f64, f64, f64);
    let mut goto_40: bool;
    k_aux = -1;

    /*
     * --------------------------------------------------------------------
     * how many rows of result
     * --------------------------------------------------------------------
     */
    let nrows: usize = (LASTROW - FIRSTROW + 1) as usize;

    /*
     * --------------------------------------------------------------------
     * ...count the number of triples in each row
     * --------------------------------------------------------------------
     */
    rowstr[0..(nrows + 1) as usize].fill(0);

    for i in 0..n {
        for nza in 0..arow[i as usize] {
            j = acol[(i * (NONZER + 1) + nza) as usize] + 1;
            rowstr[j as usize] += arow[i as usize];
        }
    }
    rowstr[0] = 0;
    for j in 1..nrows + 1 {
        rowstr[j] += rowstr[j - 1]
    }
    nza = rowstr[nrows] - 1;

    /*
     * ---------------------------------------------------------------------
     * ... rowstr(j) now is the location of the first nonzero
     * of row j of a
     * ---------------------------------------------------------------------
     */
    if nza as usize > nz {
        print!("Space for matrix elements exceeded in sparse\n");
        print!("nza, nzmax = {}, {}\n", nza, nz);
        std::process::exit(0);
    }

    /*
     * ---------------------------------------------------------------------
     * ... preload data pages
     * ---------------------------------------------------------------------
     */
    (&rowstr[0..nrows])
        .into_iter()
        .zip(&rowstr[1..nrows + 1])
        .zip(&mut nzloc[0..nrows])
        .for_each(|((j, j1), nzloc)| {
            for k in *j..*j1 {
                a[k as usize] = 0.0;
                colidx[k as usize] = -1;
            }
            *nzloc = 0;
        });

    /*
     * ---------------------------------------------------------------------
     * ... generate actual values by summing duplicates
     * ---------------------------------------------------------------------
     */
    size = 1.0;
    ratio = f64::powf(rcond, 1.0 / n as f64);
    for i in 0..n {
        for nza in 0..arow[i as usize] {
            j = acol[(i * (NONZER + 1) + nza) as usize];

            scale = size * aelt[(i * (NONZER + 1) + nza) as usize];
            for nzrow in 0..arow[i as usize] {
                jcol = acol[(i * (NONZER + 1) + nzrow) as usize];
                va = aelt[(i * (NONZER + 1) + nzrow) as usize] * scale;

                /*
                 * --------------------------------------------------------------------
                 * ... add the identity * rcond to the generated matrix to bound
                 * the smallest eigenvalue from below by rcond
                 * --------------------------------------------------------------------
                 */
                if jcol == j && j == i {
                    va = va + rcond - SHIFT;
                }

                goto_40 = false;
                for k in rowstr[j as usize]..rowstr[(j + 1) as usize] {
                    if colidx[k as usize] > jcol {
                        /*
                         * ----------------------------------------------------------------
                         * ... insert colidx here orderly
                         * ----------------------------------------------------------------
                         */
                        for kk in (k..rowstr[j as usize + 1] - 1).rev() {
                            if colidx[kk as usize] > -1 {
                                a[(kk + 1) as usize] = a[kk as usize];
                                colidx[(kk + 1) as usize] = colidx[kk as usize];
                            }
                        }
                        colidx[k as usize] = jcol;
                        a[k as usize] = 0.0;
                        goto_40 = true;
                        k_aux = k;
                        break;
                    } else if colidx[k as usize] == -1 {
                        colidx[k as usize] = jcol;
                        goto_40 = true;
                        k_aux = k;
                        break;
                    } else if colidx[k as usize] == jcol {
                        /*
                         * --------------------------------------------------------------
                         * ... mark the duplicated entry
                         * -------------------------------------------------------------
                         */
                        nzloc[j as usize] = nzloc[j as usize] + 1;
                        goto_40 = true;
                        k_aux = k;
                        break;
                    }
                }
                if !goto_40 {
                    print!("internal error in sparse: i={}\n", i);
                    std::process::exit(0);
                }
                a[k_aux as usize] += va;
            }
        }
        size *= ratio;
    }

    /*
     * ---------------------------------------------------------------------
     * ... remove empty entries and generate final results
     * ---------------------------------------------------------------------
     */
    for j in 1..nrows {
        nzloc[j as usize] += nzloc[(j - 1) as usize];
    }

    for j in 0..nrows {
        j1 = if j > 0 { rowstr[j] - nzloc[j - 1] } else { 0 };
        j2 = rowstr[j + 1] - nzloc[j];
        nza = rowstr[j];
        for k in j1..j2 {
            a[k as usize] = a[nza as usize];
            colidx[k as usize] = colidx[nza as usize];
            nza += 1;
        }
    }
    for j in 1..nrows + 1 {
        rowstr[j] -= nzloc[j - 1];
    }
}

/*
 * ---------------------------------------------------------------------
 * generate a sparse n-vector (v, iv)
 * having nzv nonzeros
 *
 * mark(i) is set to 1 if position i is nonzero.
 * mark is all zero on entry and is reset to all zero before exit
 * this corrects a performance bug found by John G. Lewis, caused by
 * reinitialization of mark on every one of the n calls to sprnvc
 * ---------------------------------------------------------------------
 */
fn sprnvc(n: i32, nz: i32, nn1: i32, v: &mut [f64], iv: &mut [i32], tran: &mut f64) {
    let (mut nzv, mut i): (i32, i32);
    let (mut vecelt, mut vecloc): (f64, f64);

    nzv = 0;
    while nzv < nz {
        vecelt = randlc(tran, AMULT);

        /*
         * --------------------------------------------------------------------
         * generate an integer between 1 and n in a portable manner
         * --------------------------------------------------------------------
         */
        vecloc = randlc(tran, AMULT);
        i = icnvrt(vecloc, nn1) + 1;
        if i > n {
            continue;
        }

        /*
         * --------------------------------------------------------------------
         * was this integer generated already?
         * --------------------------------------------------------------------
         */
        if iv[0..nzv as usize].iter().any(|v| *v == i) {
            continue;
        }
        v[nzv as usize] = vecelt;
        iv[nzv as usize] = i;
        nzv += 1;
    }
}

/*
 * --------------------------------------------------------------------
 * set ith element of sparse vector (v, iv) with
 * nzv nonzeros to val
 * --------------------------------------------------------------------
 */
fn vecset(v: &mut [f64], iv: &mut [i32], nzv: &mut i32, i: i32, val: f64) {
    let mut set: bool = false;

    (&iv[0..*nzv as usize])
        .into_iter()
        .zip(&mut v[0..*nzv as usize])
        .for_each(|(iv, v)| {
            if *iv == i {
                *v = val;
                set = true;
            }
        });
    if !set {
        v[*nzv as usize] = val;
        iv[*nzv as usize] = i;
        *nzv += 1;
    }
}
