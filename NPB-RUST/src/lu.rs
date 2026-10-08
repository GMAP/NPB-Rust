mod common;
use crate::common::print_results::*;
use crate::common::timers::*;

#[cfg(class = "S")]
mod params {
    pub const CLASS: char = 'S';
    pub const ISIZ1: usize = 12;
    pub const ISIZ2: usize = 12;
    pub const ISIZ3: usize = 12;
    pub const DT_DEFAULT: f64 = 0.5;
    pub const INORM_DEFAULT: usize = 50;
    pub const ITMAX_DEFAULT: usize = 50;
}

#[cfg(class = "W")]
mod params {
    pub const CLASS: char = 'W';
    pub const ISIZ1: usize = 33;
    pub const ISIZ2: usize = 33;
    pub const ISIZ3: usize = 33;
    pub const DT_DEFAULT: f64 = 1.5e-3;
    pub const INORM_DEFAULT: usize = 300;
    pub const ITMAX_DEFAULT: usize = 300;
}

#[cfg(class = "A")]
mod params {
    pub const CLASS: char = 'A';
    pub const ISIZ1: usize = 64;
    pub const ISIZ2: usize = 64;
    pub const ISIZ3: usize = 64;
    pub const DT_DEFAULT: f64 = 2.0;
    pub const INORM_DEFAULT: usize = 250;
    pub const ITMAX_DEFAULT: usize = 250;
}

#[cfg(class = "B")]
mod params {
    pub const CLASS: char = 'B';
    pub const ISIZ1: usize = 102;
    pub const ISIZ2: usize = 102;
    pub const ISIZ3: usize = 102;
    pub const DT_DEFAULT: f64 = 2.0;
    pub const INORM_DEFAULT: usize = 250;
    pub const ITMAX_DEFAULT: usize = 250;
}

#[cfg(class = "C")]
mod params {
    pub const CLASS: char = 'C';
    pub const ISIZ1: usize = 162;
    pub const ISIZ2: usize = 162;
    pub const ISIZ3: usize = 162;
    pub const DT_DEFAULT: f64 = 2.0;
    pub const INORM_DEFAULT: usize = 250;
    pub const ITMAX_DEFAULT: usize = 250;
}

#[cfg(class = "D")]
mod params {
    pub const CLASS: char = 'D';
    pub const ISIZ1: usize = 408;
    pub const ISIZ2: usize = 408;
    pub const ISIZ3: usize = 408;
    pub const DT_DEFAULT: f64 = 1.0;
    pub const INORM_DEFAULT: usize = 300;
    pub const ITMAX_DEFAULT: usize = 300;
}

#[cfg(class = "E")]
mod params {
    pub const CLASS: char = 'E';
    pub const ISIZ1: usize = 1020;
    pub const ISIZ2: usize = 1020;
    pub const ISIZ3: usize = 1020;
    pub const DT_DEFAULT: f64 = 0.5;
    pub const INORM_DEFAULT: usize = 300;
    pub const ITMAX_DEFAULT: usize = 300;
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
    pub const PROBLEM_SIZE: usize = 1;
    pub const ISIZ1: usize = 5;
    pub const ISIZ2: usize = 5;
    pub const ISIZ3: usize = 5;
    pub const DT_DEFAULT: f64 = 1.0;
    pub const INORM_DEFAULT: usize = 1;
    pub const ITMAX_DEFAULT: usize = 1;
    compile_error!(
        "\n\n\
		Must set a class at compilation time by setting RUSTFLAGS\n\
		class options for LU are: {S, W, A, B, C, D, E}\n\
		For example:\n\
		RUSTFLAGS='--cfg class=\"A\"' cargo build --release --bin lu\n\n\n\
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

pub const IPR_DEFAULT: i32 = 1;
pub const OMEGA_DEFAULT: f64 = 1.2;
pub const TOLRSD1_DEF: f64 = 1.0e-08;
pub const TOLRSD2_DEF: f64 = 1.0e-08;
pub const TOLRSD3_DEF: f64 = 1.0e-08;
pub const TOLRSD4_DEF: f64 = 1.0e-08;
pub const TOLRSD5_DEF: f64 = 1.0e-08;
pub const C1: f64 = 1.40e+00;
pub const C2: f64 = 0.40e+00;
pub const C3: f64 = 1.00e-01;
pub const C4: f64 = 1.00e+00;
pub const C5: f64 = 1.40e+00;
pub const T_TOTAL: usize = 1;
pub const T_RHSX: usize = 2;
pub const T_RHSY: usize = 3;
pub const T_RHSZ: usize = 4;
pub const T_RHS: usize = 5;
pub const T_JACLD: usize = 6;
pub const T_BLTS: usize = 7;
pub const T_JACU: usize = 8;
pub const T_BUTS: usize = 9;
pub const T_ADD: usize = 10;
pub const T_L2NORM: usize = 11;
pub const T_LAST: usize = 11;

pub const EPSILON: f64 = 1.0e-08;
pub const NX0: usize = ISIZ1;
pub const NY0: usize = ISIZ2;
pub const NZ0: usize = ISIZ3;
pub const NX: usize = NX0;
pub const NY: usize = NY0;
pub const NZ: usize = NZ0;
pub const IST: usize = 1;
pub const IEND: usize = NX - 1;
pub const JST: usize = 1;
pub const JEND: usize = NY - 1;
pub const II1: usize = 1;
pub const II2: usize = NX0 - 1;
pub const JI1: usize = 1;
pub const JI2: usize = NY0 - 2;
pub const KI1: usize = 2;
pub const KI2: usize = NZ0 - 1;
pub const DXI: f64 = 1.0 / (NX0 - 1) as f64;
pub const DETA: f64 = 1.0 / (NY0 - 1) as f64;
pub const DZETA: f64 = 1.0 / (NZ0 - 1) as f64;
pub const TX1: f64 = 1.0 / (DXI * DXI);
pub const TX2: f64 = 1.0 / (2.0 * DXI);
pub const TX3: f64 = 1.0 / DXI;
pub const TY1: f64 = 1.0 / (DETA * DETA);
pub const TY2: f64 = 1.0 / (2.0 * DETA);
pub const TY3: f64 = 1.0 / DETA;
pub const TZ1: f64 = 1.0 / (DZETA * DZETA);
pub const TZ2: f64 = 1.0 / (2.0 * DZETA);
pub const TZ3: f64 = 1.0 / DZETA;
pub const DX1: f64 = 0.75;
pub const DX2: f64 = DX1;
pub const DX3: f64 = DX1;
pub const DX4: f64 = DX1;
pub const DX5: f64 = DX1;
pub const DY1: f64 = 0.75;
pub const DY2: f64 = DY1;
pub const DY3: f64 = DY1;
pub const DY4: f64 = DY1;
pub const DY5: f64 = DY1;
pub const DZ1: f64 = 1.00;
pub const DZ2: f64 = DZ1;
pub const DZ3: f64 = DZ1;
pub const DZ4: f64 = DZ1;
pub const DZ5: f64 = DZ1;
pub const DSSP: f64 = {
    let (x, y);
    if DY1 > DZ1 {
        x = DY1
    } else {
        x = DZ1
    }
    if x > DX1 {
        y = x * 0.25
    } else {
        y = DX1 * 0.25
    }
    y
};

/* lu */
fn main() {
    let mut u: Vec<[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]> =
        vec![[[[0.0; 5]; ISIZ1 + 1]; ISIZ2 + 1]; ISIZ3];
    let mut rsd: Vec<[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]> =
        vec![[[[0.0; 5]; ISIZ1 + 1]; ISIZ2 + 1]; ISIZ3];
    let mut frct: Vec<[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]> =
        vec![[[[0.0; 5]; ISIZ1 + 1]; ISIZ2 + 1]; ISIZ3];
    let mut flux: Vec<[f64; 5]> = vec![[0.0; 5]; ISIZ1];
    let mut qs: Vec<[[f64; ISIZ1 + 1]; ISIZ2 + 1]> = vec![[[0.0; ISIZ1 + 1]; ISIZ2 + 1]; ISIZ3];
    let mut rho_i: Vec<[[f64; ISIZ1 + 1]; ISIZ2 + 1]> = vec![[[0.0; ISIZ1 + 1]; ISIZ2 + 1]; ISIZ3];
    let mut a: Vec<[[[f64; 5]; 5]; ISIZ1 + 1]> = vec![[[[0.0; 5]; 5]; ISIZ1 + 1]; ISIZ2];
    let mut b: Vec<[[[f64; 5]; 5]; ISIZ1 + 1]> = vec![[[[0.0; 5]; 5]; ISIZ1 + 1]; ISIZ2];
    let mut c: Vec<[[[f64; 5]; 5]; ISIZ1 + 1]> = vec![[[[0.0; 5]; 5]; ISIZ1 + 1]; ISIZ2];
    let mut d: Vec<[[[f64; 5]; 5]; ISIZ1 + 1]> = vec![[[[0.0; 5]; 5]; ISIZ1 + 1]; ISIZ2];
    let mut ce: Vec<[f64; 5]> = vec![[0.0; 5]; 13];
    let mut tolrsd: Vec<f64> = vec![0.0; 5];
    let mut rsdnm: Vec<f64> = vec![0.0; 5];
    let mut errnm: Vec<f64> = vec![0.0; 5];
    let mut maxtime: f64 = 0.0;
    let mut frc: f64 = 0.0;
    let mut verified: i8 = 0;

    println!(" Using compiled defaults");
    let mut timers = Timer::new();
    for i in 1..T_LAST + 1 {
        timers.clear(i);
    }

    println!("\n\n NAS Parallel Benchmarks 4.1 Serial Rust version - LU Benchmark\n");
    println!(" Size: {} {} {}", NX0, NY0, NZ0);
    println!(" Iterations: {}", ITMAX_DEFAULT);
    println!("");

    /* - - - - - - - - - - SET CONSTANTS - - - - - - - - - - */
    tolrsd[0] = TOLRSD1_DEF;
    tolrsd[1] = TOLRSD2_DEF;
    tolrsd[2] = TOLRSD3_DEF;
    tolrsd[3] = TOLRSD4_DEF;
    tolrsd[4] = TOLRSD5_DEF;
    /*
     * ---------------------------------------------------------------------
     * coefficients of the exact solution to the first pde
     * ---------------------------------------------------------------------
     */
    ce[0][0] = 2.0;
    ce[1][0] = 0.0;
    ce[2][0] = 0.0;
    ce[3][0] = 4.0;
    ce[4][0] = 5.0;
    ce[5][0] = 3.0;
    ce[6][0] = 5.0e-01;
    ce[7][0] = 2.0e-02;
    ce[8][0] = 1.0e-02;
    ce[9][0] = 3.0e-02;
    ce[10][0] = 5.0e-01;
    ce[11][0] = 4.0e-01;
    ce[12][0] = 3.0e-01;
    /*
     * ---------------------------------------------------------------------
     * coefficients of the exact solution to the second pde
     * ---------------------------------------------------------------------
     */
    ce[0][1] = 1.0;
    ce[1][1] = 0.0;
    ce[2][1] = 0.0;
    ce[3][1] = 0.0;
    ce[4][1] = 1.0;
    ce[5][1] = 2.0;
    ce[6][1] = 3.0;
    ce[7][1] = 1.0e-02;
    ce[8][1] = 3.0e-02;
    ce[9][1] = 2.0e-02;
    ce[10][1] = 4.0e-01;
    ce[11][1] = 3.0e-01;
    ce[12][1] = 5.0e-01;
    /*
     * ---------------------------------------------------------------------
     * coefficients of the exact solution to the third pde
     * ---------------------------------------------------------------------
     */
    ce[0][2] = 2.0;
    ce[1][2] = 2.0;
    ce[2][2] = 0.0;
    ce[3][2] = 0.0;
    ce[4][2] = 0.0;
    ce[5][2] = 2.0;
    ce[6][2] = 3.0;
    ce[7][2] = 4.0e-02;
    ce[8][2] = 3.0e-02;
    ce[9][2] = 5.0e-02;
    ce[10][2] = 3.0e-01;
    ce[11][2] = 5.0e-01;
    ce[12][2] = 4.0e-01;
    /*
     * ---------------------------------------------------------------------
     * coefficients of the exact solution to the fourth pde
     * ---------------------------------------------------------------------
     */
    ce[0][3] = 2.0;
    ce[1][3] = 2.0;
    ce[2][3] = 0.0;
    ce[3][3] = 0.0;
    ce[4][3] = 0.0;
    ce[5][3] = 2.0;
    ce[6][3] = 3.0;
    ce[7][3] = 3.0e-02;
    ce[8][3] = 5.0e-02;
    ce[9][3] = 4.0e-02;
    ce[10][3] = 2.0e-01;
    ce[11][3] = 1.0e-01;
    ce[12][3] = 3.0e-01;
    /*
     * ---------------------------------------------------------------------
     * coefficients of the exact solution to the fifth pde
     * ---------------------------------------------------------------------
     */
    ce[0][4] = 5.0;
    ce[1][4] = 4.0;
    ce[2][4] = 3.0;
    ce[3][4] = 2.0;
    ce[4][4] = 1.0e-01;
    ce[5][4] = 4.0e-01;
    ce[6][4] = 3.0e-01;
    ce[7][4] = 5.0e-02;
    ce[8][4] = 4.0e-02;
    ce[9][4] = 3.0e-02;
    ce[10][4] = 1.0e-01;
    ce[11][4] = 3.0e-01;
    ce[12][4] = 2.0e-01;
    /* - - - - - - - - - - END SET CONSTANTS - - - - - - - - - - */

    /*
     * ---------------------------------------------------------------------
     * set the boundary values for dependent variables
     * ---------------------------------------------------------------------
     */
    setbv(&mut u[..], &ce[..]);
    /*
     * ---------------------------------------------------------------------
     * set the initial values for dependent variables
     * ---------------------------------------------------------------------
     */
    setiv(&mut u[..], &ce[..]);
    /*
     * ---------------------------------------------------------------------
     * compute the forcing term based on prescribed exact solution
     * ---------------------------------------------------------------------
     */
    erhs(&mut frct[..], &mut rsd[..], &mut flux[..], &ce[..]);
    /*
     * ---------------------------------------------------------------------
     * perform one SSOR iteration to touch all pages
     * ---------------------------------------------------------------------
     */
    ssor(
        1,
        &mut a[..],
        &mut b[..],
        &mut c[..],
        &mut d[..],
        &mut rsd[..],
        &mut frct[..],
        &mut qs[..],
        &mut flux[..],
        &mut rho_i[..],
        &mut u[..],
        &mut rsdnm[..],
        &mut tolrsd[..],
        &mut maxtime,
        &mut timers,
    );
    /*
     * ---------------------------------------------------------------------
     * reset the boundary and initial values
     * ---------------------------------------------------------------------
     */
    setbv(&mut u[..], &ce[..]);
    setiv(&mut u[..], &ce[..]);
    /*
     * ---------------------------------------------------------------------
     * perform the SSOR iterations
     * ---------------------------------------------------------------------
     */
    ssor(
        ITMAX_DEFAULT,
        &mut a[..],
        &mut b[..],
        &mut c[..],
        &mut d[..],
        &mut rsd[..],
        &mut frct[..],
        &mut qs[..],
        &mut flux[..],
        &mut rho_i[..],
        &mut u[..],
        &mut rsdnm[..],
        &mut tolrsd[..],
        &mut maxtime,
        &mut timers,
    );
    /*
     * ---------------------------------------------------------------------
     * compute the solution error
     * ---------------------------------------------------------------------
     */
    error(&mut errnm[..], &u[..], &ce[..]);
    /*
     * ---------------------------------------------------------------------
     * compute the surface integral
     * ---------------------------------------------------------------------
     */
    pintgr(&u[..], &mut frc);
    /*
     * ---------------------------------------------------------------------
     * verification test
     * ---------------------------------------------------------------------
     */
    verify(&rsdnm[..], &errnm[..], frc, &mut verified);

    let mops: f64 = ITMAX_DEFAULT as f64
        * (1984.77 * (NX0 * NY0 * NZ0) as f64
            - 10923.3 * ((NX0 + NY0 + NZ0) as f64 / 3.0).powf(2.0)
            + 27770.9 * (NX0 + NY0 + NZ0) as f64 / 3.0
            - 144010.0)
        / (maxtime * 1000000.0);

    let info = PrintInfo {
        name: String::from("LU"),
        class: CLASS.to_string(),
        size: (NX, NY, NZ),
        num_iter: ITMAX_DEFAULT as i32,
        time: maxtime,
        mops,
        operation: String::from("Floating point"),
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
        let mut t_names: Vec<String> = vec![String::new(); T_LAST + 1];
        t_names[T_TOTAL] = String::from("total");
        t_names[T_RHSX] = String::from("rhsx");
        t_names[T_RHSY] = String::from("rhsy");
        t_names[T_RHSZ] = String::from("rhsz");
        t_names[T_RHS] = String::from("rhs");
        t_names[T_JACLD] = String::from("jacld");
        t_names[T_BLTS] = String::from("blts");
        t_names[T_JACU] = String::from("jacu");
        t_names[T_BUTS] = String::from("buts");
        t_names[T_ADD] = String::from("add");
        t_names[T_L2NORM] = String::from("l2norm");

        let mut trecs: [f64; T_LAST + 1] = [0.0; T_LAST + 1];
        for i in 1..T_LAST + 1 {
            trecs[i] = timers.read(i).as_secs_f64();
        }
        let mut tmax: f64 = maxtime;
        if tmax == 0.0 {
            tmax = 1.0;
        }
        println!("  SECTION     Time (secs)");
        for i in 1..T_LAST + 1 {
            println!(
                "  {:<8}:{:>9.3}  ({:>6.2}%)",
                t_names[i],
                trecs[i],
                trecs[i] * 100.0 / tmax
            );
            if i == T_RHS {
                let mut t = trecs[T_RHSX] + trecs[T_RHSY] + trecs[T_RHSZ];
                println!(
                    "     --> {:>8}:{:>9.3}  ({:>6.2}%)",
                    "sub-rhs",
                    t,
                    t * 100.0 / tmax
                );
                t = trecs[i] - t;
                println!(
                    "     --> {:>8}:{:>9.3}  ({:>6.2}%)",
                    "rest-rhs",
                    t,
                    t * 100.0 / tmax
                );
            }
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * compute the regular-sparse, block lower triangular solution:
 * v <-- ( L-inv ) * v
 * ---------------------------------------------------------------------
 * to improve cache performance, second two dimensions padded by 1
 * for even number sizes only. only needed in v.
 * ---------------------------------------------------------------------
 */
fn blts(
    k: usize,
    v: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    ldz: &[[[[f64; 5]; 5]; ISIZ1 + 1]],
    ldy: &[[[[f64; 5]; 5]; ISIZ1 + 1]],
    ldx: &[[[[f64; 5]; 5]; ISIZ1 + 1]],
    d: &[[[[f64; 5]; 5]; ISIZ1 + 1]],
) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let (mut tmp, mut tmp1): (f64, f64);
    let mut tmat: [[f64; 5]; 5] = [[0.0; 5]; 5];
    let mut tv: [f64; 5] = [0.0; 5];
    for j in JST..JEND {
        for i in IST..IEND {
            for m in 0..5 {
                v[k][j][i][m] = v[k][j][i][m]
                    - OMEGA_DEFAULT
                        * (ldz[j][i][0][m] * v[k - 1][j][i][0]
                            + ldz[j][i][1][m] * v[k - 1][j][i][1]
                            + ldz[j][i][2][m] * v[k - 1][j][i][2]
                            + ldz[j][i][3][m] * v[k - 1][j][i][3]
                            + ldz[j][i][4][m] * v[k - 1][j][i][4]);
            }
        }
    }
    for j in JST..JEND {
        for i in IST..IEND {
            for m in 0..5 {
                tv[m] = v[k][j][i][m]
                    - OMEGA_DEFAULT
                        * (ldy[j][i][0][m] * v[k][j - 1][i][0]
                            + ldx[j][i][0][m] * v[k][j][i - 1][0]
                            + ldy[j][i][1][m] * v[k][j - 1][i][1]
                            + ldx[j][i][1][m] * v[k][j][i - 1][1]
                            + ldy[j][i][2][m] * v[k][j - 1][i][2]
                            + ldx[j][i][2][m] * v[k][j][i - 1][2]
                            + ldy[j][i][3][m] * v[k][j - 1][i][3]
                            + ldx[j][i][3][m] * v[k][j][i - 1][3]
                            + ldy[j][i][4][m] * v[k][j - 1][i][4]
                            + ldx[j][i][4][m] * v[k][j][i - 1][4]);
            }
            /*
             * ---------------------------------------------------------------------
             * diagonal block inversion
             *
             * forward elimination
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                tmat[0][m] = d[j][i][0][m];
                tmat[1][m] = d[j][i][1][m];
                tmat[2][m] = d[j][i][2][m];
                tmat[3][m] = d[j][i][3][m];
                tmat[4][m] = d[j][i][4][m];
            }
            /* */
            tmp1 = 1.0 / tmat[0][0];
            tmp = tmp1 * tmat[0][1];
            tmat[1][1] = tmat[1][1] - tmp * tmat[1][0];
            tmat[2][1] = tmat[2][1] - tmp * tmat[2][0];
            tmat[3][1] = tmat[3][1] - tmp * tmat[3][0];
            tmat[4][1] = tmat[4][1] - tmp * tmat[4][0];
            tv[1] = tv[1] - tv[0] * tmp;
            /* */
            tmp = tmp1 * tmat[0][2];
            tmat[1][2] = tmat[1][2] - tmp * tmat[1][0];
            tmat[2][2] = tmat[2][2] - tmp * tmat[2][0];
            tmat[3][2] = tmat[3][2] - tmp * tmat[3][0];
            tmat[4][2] = tmat[4][2] - tmp * tmat[4][0];
            tv[2] = tv[2] - tv[0] * tmp;
            /* */
            tmp = tmp1 * tmat[0][3];
            tmat[1][3] = tmat[1][3] - tmp * tmat[1][0];
            tmat[2][3] = tmat[2][3] - tmp * tmat[2][0];
            tmat[3][3] = tmat[3][3] - tmp * tmat[3][0];
            tmat[4][3] = tmat[4][3] - tmp * tmat[4][0];
            tv[3] = tv[3] - tv[0] * tmp;
            /* */
            tmp = tmp1 * tmat[0][4];
            tmat[1][4] = tmat[1][4] - tmp * tmat[1][0];
            tmat[2][4] = tmat[2][4] - tmp * tmat[2][0];
            tmat[3][4] = tmat[3][4] - tmp * tmat[3][0];
            tmat[4][4] = tmat[4][4] - tmp * tmat[4][0];
            tv[4] = tv[4] - tv[0] * tmp;
            /* */
            tmp1 = 1.0 / tmat[1][1];
            tmp = tmp1 * tmat[1][2];
            tmat[2][2] = tmat[2][2] - tmp * tmat[2][1];
            tmat[3][2] = tmat[3][2] - tmp * tmat[3][1];
            tmat[4][2] = tmat[4][2] - tmp * tmat[4][1];
            tv[2] = tv[2] - tv[1] * tmp;
            /* */
            tmp = tmp1 * tmat[1][3];
            tmat[2][3] = tmat[2][3] - tmp * tmat[2][1];
            tmat[3][3] = tmat[3][3] - tmp * tmat[3][1];
            tmat[4][3] = tmat[4][3] - tmp * tmat[4][1];
            tv[3] = tv[3] - tv[1] * tmp;
            /* */
            tmp = tmp1 * tmat[1][4];
            tmat[2][4] = tmat[2][4] - tmp * tmat[2][1];
            tmat[3][4] = tmat[3][4] - tmp * tmat[3][1];
            tmat[4][4] = tmat[4][4] - tmp * tmat[4][1];
            tv[4] = tv[4] - tv[1] * tmp;
            /* */
            tmp1 = 1.0 / tmat[2][2];
            tmp = tmp1 * tmat[2][3];
            tmat[3][3] = tmat[3][3] - tmp * tmat[3][2];
            tmat[4][3] = tmat[4][3] - tmp * tmat[4][2];
            tv[3] = tv[3] - tv[2] * tmp;
            /* */
            tmp = tmp1 * tmat[2][4];
            tmat[3][4] = tmat[3][4] - tmp * tmat[3][2];
            tmat[4][4] = tmat[4][4] - tmp * tmat[4][2];
            tv[4] = tv[4] - tv[2] * tmp;
            /* */
            tmp1 = 1.0 / tmat[3][3];
            tmp = tmp1 * tmat[3][4];
            tmat[4][4] = tmat[4][4] - tmp * tmat[4][3];
            tv[4] = tv[4] - tv[3] * tmp;
            /*
             * ---------------------------------------------------------------------
             * back substitution
             * ---------------------------------------------------------------------
             */
            v[k][j][i][4] = tv[4] / tmat[4][4];
            tv[3] = tv[3] - tmat[4][3] * v[k][j][i][4];
            v[k][j][i][3] = tv[3] / tmat[3][3];
            tv[2] = tv[2] - tmat[3][2] * v[k][j][i][3] - tmat[4][2] * v[k][j][i][4];
            v[k][j][i][2] = tv[2] / tmat[2][2];
            tv[1] = tv[1]
                - tmat[2][1] * v[k][j][i][2]
                - tmat[3][1] * v[k][j][i][3]
                - tmat[4][1] * v[k][j][i][4];
            v[k][j][i][1] = tv[1] / tmat[1][1];
            tv[0] = tv[0]
                - tmat[1][0] * v[k][j][i][1]
                - tmat[2][0] * v[k][j][i][2]
                - tmat[3][0] * v[k][j][i][3]
                - tmat[4][0] * v[k][j][i][4];
            v[k][j][i][0] = tv[0] / tmat[0][0];
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * compute the regular-sparse, block upper triangular solution:
 * v <-- ( U-inv ) * v
 * ---------------------------------------------------------------------
 * to improve cache performance, second two dimensions padded by 1
 * for even number sizes only. only needed in v.
 * ---------------------------------------------------------------------
 */
fn buts(
    k: usize,
    v: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    tv: &mut [f64],
    d: &[[[[f64; 5]; 5]; ISIZ1 + 1]],
    udx: &[[[[f64; 5]; 5]; ISIZ1 + 1]],
    udy: &[[[[f64; 5]; 5]; ISIZ1 + 1]],
    udz: &[[[[f64; 5]; 5]; ISIZ1 + 1]],
) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let (mut tmp, mut tmp1): (f64, f64);
    let mut tmat: [[f64; 5]; 5] = [[0.0; 5]; 5];
    for j in (JST..JEND).rev() {
        for i in (IST..IEND).rev() {
            for m in 0..5 {
                tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + m] = OMEGA_DEFAULT
                    * (udz[j][i][0][m] * v[k + 1][j][i][0]
                        + udz[j][i][1][m] * v[k + 1][j][i][1]
                        + udz[j][i][2][m] * v[k + 1][j][i][2]
                        + udz[j][i][3][m] * v[k + 1][j][i][3]
                        + udz[j][i][4][m] * v[k + 1][j][i][4]);
            }
        }
    }
    for j in (JST..JEND).rev() {
        for i in (IST..IEND).rev() {
            for m in 0..5 {
                tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + m] = tv
                    [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + m]
                    + OMEGA_DEFAULT
                        * (udy[j][i][0][m] * v[k][j + 1][i][0]
                            + udx[j][i][0][m] * v[k][j][i + 1][0]
                            + udy[j][i][1][m] * v[k][j + 1][i][1]
                            + udx[j][i][1][m] * v[k][j][i + 1][1]
                            + udy[j][i][2][m] * v[k][j + 1][i][2]
                            + udx[j][i][2][m] * v[k][j][i + 1][2]
                            + udy[j][i][3][m] * v[k][j + 1][i][3]
                            + udx[j][i][3][m] * v[k][j][i + 1][3]
                            + udy[j][i][4][m] * v[k][j + 1][i][4]
                            + udx[j][i][4][m] * v[k][j][i + 1][4]);
            }
            /*
             * ---------------------------------------------------------------------
             * diagonal block inversion
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                tmat[0][m] = d[j][i][0][m];
                tmat[1][m] = d[j][i][1][m];
                tmat[2][m] = d[j][i][2][m];
                tmat[3][m] = d[j][i][3][m];
                tmat[4][m] = d[j][i][4][m];
            }
            /* */
            tmp1 = 1.0 / tmat[0][0];
            tmp = tmp1 * tmat[0][1];
            tmat[1][1] = tmat[1][1] - tmp * tmat[1][0];
            tmat[2][1] = tmat[2][1] - tmp * tmat[2][0];
            tmat[3][1] = tmat[3][1] - tmp * tmat[3][0];
            tmat[4][1] = tmat[4][1] - tmp * tmat[4][0];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5] * tmp;
            /* */
            tmp = tmp1 * tmat[0][2];
            tmat[1][2] = tmat[1][2] - tmp * tmat[1][0];
            tmat[2][2] = tmat[2][2] - tmp * tmat[2][0];
            tmat[3][2] = tmat[3][2] - tmp * tmat[3][0];
            tmat[4][2] = tmat[4][2] - tmp * tmat[4][0];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5] * tmp;
            /* */
            tmp = tmp1 * tmat[0][3];
            tmat[1][3] = tmat[1][3] - tmp * tmat[1][0];
            tmat[2][3] = tmat[2][3] - tmp * tmat[2][0];
            tmat[3][3] = tmat[3][3] - tmp * tmat[3][0];
            tmat[4][3] = tmat[4][3] - tmp * tmat[4][0];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5] * tmp;
            /* */
            tmp = tmp1 * tmat[0][4];
            tmat[1][4] = tmat[1][4] - tmp * tmat[1][0];
            tmat[2][4] = tmat[2][4] - tmp * tmat[2][0];
            tmat[3][4] = tmat[3][4] - tmp * tmat[3][0];
            tmat[4][4] = tmat[4][4] - tmp * tmat[4][0];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5] * tmp;
            /* */
            tmp1 = 1.0 / tmat[1][1];
            tmp = tmp1 * tmat[1][2];
            tmat[2][2] = tmat[2][2] - tmp * tmat[2][1];
            tmat[3][2] = tmat[3][2] - tmp * tmat[3][1];
            tmat[4][2] = tmat[4][2] - tmp * tmat[4][1];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1] * tmp;
            /* */
            tmp = tmp1 * tmat[1][3];
            tmat[2][3] = tmat[2][3] - tmp * tmat[2][1];
            tmat[3][3] = tmat[3][3] - tmp * tmat[3][1];
            tmat[4][3] = tmat[4][3] - tmp * tmat[4][1];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1] * tmp;
            /* */
            tmp = tmp1 * tmat[1][4];
            tmat[2][4] = tmat[2][4] - tmp * tmat[2][1];
            tmat[3][4] = tmat[3][4] - tmp * tmat[3][1];
            tmat[4][4] = tmat[4][4] - tmp * tmat[4][1];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1] * tmp;
            /* */
            tmp1 = 1.0 / tmat[2][2];
            tmp = tmp1 * tmat[2][3];
            tmat[3][3] = tmat[3][3] - tmp * tmat[3][2];
            tmat[4][3] = tmat[4][3] - tmp * tmat[4][2];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2] * tmp;
            /* */
            tmp = tmp1 * tmat[2][4];
            tmat[3][4] = tmat[3][4] - tmp * tmat[3][2];
            tmat[4][4] = tmat[4][4] - tmp * tmat[4][2];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2] * tmp;
            /* */
            tmp1 = 1.0 / tmat[3][3];
            tmp = tmp1 * tmat[3][4];
            tmat[4][4] = tmat[4][4] - tmp * tmat[4][3];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4]
                - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3] * tmp;
            /*
             * ---------------------------------------------------------------------
             * back substitution
             * ---------------------------------------------------------------------
             */
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4] =
                tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4] / tmat[4][4];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3]
                - tmat[4][3] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3] =
                tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3] / tmat[3][3];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2]
                - tmat[3][2] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3]
                - tmat[4][2] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2] =
                tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2] / tmat[2][2];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1] = tv
                [j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1]
                - tmat[2][1] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2]
                - tmat[3][1] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3]
                - tmat[4][1] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1] =
                tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1] / tmat[1][1];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5] = tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5]
                - tmat[1][0] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1]
                - tmat[2][0] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2]
                - tmat[3][0] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3]
                - tmat[4][0] * tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4];
            tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5] =
                tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5] / tmat[0][0];
            v[k][j][i][0] = v[k][j][i][0] - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5];
            v[k][j][i][1] = v[k][j][i][1] - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 1];
            v[k][j][i][2] = v[k][j][i][2] - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 2];
            v[k][j][i][3] = v[k][j][i][3] - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 3];
            v[k][j][i][4] = v[k][j][i][4] - tv[j * (ISIZ1 / 2 * 2 + 1) * 5 + i * 5 + 4];
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * compute the right hand side based on exact solution
 * ---------------------------------------------------------------------
 */
fn erhs(
    frct: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    rsd: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    flux: &mut [[f64; 5]],
    ce: &[[f64; 5]],
) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let mut q;
    let mut tmp;
    let (mut u21, mut u31, mut u41): (f64, f64, f64);
    let (mut u21i, mut u31i, mut u41i, mut u51i): (f64, f64, f64, f64);
    let (mut u21j, mut u31j, mut u41j, mut u51j): (f64, f64, f64, f64);
    let (mut u21k, mut u31k, mut u41k, mut u51k): (f64, f64, f64, f64);
    let (mut u21im1, mut u31im1, mut u41im1, mut u51im1): (f64, f64, f64, f64);
    let (mut u21jm1, mut u31jm1, mut u41jm1, mut u51jm1): (f64, f64, f64, f64);
    let (mut u21km1, mut u31km1, mut u41km1, mut u51km1): (f64, f64, f64, f64);
    frct.iter_mut().for_each(|frct| {
        frct.iter_mut().for_each(|frct| {
            frct.iter_mut().for_each(|frct| {
                frct.iter_mut().for_each(|frct| *frct = 0.0);
            });
        });
    });
    for k in 0..NZ {
        let zeta = k as f64 / (NZ as f64 - 1.0);
        for j in 0..NY {
            let eta = j as f64 / (NY0 as f64 - 1.0);
            for i in 0..NX {
                let xi = i as f64 / (NX0 as f64 - 1.0);
                for m in 0..5 {
                    rsd[k][j][i][m] = ce[0][m]
                        + (ce[1][m] + (ce[4][m] + (ce[7][m] + ce[10][m] * xi) * xi) * xi) * xi
                        + (ce[2][m] + (ce[5][m] + (ce[8][m] + ce[11][m] * eta) * eta) * eta) * eta
                        + (ce[3][m] + (ce[6][m] + (ce[9][m] + ce[12][m] * zeta) * zeta) * zeta)
                            * zeta;
                }
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * xi-direction flux differences
     * ---------------------------------------------------------------------
     */
    for k in 1..NZ - 1 {
        for j in JST..JEND {
            for i in 0..NX {
                flux[i][0] = rsd[k][j][i][1];
                u21 = rsd[k][j][i][1] / rsd[k][j][i][0];
                q = 0.50
                    * (rsd[k][j][i][1] * rsd[k][j][i][1]
                        + rsd[k][j][i][2] * rsd[k][j][i][2]
                        + rsd[k][j][i][3] * rsd[k][j][i][3])
                    / rsd[k][j][i][0];
                flux[i][1] = rsd[k][j][i][1] * u21 + C2 * (rsd[k][j][i][4] - q);
                flux[i][2] = rsd[k][j][i][2] * u21;
                flux[i][3] = rsd[k][j][i][3] * u21;
                flux[i][4] = (C1 * rsd[k][j][i][4] - C2 * q) * u21;
            }
            for i in IST..IEND {
                for m in 0..5 as usize {
                    frct[k][j][i][m] = frct[k][j][i][m] - TX2 * (flux[i + 1][m] - flux[i - 1][m]);
                }
            }
            for i in IST..NX {
                tmp = 1.0 / rsd[k][j][i][0];
                u21i = tmp * rsd[k][j][i][1];
                u31i = tmp * rsd[k][j][i][2];
                u41i = tmp * rsd[k][j][i][3];
                u51i = tmp * rsd[k][j][i][4];
                tmp = 1.0 / rsd[k][j][i - 1][0];
                u21im1 = tmp * rsd[k][j][i - 1][1];
                u31im1 = tmp * rsd[k][j][i - 1][2];
                u41im1 = tmp * rsd[k][j][i - 1][3];
                u51im1 = tmp * rsd[k][j][i - 1][4];
                flux[i][1] = (4.0 / 3.0) * TX3 * (u21i - u21im1);
                flux[i][2] = TX3 * (u31i - u31im1);
                flux[i][3] = TX3 * (u41i - u41im1);
                flux[i][4] = 0.50
                    * (1.0 - C1 * C5)
                    * TX3
                    * ((u21i * u21i + u31i * u31i + u41i * u41i)
                        - (u21im1 * u21im1 + u31im1 * u31im1 + u41im1 * u41im1))
                    + (1.0 / 6.0) * TX3 * (u21i * u21i - u21im1 * u21im1)
                    + C1 * C5 * TX3 * (u51i - u51im1);
            }
            for i in IST..IEND {
                frct[k][j][i][0] = frct[k][j][i][0]
                    + DX1
                        * TX1
                        * (rsd[k][j][i - 1][0] - 2.0 * rsd[k][j][i][0] + rsd[k][j][i + 1][0]);
                frct[k][j][i][1] = frct[k][j][i][1]
                    + TX3 * C3 * C4 * (flux[i + 1][1] - flux[i][1])
                    + DX2
                        * TX1
                        * (rsd[k][j][i - 1][1] - 2.0 * rsd[k][j][i][1] + rsd[k][j][i + 1][1]);
                frct[k][j][i][2] = frct[k][j][i][2]
                    + TX3 * C3 * C4 * (flux[i + 1][2] - flux[i][2])
                    + DX3
                        * TX1
                        * (rsd[k][j][i - 1][2] - 2.0 * rsd[k][j][i][2] + rsd[k][j][i + 1][2]);
                frct[k][j][i][3] = frct[k][j][i][3]
                    + TX3 * C3 * C4 * (flux[i + 1][3] - flux[i][3])
                    + DX4
                        * TX1
                        * (rsd[k][j][i - 1][3] - 2.0 * rsd[k][j][i][3] + rsd[k][j][i + 1][3]);
                frct[k][j][i][4] = frct[k][j][i][4]
                    + TX3 * C3 * C4 * (flux[i + 1][4] - flux[i][4])
                    + DX5
                        * TX1
                        * (rsd[k][j][i - 1][4] - 2.0 * rsd[k][j][i][4] + rsd[k][j][i + 1][4]);
            }
            /*
             * ---------------------------------------------------------------------
             * fourth-order dissipation
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                frct[k][j][1][m] = frct[k][j][1][m]
                    - DSSP * (5.0 * rsd[k][j][1][m] - 4.0 * rsd[k][j][2][m] + rsd[k][j][3][m]);
                frct[k][j][2][m] = frct[k][j][2][m]
                    - DSSP
                        * (-4.0 * rsd[k][j][1][m] + 6.0 * rsd[k][j][2][m] - 4.0 * rsd[k][j][3][m]
                            + rsd[k][j][4][m]);
            }
            for i in 3..NX - 3 {
                for m in 0..5 {
                    frct[k][j][i][m] = frct[k][j][i][m]
                        - DSSP
                            * (rsd[k][j][i - 2][m] - 4.0 * rsd[k][j][i - 1][m]
                                + 6.0 * rsd[k][j][i][m]
                                - 4.0 * rsd[k][j][i + 1][m]
                                + rsd[k][j][i + 2][m]);
                }
            }
            for m in 0..5 {
                frct[k][j][NX - 3][m] = frct[k][j][NX - 3][m]
                    - DSSP
                        * (rsd[k][j][NX - 5][m] - 4.0 * rsd[k][j][NX - 4][m]
                            + 6.0 * rsd[k][j][NX - 3][m]
                            - 4.0 * rsd[k][j][NX - 2][m]);
                frct[k][j][NX - 2][m] = frct[k][j][NX - 2][m]
                    - DSSP
                        * (rsd[k][j][NX - 4][m] - 4.0 * rsd[k][j][NX - 3][m]
                            + 5.0 * rsd[k][j][NX - 2][m]);
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * eta-direction flux differences
     * ---------------------------------------------------------------------
     */
    for k in 1..NZ - 1 {
        for i in IST..IEND {
            for j in 0..NY {
                flux[j][0] = rsd[k][j][i][2];
                u31 = rsd[k][j][i][2] / rsd[k][j][i][0];
                q = 0.50
                    * (rsd[k][j][i][1] * rsd[k][j][i][1]
                        + rsd[k][j][i][2] * rsd[k][j][i][2]
                        + rsd[k][j][i][3] * rsd[k][j][i][3])
                    / rsd[k][j][i][0];
                flux[j][1] = rsd[k][j][i][1] * u31;
                flux[j][2] = rsd[k][j][i][2] * u31 + C2 * (rsd[k][j][i][4] - q);
                flux[j][3] = rsd[k][j][i][3] * u31;
                flux[j][4] = (C1 * rsd[k][j][i][4] - C2 * q) * u31;
            }
            for j in JST..JEND {
                for m in 0..5 {
                    frct[k][j][i][m] = frct[k][j][i][m] - TY2 * (flux[j + 1][m] - flux[j - 1][m]);
                }
            }
            for j in JST..NY {
                tmp = 1.0 / rsd[k][j][i][0];
                u21j = tmp * rsd[k][j][i][1];
                u31j = tmp * rsd[k][j][i][2];
                u41j = tmp * rsd[k][j][i][3];
                u51j = tmp * rsd[k][j][i][4];
                tmp = 1.0 / rsd[k][j - 1][i][0];
                u21jm1 = tmp * rsd[k][j - 1][i][1];
                u31jm1 = tmp * rsd[k][j - 1][i][2];
                u41jm1 = tmp * rsd[k][j - 1][i][3];
                u51jm1 = tmp * rsd[k][j - 1][i][4];
                flux[j][1] = TY3 * (u21j - u21jm1);
                flux[j][2] = (4.0 / 3.0) * TY3 * (u31j - u31jm1);
                flux[j][3] = TY3 * (u41j - u41jm1);
                flux[j][4] = 0.50
                    * (1.0 - C1 * C5)
                    * TY3
                    * ((u21j * u21j + u31j * u31j + u41j * u41j)
                        - (u21jm1 * u21jm1 + u31jm1 * u31jm1 + u41jm1 * u41jm1))
                    + (1.0 / 6.0) * TY3 * (u31j * u31j - u31jm1 * u31jm1)
                    + C1 * C5 * TY3 * (u51j - u51jm1);
            }
            for j in JST..JEND {
                frct[k][j][i][0] = frct[k][j][i][0]
                    + DY1
                        * TY1
                        * (rsd[k][j - 1][i][0] - 2.0 * rsd[k][j][i][0] + rsd[k][j + 1][i][0]);
                frct[k][j][i][1] = frct[k][j][i][1]
                    + TY3 * C3 * C4 * (flux[j + 1][1] - flux[j][1])
                    + DY2
                        * TY1
                        * (rsd[k][j - 1][i][1] - 2.0 * rsd[k][j][i][1] + rsd[k][j + 1][i][1]);
                frct[k][j][i][2] = frct[k][j][i][2]
                    + TY3 * C3 * C4 * (flux[j + 1][2] - flux[j][2])
                    + DY3
                        * TY1
                        * (rsd[k][j - 1][i][2] - 2.0 * rsd[k][j][i][2] + rsd[k][j + 1][i][2]);
                frct[k][j][i][3] = frct[k][j][i][3]
                    + TY3 * C3 * C4 * (flux[j + 1][3] - flux[j][3])
                    + DY4
                        * TY1
                        * (rsd[k][j - 1][i][3] - 2.0 * rsd[k][j][i][3] + rsd[k][j + 1][i][3]);
                frct[k][j][i][4] = frct[k][j][i][4]
                    + TY3 * C3 * C4 * (flux[j + 1][4] - flux[j][4])
                    + DY5
                        * TY1
                        * (rsd[k][j - 1][i][4] - 2.0 * rsd[k][j][i][4] + rsd[k][j + 1][i][4]);
            }
            /*
             * ---------------------------------------------------------------------
             * fourth-order dissipation
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                frct[k][1][i][m] = frct[k][1][i][m]
                    - DSSP * (5.0 * rsd[k][1][i][m] - 4.0 * rsd[k][2][i][m] + rsd[k][3][i][m]);
                frct[k][2][i][m] = frct[k][2][i][m]
                    - DSSP
                        * (-4.0 * rsd[k][1][i][m] + 6.0 * rsd[k][2][i][m] - 4.0 * rsd[k][3][i][m]
                            + rsd[k][4][i][m]);
            }
            for j in 3..NY - 3 {
                for m in 0..5 {
                    frct[k][j][i][m] = frct[k][j][i][m]
                        - DSSP
                            * (rsd[k][j - 2][i][m] - 4.0 * rsd[k][j - 1][i][m]
                                + 6.0 * rsd[k][j][i][m]
                                - 4.0 * rsd[k][j + 1][i][m]
                                + rsd[k][j + 2][i][m]);
                }
            }
            for m in 0..5 {
                frct[k][NY - 3][i][m] = frct[k][NY - 3][i][m]
                    - DSSP
                        * (rsd[k][NY - 5][i][m] - 4.0 * rsd[k][NY - 4][i][m]
                            + 6.0 * rsd[k][NY - 3][i][m]
                            - 4.0 * rsd[k][NY - 2][i][m]);
                frct[k][NY - 2][i][m] = frct[k][NY - 2][i][m]
                    - DSSP
                        * (rsd[k][NY - 4][i][m] - 4.0 * rsd[k][NY - 3][i][m]
                            + 5.0 * rsd[k][NY - 2][i][m]);
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * zeta-direction flux differences
     * ---------------------------------------------------------------------
     */
    for j in JST..JEND {
        for i in IST..IEND {
            for k in 0..NZ {
                flux[k][0] = rsd[k][j][i][3];
                u41 = rsd[k][j][i][3] / rsd[k][j][i][0];
                q = 0.50
                    * (rsd[k][j][i][1] * rsd[k][j][i][1]
                        + rsd[k][j][i][2] * rsd[k][j][i][2]
                        + rsd[k][j][i][3] * rsd[k][j][i][3])
                    / rsd[k][j][i][0];
                flux[k][1] = rsd[k][j][i][1] * u41;
                flux[k][2] = rsd[k][j][i][2] * u41;
                flux[k][3] = rsd[k][j][i][3] * u41 + C2 * (rsd[k][j][i][4] - q);
                flux[k][4] = (C1 * rsd[k][j][i][4] - C2 * q) * u41;
            }
            for k in 1..NZ - 1 {
                for m in 0..5 {
                    frct[k][j][i][m] = frct[k][j][i][m] - TZ2 * (flux[k + 1][m] - flux[k - 1][m]);
                }
            }
            for k in 1..NZ {
                tmp = 1.0 / rsd[k][j][i][0];
                u21k = tmp * rsd[k][j][i][1];
                u31k = tmp * rsd[k][j][i][2];
                u41k = tmp * rsd[k][j][i][3];
                u51k = tmp * rsd[k][j][i][4];
                tmp = 1.0 / rsd[k - 1][j][i][0];
                u21km1 = tmp * rsd[k - 1][j][i][1];
                u31km1 = tmp * rsd[k - 1][j][i][2];
                u41km1 = tmp * rsd[k - 1][j][i][3];
                u51km1 = tmp * rsd[k - 1][j][i][4];
                flux[k][1] = TZ3 * (u21k - u21km1);
                flux[k][2] = TZ3 * (u31k - u31km1);
                flux[k][3] = (4.0 / 3.0) * TZ3 * (u41k - u41km1);
                flux[k][4] = 0.50
                    * (1.0 - C1 * C5)
                    * TZ3
                    * ((u21k * u21k + u31k * u31k + u41k * u41k)
                        - (u21km1 * u21km1 + u31km1 * u31km1 + u41km1 * u41km1))
                    + (1.0 / 6.0) * TZ3 * (u41k * u41k - u41km1 * u41km1)
                    + C1 * C5 * TZ3 * (u51k - u51km1);
            }
            for k in 1..NZ - 1 {
                frct[k][j][i][0] = frct[k][j][i][0]
                    + DZ1
                        * TZ1
                        * (rsd[k + 1][j][i][0] - 2.0 * rsd[k][j][i][0] + rsd[k - 1][j][i][0]);
                frct[k][j][i][1] = frct[k][j][i][1]
                    + TZ3 * C3 * C4 * (flux[k + 1][1] - flux[k][1])
                    + DZ2
                        * TZ1
                        * (rsd[k + 1][j][i][1] - 2.0 * rsd[k][j][i][1] + rsd[k - 1][j][i][1]);
                frct[k][j][i][2] = frct[k][j][i][2]
                    + TZ3 * C3 * C4 * (flux[k + 1][2] - flux[k][2])
                    + DZ3
                        * TZ1
                        * (rsd[k + 1][j][i][2] - 2.0 * rsd[k][j][i][2] + rsd[k - 1][j][i][2]);
                frct[k][j][i][3] = frct[k][j][i][3]
                    + TZ3 * C3 * C4 * (flux[k + 1][3] - flux[k][3])
                    + DZ4
                        * TZ1
                        * (rsd[k + 1][j][i][3] - 2.0 * rsd[k][j][i][3] + rsd[k - 1][j][i][3]);
                frct[k][j][i][4] = frct[k][j][i][4]
                    + TZ3 * C3 * C4 * (flux[k + 1][4] - flux[k][4])
                    + DZ5
                        * TZ1
                        * (rsd[k + 1][j][i][4] - 2.0 * rsd[k][j][i][4] + rsd[k - 1][j][i][4]);
            }
            /*
             * ---------------------------------------------------------------------
             * fourth-order dissipation
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                frct[1][j][i][m] = frct[1][j][i][m]
                    - DSSP * (5.0 * rsd[1][j][i][m] - 4.0 * rsd[2][j][i][m] + rsd[3][j][i][m]);
                frct[2][j][i][m] = frct[2][j][i][m]
                    - DSSP
                        * (-4.0 * rsd[1][j][i][m] + 6.0 * rsd[2][j][i][m] - 4.0 * rsd[3][j][i][m]
                            + rsd[4][j][i][m]);
            }
            for k in 3..NZ - 3 {
                for m in 0..5 {
                    frct[k][j][i][m] = frct[k][j][i][m]
                        - DSSP
                            * (rsd[k - 2][j][i][m] - 4.0 * rsd[k - 1][j][i][m]
                                + 6.0 * rsd[k][j][i][m]
                                - 4.0 * rsd[k + 1][j][i][m]
                                + rsd[k + 2][j][i][m]);
                }
            }
            for m in 0..5 {
                frct[NZ - 3][j][i][m] = frct[NZ - 3][j][i][m]
                    - DSSP
                        * (rsd[NZ - 5][j][i][m] - 4.0 * rsd[NZ - 4][j][i][m]
                            + 6.0 * rsd[NZ - 3][j][i][m]
                            - 4.0 * rsd[NZ - 2][j][i][m]);
                frct[NZ - 2][j][i][m] = frct[NZ - 2][j][i][m]
                    - DSSP
                        * (rsd[NZ - 4][j][i][m] - 4.0 * rsd[NZ - 3][j][i][m]
                            + 5.0 * rsd[NZ - 2][j][i][m]);
            }
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * compute the solution error
 * ---------------------------------------------------------------------
 */
fn error(errnm: &mut [f64], u: &[[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]], ce: &[[f64; 5]]) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let mut tmp: f64;
    let mut u000ijk: [f64; 5] = [0.0; 5];
    errnm.iter_mut().for_each(|errnm| *errnm = 0.0);
    for k in 1..NZ - 1 {
        for j in JST..JEND {
            for i in IST..IEND {
                exact(i, j, k, &mut u000ijk[..], &ce[..]);
                for m in 0..5 {
                    tmp = u000ijk[m] - u[k][j][i][m];
                    errnm[m] = errnm[m] + tmp * tmp;
                }
            }
        }
    }
    errnm.iter_mut().for_each(|errnm| {
        *errnm = f64::sqrt(*errnm / ((NX0 - 2) * (NY0 - 2) * (NZ0 - 2)) as f64);
    });
}

/*
 * ---------------------------------------------------------------------
 * to compute the l2-norm of vector v.
 * ---------------------------------------------------------------------
 * to improve cache performance, second two dimensions padded by 1
 * for even number sizes only.  Only needed in v.
 * ---------------------------------------------------------------------
 */
fn l2norm(v: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]], sum: &mut [f64]) {
    /*
    	* ---------------------------------------------------------------------
    	* local variables
    	* ---------------------------------------------------------------------
    	*/
    for m in 0..5 {
        sum[m] = 0.0;
    }
    for k in 1..NZ0 - 1 {
        for j in JST..JEND {
            for i in IST..IEND {
                for m in 0..5 {
                    sum[m] = sum[m] + v[k][j][i][m] * v[k][j][i][m];
                }
            }
        }
    }
    sum.iter_mut().for_each(|sum| {
        *sum = f64::sqrt(*sum / ((NX0 - 2) * (NY0 - 2) * (NZ0 - 2)) as f64);
    });
}

fn pintgr(u: &[[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]], frc: &mut f64) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let (ibeg, ifin, ifin1): (usize, usize, usize);
    let (jbeg, jfin, jfin1): (usize, usize, usize);
    let mut phi1: [[f64; ISIZ2 + 2]; ISIZ3 + 2] = [[0.0; ISIZ2 + 2]; ISIZ3 + 2];
    let mut phi2: [[f64; ISIZ2 + 2]; ISIZ3 + 2] = [[0.0; ISIZ2 + 2]; ISIZ3 + 2];
    let (mut frc1, mut frc2, mut frc3): (f64, f64, f64);
    /*
     * ---------------------------------------------------------------------
     * set up the sub-domains for integeration in each processor
     * ---------------------------------------------------------------------
     */
    ibeg = II1;
    ifin = II2;
    jbeg = JI1;
    jfin = JI2;
    ifin1 = ifin - 1;
    jfin1 = jfin - 1;
    /*
     * ---------------------------------------------------------------------
     * initialize
     * ---------------------------------------------------------------------
     */
    for i in 0..ISIZ2 + 2 {
        for k in 0..ISIZ3 + 2 {
            phi1[k][i] = 0.0;
            phi2[k][i] = 0.0;
        }
    }
    for j in jbeg..jfin {
        for i in ibeg..ifin {
            let mut k = KI1;
            phi1[j][i] = C2
                * (u[k][j][i][4]
                    - 0.50
                        * (u[k][j][i][1] * u[k][j][i][1]
                            + u[k][j][i][2] * u[k][j][i][2]
                            + u[k][j][i][3] * u[k][j][i][3])
                        / u[k][j][i][0]);
            k = KI2 - 1;
            phi2[j][i] = C2
                * (u[k][j][i][4]
                    - 0.50
                        * (u[k][j][i][1] * u[k][j][i][1]
                            + u[k][j][i][2] * u[k][j][i][2]
                            + u[k][j][i][3] * u[k][j][i][3])
                        / u[k][j][i][0]);
        }
    }
    frc1 = 0.0;
    for j in jbeg..jfin1 {
        for i in ibeg..ifin1 {
            frc1 = frc1
                + (phi1[j][i]
                    + phi1[j][i + 1]
                    + phi1[j + 1][i]
                    + phi1[j + 1][i + 1]
                    + phi2[j][i]
                    + phi2[j][i + 1]
                    + phi2[j + 1][i]
                    + phi2[j + 1][i + 1]);
        }
    }
    frc1 = DXI * DETA * frc1;
    /*
     * ---------------------------------------------------------------------
     * initialize
     * ---------------------------------------------------------------------
     */
    for i in 0..ISIZ2 + 2 {
        for k in 0..ISIZ3 + 2 {
            phi1[k][i] = 0.0;
            phi2[k][i] = 0.0;
        }
    }
    if jbeg == JI1 {
        for k in KI1..KI2 {
            for i in ibeg..ifin {
                phi1[k][i] = C2
                    * (u[k][jbeg][i][4]
                        - 0.50
                            * (u[k][jbeg][i][1] * u[k][jbeg][i][1]
                                + u[k][jbeg][i][2] * u[k][jbeg][i][2]
                                + u[k][jbeg][i][3] * u[k][jbeg][i][3])
                            / u[k][jbeg][i][0]);
            }
        }
    }
    if jfin == JI2 {
        for k in KI1..KI2 {
            for i in ibeg..ifin {
                phi2[k][i] = C2
                    * (u[k][jfin - 1][i][4]
                        - 0.50
                            * (u[k][jfin - 1][i][1] * u[k][jfin - 1][i][1]
                                + u[k][jfin - 1][i][2] * u[k][jfin - 1][i][2]
                                + u[k][jfin - 1][i][3] * u[k][jfin - 1][i][3])
                            / u[k][jfin - 1][i][0]);
            }
        }
    }
    frc2 = 0.0;
    for k in KI1..KI2 - 1 {
        for i in ibeg..ifin1 {
            frc2 = frc2
                + (phi1[k][i]
                    + phi1[k][i + 1]
                    + phi1[k + 1][i]
                    + phi1[k + 1][i + 1]
                    + phi2[k][i]
                    + phi2[k][i + 1]
                    + phi2[k + 1][i]
                    + phi2[k + 1][i + 1]);
        }
    }
    frc2 = DXI * DZETA * frc2;
    /*
     * ---------------------------------------------------------------------
     * initialize
     * ---------------------------------------------------------------------
     */
    for i in 0..ISIZ2 + 2 {
        for k in 0..ISIZ3 + 2 {
            phi1[k][i] = 0.0;
            phi2[k][i] = 0.0;
        }
    }
    if ibeg == II1 {
        for k in KI1..KI2 {
            for j in jbeg..jfin {
                phi1[k][j] = C2
                    * (u[k][j][ibeg][4]
                        - 0.50
                            * (u[k][j][ibeg][1] * u[k][j][ibeg][1]
                                + u[k][j][ibeg][2] * u[k][j][ibeg][2]
                                + u[k][j][ibeg][3] * u[k][j][ibeg][3])
                            / u[k][j][ibeg][0]);
            }
        }
    }
    if ifin == II2 {
        for k in KI1..KI2 {
            for j in jbeg..jfin {
                phi2[k][j] = C2
                    * (u[k][j][ifin - 1][4]
                        - 0.50
                            * (u[k][j][ifin - 1][1] * u[k][j][ifin - 1][1]
                                + u[k][j][ifin - 1][2] * u[k][j][ifin - 1][2]
                                + u[k][j][ifin - 1][3] * u[k][j][ifin - 1][3])
                            / u[k][j][ifin - 1][0]);
            }
        }
    }
    frc3 = 0.0;
    for k in KI1..KI2 - 1 {
        for j in jbeg..jfin1 {
            frc3 = frc3
                + (phi1[k][j]
                    + phi1[k][j + 1]
                    + phi1[k + 1][j]
                    + phi1[k + 1][j + 1]
                    + phi2[k][j]
                    + phi2[k][j + 1]
                    + phi2[k + 1][j]
                    + phi2[k + 1][j + 1]);
        }
    }
    frc3 = DETA * DZETA * frc3;
    *frc = 0.25 * (frc1 + frc2 + frc3);
}

/*
 * ---------------------------------------------------------------------
 * compute the right hand sides
 * ---------------------------------------------------------------------
 */
fn rhs(
    frct: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    rsd: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    rho_i: &mut [[[f64; ISIZ1 + 1]; ISIZ2 + 1]],
    qs: &mut [[[f64; ISIZ1 + 1]; ISIZ2 + 1]],
    flux: &mut [[f64; 5]],
    u: &[[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    timers: &mut Timer,
) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let mut q: f64;
    let mut tmp: f64;
    let mut utmp: [[f64; 6]; ISIZ3] = [[0.0; 6]; ISIZ3];
    let mut rtmp: [[f64; 6]; ISIZ3] = [[0.0; 6]; ISIZ3];
    let (mut u21, mut u31, mut u41): (f64, f64, f64);
    let (mut u21i, mut u31i, mut u41i, mut u51i): (f64, f64, f64, f64);
    let (mut u21j, mut u31j, mut u41j, mut u51j): (f64, f64, f64, f64);
    let (mut u21k, mut u31k, mut u41k, mut u51k): (f64, f64, f64, f64);
    let (mut u21im1, mut u31im1, mut u41im1, mut u51im1): (f64, f64, f64, f64);
    let (mut u21jm1, mut u31jm1, mut u41jm1, mut u51jm1): (f64, f64, f64, f64);
    let (mut u21km1, mut u31km1, mut u41km1, mut u51km1): (f64, f64, f64, f64);

    if TIMERS {
        timers.start(T_RHS);
    }
    for k in 0..NZ {
        for j in 0..NY {
            for i in 0..NX {
                for m in 0..5 {
                    rsd[k][j][i][m] = -frct[k][j][i][m];
                }
                tmp = 1.0 / u[k][j][i][0];
                rho_i[k][j][i] = tmp;
                qs[k][j][i] = 0.50
                    * (u[k][j][i][1] * u[k][j][i][1]
                        + u[k][j][i][2] * u[k][j][i][2]
                        + u[k][j][i][3] * u[k][j][i][3])
                    * tmp;
            }
        }
    }
    if TIMERS {
        timers.start(T_RHSX);
    }
    /*
     * ---------------------------------------------------------------------
     * xi-direction flux differences
     * ---------------------------------------------------------------------
     */
    for k in 1..NZ - 1 {
        for j in JST..JEND {
            for i in 0..NX {
                flux[i][0] = u[k][j][i][1];
                u21 = u[k][j][i][1] * rho_i[k][j][i];
                q = qs[k][j][i];
                flux[i][1] = u[k][j][i][1] * u21 + C2 * (u[k][j][i][4] - q);
                flux[i][2] = u[k][j][i][2] * u21;
                flux[i][3] = u[k][j][i][3] * u21;
                flux[i][4] = (C1 * u[k][j][i][4] - C2 * q) * u21;
            }
            for i in IST..IEND {
                for m in 0..5 {
                    rsd[k][j][i][m] = rsd[k][j][i][m] - TX2 * (flux[i + 1][m] - flux[i - 1][m]);
                }
            }
            for i in IST..NX {
                tmp = rho_i[k][j][i];
                u21i = tmp * u[k][j][i][1];
                u31i = tmp * u[k][j][i][2];
                u41i = tmp * u[k][j][i][3];
                u51i = tmp * u[k][j][i][4];
                tmp = rho_i[k][j][i - 1];
                u21im1 = tmp * u[k][j][i - 1][1];
                u31im1 = tmp * u[k][j][i - 1][2];
                u41im1 = tmp * u[k][j][i - 1][3];
                u51im1 = tmp * u[k][j][i - 1][4];
                flux[i][1] = (4.0 / 3.0) * TX3 * (u21i - u21im1);
                flux[i][2] = TX3 * (u31i - u31im1);
                flux[i][3] = TX3 * (u41i - u41im1);
                flux[i][4] = 0.50
                    * (1.0 - C1 * C5)
                    * TX3
                    * ((u21i * u21i + u31i * u31i + u41i * u41i)
                        - (u21im1 * u21im1 + u31im1 * u31im1 + u41im1 * u41im1))
                    + (1.0 / 6.0) * TX3 * (u21i * u21i - u21im1 * u21im1)
                    + C1 * C5 * TX3 * (u51i - u51im1);
            }
            for i in IST..IEND {
                rsd[k][j][i][0] = rsd[k][j][i][0]
                    + DX1 * TX1 * (u[k][j][i - 1][0] - 2.0 * u[k][j][i][0] + u[k][j][i + 1][0]);
                rsd[k][j][i][1] = rsd[k][j][i][1]
                    + TX3 * C3 * C4 * (flux[i + 1][1] - flux[i][1])
                    + DX2 * TX1 * (u[k][j][i - 1][1] - 2.0 * u[k][j][i][1] + u[k][j][i + 1][1]);
                rsd[k][j][i][2] = rsd[k][j][i][2]
                    + TX3 * C3 * C4 * (flux[i + 1][2] - flux[i][2])
                    + DX3 * TX1 * (u[k][j][i - 1][2] - 2.0 * u[k][j][i][2] + u[k][j][i + 1][2]);
                rsd[k][j][i][3] = rsd[k][j][i][3]
                    + TX3 * C3 * C4 * (flux[i + 1][3] - flux[i][3])
                    + DX4 * TX1 * (u[k][j][i - 1][3] - 2.0 * u[k][j][i][3] + u[k][j][i + 1][3]);
                rsd[k][j][i][4] = rsd[k][j][i][4]
                    + TX3 * C3 * C4 * (flux[i + 1][4] - flux[i][4])
                    + DX5 * TX1 * (u[k][j][i - 1][4] - 2.0 * u[k][j][i][4] + u[k][j][i + 1][4]);
            }
            /*
             * ---------------------------------------------------------------------
             * fourth-order dissipation
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                rsd[k][j][1][m] = rsd[k][j][1][m]
                    - DSSP * (5.0 * u[k][j][1][m] - 4.0 * u[k][j][2][m] + u[k][j][3][m]);
                rsd[k][j][2][m] = rsd[k][j][2][m]
                    - DSSP
                        * (-4.0 * u[k][j][1][m] + 6.0 * u[k][j][2][m] - 4.0 * u[k][j][3][m]
                            + u[k][j][4][m]);
            }
            for i in 3..NX - 3 {
                for m in 0..5 {
                    rsd[k][j][i][m] = rsd[k][j][i][m]
                        - DSSP
                            * (u[k][j][i - 2][m] - 4.0 * u[k][j][i - 1][m] + 6.0 * u[k][j][i][m]
                                - 4.0 * u[k][j][i + 1][m]
                                + u[k][j][i + 2][m]);
                }
            }
            for m in 0..5 {
                rsd[k][j][NX - 3][m] = rsd[k][j][NX - 3][m]
                    - DSSP
                        * (u[k][j][NX - 5][m] - 4.0 * u[k][j][NX - 4][m]
                            + 6.0 * u[k][j][NX - 3][m]
                            - 4.0 * u[k][j][NX - 2][m]);
                rsd[k][j][NX - 2][m] = rsd[k][j][NX - 2][m]
                    - DSSP
                        * (u[k][j][NX - 4][m] - 4.0 * u[k][j][NX - 3][m]
                            + 5.0 * u[k][j][NX - 2][m]);
            }
        }
    }
    if TIMERS {
        timers.stop(T_RHSX);
    }
    if TIMERS {
        timers.start(T_RHSY);
    }
    /*
     * ---------------------------------------------------------------------
     * eta-direction flux differences
     * ---------------------------------------------------------------------
     */
    for k in 1..NZ - 1 {
        for i in IST..IEND {
            for j in 0..NY {
                flux[j][0] = u[k][j][i][2];
                u31 = u[k][j][i][2] * rho_i[k][j][i];
                q = qs[k][j][i];
                flux[j][1] = u[k][j][i][1] * u31;
                flux[j][2] = u[k][j][i][2] * u31 + C2 * (u[k][j][i][4] - q);
                flux[j][3] = u[k][j][i][3] * u31;
                flux[j][4] = (C1 * u[k][j][i][4] - C2 * q) * u31;
            }
            for j in JST..JEND {
                for m in 0..5 {
                    rsd[k][j][i][m] = rsd[k][j][i][m] - TY2 * (flux[j + 1][m] - flux[j - 1][m]);
                }
            }
            for j in JST..NY {
                tmp = rho_i[k][j][i];
                u21j = tmp * u[k][j][i][1];
                u31j = tmp * u[k][j][i][2];
                u41j = tmp * u[k][j][i][3];
                u51j = tmp * u[k][j][i][4];
                tmp = rho_i[k][j - 1][i];
                u21jm1 = tmp * u[k][j - 1][i][1];
                u31jm1 = tmp * u[k][j - 1][i][2];
                u41jm1 = tmp * u[k][j - 1][i][3];
                u51jm1 = tmp * u[k][j - 1][i][4];
                flux[j][1] = TY3 * (u21j - u21jm1);
                flux[j][2] = (4.0 / 3.0) * TY3 * (u31j - u31jm1);
                flux[j][3] = TY3 * (u41j - u41jm1);
                flux[j][4] = 0.50
                    * (1.0 - C1 * C5)
                    * TY3
                    * ((u21j * u21j + u31j * u31j + u41j * u41j)
                        - (u21jm1 * u21jm1 + u31jm1 * u31jm1 + u41jm1 * u41jm1))
                    + (1.0 / 6.0) * TY3 * (u31j * u31j - u31jm1 * u31jm1)
                    + C1 * C5 * TY3 * (u51j - u51jm1);
            }
            for j in JST..JEND {
                rsd[k][j][i][0] = rsd[k][j][i][0]
                    + DY1 * TY1 * (u[k][j - 1][i][0] - 2.0 * u[k][j][i][0] + u[k][j + 1][i][0]);
                rsd[k][j][i][1] = rsd[k][j][i][1]
                    + TY3 * C3 * C4 * (flux[j + 1][1] - flux[j][1])
                    + DY2 * TY1 * (u[k][j - 1][i][1] - 2.0 * u[k][j][i][1] + u[k][j + 1][i][1]);
                rsd[k][j][i][2] = rsd[k][j][i][2]
                    + TY3 * C3 * C4 * (flux[j + 1][2] - flux[j][2])
                    + DY3 * TY1 * (u[k][j - 1][i][2] - 2.0 * u[k][j][i][2] + u[k][j + 1][i][2]);
                rsd[k][j][i][3] = rsd[k][j][i][3]
                    + TY3 * C3 * C4 * (flux[j + 1][3] - flux[j][3])
                    + DY4 * TY1 * (u[k][j - 1][i][3] - 2.0 * u[k][j][i][3] + u[k][j + 1][i][3]);
                rsd[k][j][i][4] = rsd[k][j][i][4]
                    + TY3 * C3 * C4 * (flux[j + 1][4] - flux[j][4])
                    + DY5 * TY1 * (u[k][j - 1][i][4] - 2.0 * u[k][j][i][4] + u[k][j + 1][i][4]);
            }
        }
        /*
         * ---------------------------------------------------------------------
         * fourth-order dissipation
         * ---------------------------------------------------------------------
         */
        for i in IST..IEND {
            for m in 0..5 {
                rsd[k][1][i][m] = rsd[k][1][i][m]
                    - DSSP * (5.0 * u[k][1][i][m] - 4.0 * u[k][2][i][m] + u[k][3][i][m]);
                rsd[k][2][i][m] = rsd[k][2][i][m]
                    - DSSP
                        * (-4.0 * u[k][1][i][m] + 6.0 * u[k][2][i][m] - 4.0 * u[k][3][i][m]
                            + u[k][4][i][m]);
            }
        }
        for j in 3..NY - 3 {
            for i in IST..IEND {
                for m in 0..5 {
                    rsd[k][j][i][m] = rsd[k][j][i][m]
                        - DSSP
                            * (u[k][j - 2][i][m] - 4.0 * u[k][j - 1][i][m] + 6.0 * u[k][j][i][m]
                                - 4.0 * u[k][j + 1][i][m]
                                + u[k][j + 2][i][m]);
                }
            }
        }
        for i in IST..IEND {
            for m in 0..5 {
                rsd[k][NY - 3][i][m] = rsd[k][NY - 3][i][m]
                    - DSSP
                        * (u[k][NY - 5][i][m] - 4.0 * u[k][NY - 4][i][m]
                            + 6.0 * u[k][NY - 3][i][m]
                            - 4.0 * u[k][NY - 2][i][m]);
                rsd[k][NY - 2][i][m] = rsd[k][NY - 2][i][m]
                    - DSSP
                        * (u[k][NY - 4][i][m] - 4.0 * u[k][NY - 3][i][m]
                            + 5.0 * u[k][NY - 2][i][m]);
            }
        }
    }
    if TIMERS {
        timers.stop(T_RHSY);
    }
    if TIMERS {
        timers.start(T_RHSZ);
    }
    /*
     * ---------------------------------------------------------------------
     * zeta-direction flux differences
     * ---------------------------------------------------------------------
     */
    for j in JST..JEND {
        for i in IST..IEND {
            for k in 0..NZ {
                utmp[k][0] = u[k][j][i][0];
                utmp[k][1] = u[k][j][i][1];
                utmp[k][2] = u[k][j][i][2];
                utmp[k][3] = u[k][j][i][3];
                utmp[k][4] = u[k][j][i][4];
                utmp[k][5] = rho_i[k][j][i];
            }
            for k in 0..NZ {
                flux[k][0] = utmp[k][3];
                u41 = utmp[k][3] * utmp[k][5];
                q = qs[k][j][i];
                flux[k][1] = utmp[k][1] * u41;
                flux[k][2] = utmp[k][2] * u41;
                flux[k][3] = utmp[k][3] * u41 + C2 * (utmp[k][4] - q);
                flux[k][4] = (C1 * utmp[k][4] - C2 * q) * u41;
            }
            for k in 1..NZ - 1 {
                for m in 0..5 {
                    rtmp[k][m] = rsd[k][j][i][m] - TZ2 * (flux[k + 1][m] - flux[k - 1][m]);
                }
            }
            for k in 1..NZ {
                tmp = utmp[k][5];
                u21k = tmp * utmp[k][1];
                u31k = tmp * utmp[k][2];
                u41k = tmp * utmp[k][3];
                u51k = tmp * utmp[k][4];
                tmp = utmp[k - 1][5];
                u21km1 = tmp * utmp[k - 1][1];
                u31km1 = tmp * utmp[k - 1][2];
                u41km1 = tmp * utmp[k - 1][3];
                u51km1 = tmp * utmp[k - 1][4];
                flux[k][1] = TZ3 * (u21k - u21km1);
                flux[k][2] = TZ3 * (u31k - u31km1);
                flux[k][3] = (4.0 / 3.0) * TZ3 * (u41k - u41km1);
                flux[k][4] = 0.50
                    * (1.0 - C1 * C5)
                    * TZ3
                    * ((u21k * u21k + u31k * u31k + u41k * u41k)
                        - (u21km1 * u21km1 + u31km1 * u31km1 + u41km1 * u41km1))
                    + (1.0 / 6.0) * TZ3 * (u41k * u41k - u41km1 * u41km1)
                    + C1 * C5 * TZ3 * (u51k - u51km1);
            }
            for k in 1..NZ - 1 {
                rtmp[k][0] =
                    rtmp[k][0] + DZ1 * TZ1 * (utmp[k - 1][0] - 2.0 * utmp[k][0] + utmp[k + 1][0]);
                rtmp[k][1] = rtmp[k][1]
                    + TZ3 * C3 * C4 * (flux[k + 1][1] - flux[k][1])
                    + DZ2 * TZ1 * (utmp[k - 1][1] - 2.0 * utmp[k][1] + utmp[k + 1][1]);
                rtmp[k][2] = rtmp[k][2]
                    + TZ3 * C3 * C4 * (flux[k + 1][2] - flux[k][2])
                    + DZ3 * TZ1 * (utmp[k - 1][2] - 2.0 * utmp[k][2] + utmp[k + 1][2]);
                rtmp[k][3] = rtmp[k][3]
                    + TZ3 * C3 * C4 * (flux[k + 1][3] - flux[k][3])
                    + DZ4 * TZ1 * (utmp[k - 1][3] - 2.0 * utmp[k][3] + utmp[k + 1][3]);
                rtmp[k][4] = rtmp[k][4]
                    + TZ3 * C3 * C4 * (flux[k + 1][4] - flux[k][4])
                    + DZ5 * TZ1 * (utmp[k - 1][4] - 2.0 * utmp[k][4] + utmp[k + 1][4]);
            }
            /*
             * ---------------------------------------------------------------------
             * fourth-order dissipation
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                rsd[1][j][i][m] =
                    rtmp[1][m] - DSSP * (5.0 * utmp[1][m] - 4.0 * utmp[2][m] + utmp[3][m]);
                rsd[2][j][i][m] = rtmp[2][m]
                    - DSSP * (-4.0 * utmp[1][m] + 6.0 * utmp[2][m] - 4.0 * utmp[3][m] + utmp[4][m]);
            }
            for k in 3..NZ - 3 {
                for m in 0..5 {
                    rsd[k][j][i][m] = rtmp[k][m]
                        - DSSP
                            * (utmp[k - 2][m] - 4.0 * utmp[k - 1][m] + 6.0 * utmp[k][m]
                                - 4.0 * utmp[k + 1][m]
                                + utmp[k + 2][m]);
                }
            }
            for m in 0..5 {
                rsd[NZ - 3][j][i][m] = rtmp[NZ - 3][m]
                    - DSSP
                        * (utmp[NZ - 5][m] - 4.0 * utmp[NZ - 4][m] + 6.0 * utmp[NZ - 3][m]
                            - 4.0 * utmp[NZ - 2][m]);
                rsd[NZ - 2][j][i][m] = rtmp[NZ - 2][m]
                    - DSSP * (utmp[NZ - 4][m] - 4.0 * utmp[NZ - 3][m] + 5.0 * utmp[NZ - 2][m]);
            }
        }
    }
    if TIMERS {
        timers.stop(T_RHSZ);
    }
    if TIMERS {
        timers.stop(T_RHS);
    }
}

fn setbv(u: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]], ce: &[[f64; 5]]) {
    let mut temp1: [f64; 5] = [0.0; 5];
    let mut temp2: [f64; 5] = [0.0; 5];
    /*
     * ---------------------------------------------------------------------
     * set the dependent variable values along the top and bottom faces
     * ---------------------------------------------------------------------
     */
    for j in 0..NY {
        for i in 0..NX {
            exact(i, j, 0, &mut temp1[..], &ce[..]);
            exact(i, j, NZ - 1, &mut temp2[..], &ce[..]);
            for m in 0..5 {
                u[0][j][i][m] = temp1[m];
                u[NZ - 1][j][i][m] = temp2[m];
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * set the dependent variable values along north and south faces
     * ---------------------------------------------------------------------
     */
    for k in 0..NZ {
        for i in 0..NX {
            exact(i, 0, k, &mut temp1[..], &ce[..]);
            exact(i, NY - 1, k, &mut temp2[..], &ce[..]);
            for m in 0..5 {
                u[k][0][i][m] = temp1[m];
                u[k][NY - 1][i][m] = temp2[m];
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * set the dependent variable values along east and west faces
     * ---------------------------------------------------------------------
     */
    for k in 0..NZ {
        for j in 0..NY {
            exact(0, j, k, &mut temp1[..], &ce[..]);
            exact(NX - 1, j, k, &mut temp2[..], &ce[..]);
            for m in 0..5 {
                u[k][j][0][m] = temp1[m];
                u[k][j][NX - 1][m] = temp2[m];
            }
        }
    }
}

fn setiv(u: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]], ce: &[[f64; 5]]) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let (mut pxi, mut peta, mut pzeta): (f64, f64, f64);
    let mut ue_1jk: [f64; 5] = [0.0; 5];
    let mut ue_nx0jk: [f64; 5] = [0.0; 5];
    let mut ue_i1k: [f64; 5] = [0.0; 5];
    let mut ue_iny0k: [f64; 5] = [0.0; 5];
    let mut ue_ij1: [f64; 5] = [0.0; 5];
    let mut ue_ijnz: [f64; 5] = [0.0; 5];
    for k in 1..NZ - 1 {
        let zeta = k as f64 / (NZ as f64 - 1.0);
        for j in 1..NY - 1 {
            let eta = j as f64 / (NY0 as f64 - 1.0);
            for i in 1..NX - 1 {
                let xi = i as f64 / (NX0 as f64 - 1.0);
                exact(0, j, k, &mut ue_1jk[..], &ce[..]);
                exact(NX0 - 1, j, k, &mut ue_nx0jk[..], &ce[..]);
                exact(i, 0, k, &mut ue_i1k[..], &ce[..]);
                exact(i, NY0 - 1, k, &mut ue_iny0k[..], &ce[..]);
                exact(i, j, 0, &mut ue_ij1[..], &ce[..]);
                exact(i, j, NZ - 1, &mut ue_ijnz[..], &ce[..]);
                for m in 0..5 {
                    pxi = (1.0 - xi) * ue_1jk[m] + xi * ue_nx0jk[m];
                    peta = (1.0 - eta) * ue_i1k[m] + eta * ue_iny0k[m];
                    pzeta = (1.0 - zeta) * ue_ij1[m] + zeta * ue_ijnz[m];
                    u[k][j][i][m] = pxi + peta + pzeta - pxi * peta - peta * pzeta - pzeta * pxi
                        + pxi * peta * pzeta;
                }
            }
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * to perform pseudo-time stepping SSOR iterations
 * for five nonlinear pde's.
 * ---------------------------------------------------------------------
 */
fn ssor(
    niter: usize,
    a: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    b: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    c: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    d: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    rsd: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    frct: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    qs: &mut [[[f64; ISIZ1 + 1]; ISIZ2 + 1]],
    flux: &mut [[f64; 5]],
    rho_i: &mut [[[f64; ISIZ1 + 1]; ISIZ2 + 1]],
    u: &mut [[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    rsdnm: &mut [f64],
    tolrsd: &mut [f64],
    maxtime: &mut f64,
    timers: &mut Timer,
) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let tmp: f64;
    let mut tv: [f64; ISIZ2 * (ISIZ1 + 1) * 5] = [0.0; ISIZ2 * (ISIZ1 + 1) * 5];
    let mut delunm: [f64; 5] = [0.0; 5];
    /*
     * ---------------------------------------------------------------------
     * begin pseudo-time stepping iterations
     * ---------------------------------------------------------------------
     */
    tmp = 1.0 / (OMEGA_DEFAULT * (2.0 - OMEGA_DEFAULT));
    /*
     * ---------------------------------------------------------------------
     * initialize a,b,c,d to zero (guarantees that page tables have been
     * formed, if applicable on given architecture, before timestepping).
     * ---------------------------------------------------------------------
     */
    for j in 0..ISIZ2 {
        for i in 0..ISIZ1 {
            for n in 0..5 {
                for m in 0..5 {
                    a[j][i][n][m] = 0.0;
                    b[j][i][n][m] = 0.0;
                    c[j][i][n][m] = 0.0;
                    d[j][i][n][m] = 0.0;
                }
            }
        }
    }
    for i in 1..T_LAST + 1 {
        timers.clear(i);
    }
    /*
     * ---------------------------------------------------------------------
     * compute the steady-state residuals
     * ---------------------------------------------------------------------
     */
    rhs(
        &mut frct[..],
        &mut rsd[..],
        &mut rho_i[..],
        &mut qs[..],
        &mut flux[..],
        &u[..],
        timers,
    );
    /*
     * ---------------------------------------------------------------------
     * compute the L2 norms of newton iteration residuals
     * ---------------------------------------------------------------------
     */
    l2norm(&mut rsd[..], &mut rsdnm[..]);
    for i in 1..T_LAST + 1 {
        timers.clear(i);
    }
    timers.start(1);
    /*
     * ---------------------------------------------------------------------
     * the timestep loop
     * ---------------------------------------------------------------------
     */
    for istep in 1..niter + 1 {
        if istep % 20 == 0 || istep == ITMAX_DEFAULT || istep == 1 {
            if niter > 1 {
                println!(" Time step {}", istep);
            }
        }
        /*
         * ---------------------------------------------------------------------
         * perform SSOR iteration
         * ---------------------------------------------------------------------
         */
        if TIMERS {
            timers.start(T_RHS);
        }
        rsd.iter_mut().for_each(|rsd| {
            rsd.iter_mut().for_each(|rsd| {
                rsd.iter_mut().for_each(|rsd| {
                    rsd.iter_mut().for_each(|rsd| *rsd *= DT_DEFAULT);
                });
            });
        });
        if TIMERS {
            timers.stop(T_RHS);
        }
        for k in 1..NZ - 1 {
            /*
             * ---------------------------------------------------------------------
             * form the lower triangular part of the jacobian matrix
             * ---------------------------------------------------------------------
             */
            if TIMERS {
                timers.start(T_JACLD);
            }
            jacld(
                k,
                &mut d[..],
                &mut c[..],
                &mut b[..],
                &mut a[..],
                &u[..],
                &qs[..],
                &rho_i[..],
            );
            if TIMERS {
                timers.stop(T_JACLD);
            }
            /*
             * ---------------------------------------------------------------------
             * perform the lower triangular solution
             * ---------------------------------------------------------------------
             */
            if TIMERS {
                timers.start(T_BLTS);
            }
            blts(k, &mut rsd[..], &a[..], &b[..], &c[..], &d[..]);
            if TIMERS {
                timers.stop(T_BLTS);
            }
        }
        for k in (1..NZ - 1).rev() {
            /*
             * ---------------------------------------------------------------------
             * form the strictly upper triangular part of the jacobian matrix
             * ---------------------------------------------------------------------
             */
            if TIMERS {
                timers.start(T_JACU);
            }
            jacu(
                k,
                &mut d[..],
                &mut c[..],
                &mut b[..],
                &mut a[..],
                &u[..],
                &qs[..],
                &rho_i[..],
            );
            if TIMERS {
                timers.stop(T_JACU);
            }
            /*
             * ---------------------------------------------------------------------
             * perform the upper triangular solution
             * ---------------------------------------------------------------------
             */
            if TIMERS {
                timers.start(T_BUTS);
            }
            buts(k, &mut rsd[..], &mut tv[..], &d[..], &a[..], &b[..], &c[..]);
            if TIMERS {
                timers.stop(T_BUTS);
            }
        }
        /*
         * ---------------------------------------------------------------------
         * update the variables
         * ---------------------------------------------------------------------
         */
        if TIMERS {
            timers.start(T_ADD);
        }
        u.iter_mut().zip(rsd.iter()).for_each(|(u, rsd)| {
            u.iter_mut().zip(rsd.iter()).for_each(|(u, rsd)| {
                u.iter_mut().zip(rsd.iter()).for_each(|(u, rsd)| {
                    u.iter_mut()
                        .zip(rsd.iter())
                        .for_each(|(u, rsd)| *u += tmp * rsd);
                });
            });
        });
        if TIMERS {
            timers.stop(T_ADD);
        }
        /*
         * ---------------------------------------------------------------------
         * compute the max-norms of newton iteration corrections
         * ---------------------------------------------------------------------
         */
        if istep % INORM_DEFAULT == 0 {
            if TIMERS {
                timers.start(T_L2NORM);
            }
            l2norm(&mut rsd[..], &mut delunm[..]);
            if TIMERS {
                timers.stop(T_L2NORM);
            }
        }
        /*
         * ---------------------------------------------------------------------
         * compute the steady-state residuals
         * ---------------------------------------------------------------------
         */
        rhs(
            &mut frct[..],
            &mut rsd[..],
            &mut rho_i[..],
            &mut qs[..],
            &mut flux[..],
            &u[..],
            timers,
        );
        /*
         * ---------------------------------------------------------------------
         * compute the max-norms of newton iteration residuals
         * ---------------------------------------------------------------------
         */
        if istep % INORM_DEFAULT == 0 || istep == ITMAX_DEFAULT {
            if TIMERS {
                timers.start(T_L2NORM);
            }
            l2norm(&mut rsd[..], &mut rsdnm[..]);
            if TIMERS {
                timers.stop(T_L2NORM);
            }
        }
        /*
         * ---------------------------------------------------------------------
         * check the newton-iteration residuals against the tolerance levels
         * ---------------------------------------------------------------------
         */
        if rsdnm[0] < tolrsd[0]
            && rsdnm[1] < tolrsd[1]
            && rsdnm[2] < tolrsd[2]
            && rsdnm[3] < tolrsd[3]
            && rsdnm[4] < tolrsd[4]
        {
            println!(
                " \n convergence was achieved after {} pseudo-time steps",
                istep
            );
            break;
        }
    }
    timers.stop(1);
    *maxtime = timers.read(1).as_secs_f64();
}

fn exact(i: usize, j: usize, k: usize, u000ijk: &mut [f64], ce: &[[f64; 5]]) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let xi = i as f64 / (NX0 as f64 - 1.0);
    let eta = j as f64 / (NY0 as f64 - 1.0);
    let zeta = k as f64 / (NZ as f64 - 1.0);
    for m in 0..5 as usize {
        u000ijk[m] = ce[0][m]
            + (ce[1][m] + (ce[4][m] + (ce[7][m] + ce[10][m] * xi) * xi) * xi) * xi
            + (ce[2][m] + (ce[5][m] + (ce[8][m] + ce[11][m] * eta) * eta) * eta) * eta
            + (ce[3][m] + (ce[6][m] + (ce[9][m] + ce[12][m] * zeta) * zeta) * zeta) * zeta;
    }
}

/*
 * ---------------------------------------------------------------------
 * compute the lower triangular part of the jacobian matrix
 * ---------------------------------------------------------------------
 */
fn jacld(
    k: usize,
    d: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    c: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    b: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    a: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    u: &[[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    qs: &[[[f64; ISIZ1 + 1]; ISIZ2 + 1]],
    rho_i: &[[[f64; ISIZ1 + 1]; ISIZ2 + 1]],
) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let (r43, c1345, c34): (f64, f64, f64);
    let (mut tmp1, mut tmp2, mut tmp3): (f64, f64, f64);
    r43 = 4.0 / 3.0;
    c1345 = C1 * C3 * C4 * C5;
    c34 = C3 * C4;
    for j in JST..JEND {
        for i in IST..IEND {
            /*
             * ---------------------------------------------------------------------
             * form the block daigonal
             * ---------------------------------------------------------------------
             */
            tmp1 = rho_i[k][j][i];
            tmp2 = tmp1 * tmp1;
            tmp3 = tmp1 * tmp2;
            d[j][i][0][0] = 1.0 + DT_DEFAULT * 2.0 * (TX1 * DX1 + TY1 * DY1 + TZ1 * DZ1);
            d[j][i][1][0] = 0.0;
            d[j][i][2][0] = 0.0;
            d[j][i][3][0] = 0.0;
            d[j][i][4][0] = 0.0;
            d[j][i][0][1] =
                -DT_DEFAULT * 2.0 * (TX1 * r43 + TY1 + TZ1) * c34 * tmp2 * u[k][j][i][1];
            d[j][i][1][1] = 1.0
                + DT_DEFAULT * 2.0 * c34 * tmp1 * (TX1 * r43 + TY1 + TZ1)
                + DT_DEFAULT * 2.0 * (TX1 * DX2 + TY1 * DY2 + TZ1 * DZ2);
            d[j][i][2][1] = 0.0;
            d[j][i][3][1] = 0.0;
            d[j][i][4][1] = 0.0;
            d[j][i][0][2] =
                -DT_DEFAULT * 2.0 * (TX1 + TY1 * r43 + TZ1) * c34 * tmp2 * u[k][j][i][2];
            d[j][i][1][2] = 0.0;
            d[j][i][2][2] = 1.0
                + DT_DEFAULT * 2.0 * c34 * tmp1 * (TX1 + TY1 * r43 + TZ1)
                + DT_DEFAULT * 2.0 * (TX1 * DX3 + TY1 * DY3 + TZ1 * DZ3);
            d[j][i][3][2] = 0.0;
            d[j][i][4][2] = 0.0;
            d[j][i][0][3] =
                -DT_DEFAULT * 2.0 * (TX1 + TY1 + TZ1 * r43) * c34 * tmp2 * u[k][j][i][3];
            d[j][i][1][3] = 0.0;
            d[j][i][2][3] = 0.0;
            d[j][i][3][3] = 1.0
                + DT_DEFAULT * 2.0 * c34 * tmp1 * (TX1 + TY1 + TZ1 * r43)
                + DT_DEFAULT * 2.0 * (TX1 * DX4 + TY1 * DY4 + TZ1 * DZ4);
            d[j][i][4][3] = 0.0;
            d[j][i][0][4] = -DT_DEFAULT
                * 2.0
                * (((TX1 * (r43 * c34 - c1345) + TY1 * (c34 - c1345) + TZ1 * (c34 - c1345))
                    * (u[k][j][i][1] * u[k][j][i][1])
                    + (TX1 * (c34 - c1345) + TY1 * (r43 * c34 - c1345) + TZ1 * (c34 - c1345))
                        * (u[k][j][i][2] * u[k][j][i][2])
                    + (TX1 * (c34 - c1345) + TY1 * (c34 - c1345) + TZ1 * (r43 * c34 - c1345))
                        * (u[k][j][i][3] * u[k][j][i][3]))
                    * tmp3
                    + (TX1 + TY1 + TZ1) * c1345 * tmp2 * u[k][j][i][4]);
            d[j][i][1][4] = DT_DEFAULT
                * 2.0
                * tmp2
                * u[k][j][i][1]
                * (TX1 * (r43 * c34 - c1345) + TY1 * (c34 - c1345) + TZ1 * (c34 - c1345));
            d[j][i][2][4] = DT_DEFAULT
                * 2.0
                * tmp2
                * u[k][j][i][2]
                * (TX1 * (c34 - c1345) + TY1 * (r43 * c34 - c1345) + TZ1 * (c34 - c1345));
            d[j][i][3][4] = DT_DEFAULT
                * 2.0
                * tmp2
                * u[k][j][i][3]
                * (TX1 * (c34 - c1345) + TY1 * (c34 - c1345) + TZ1 * (r43 * c34 - c1345));
            d[j][i][4][4] = 1.0
                + DT_DEFAULT * 2.0 * (TX1 + TY1 + TZ1) * c1345 * tmp1
                + DT_DEFAULT * 2.0 * (TX1 * DX5 + TY1 * DY5 + TZ1 * DZ5);
            /*
             * ---------------------------------------------------------------------
             * form the first block sub-diagonal
             * ---------------------------------------------------------------------
             */
            tmp1 = rho_i[k - 1][j][i];
            tmp2 = tmp1 * tmp1;
            tmp3 = tmp1 * tmp2;
            a[j][i][0][0] = -DT_DEFAULT * TZ1 * DZ1;
            a[j][i][1][0] = 0.0;
            a[j][i][2][0] = 0.0;
            a[j][i][3][0] = -DT_DEFAULT * TZ2;
            a[j][i][4][0] = 0.0;
            a[j][i][0][1] = -DT_DEFAULT * TZ2 * (-(u[k - 1][j][i][1] * u[k - 1][j][i][3]) * tmp2)
                - DT_DEFAULT * TZ1 * (-c34 * tmp2 * u[k - 1][j][i][1]);
            a[j][i][1][1] = -DT_DEFAULT * TZ2 * (u[k - 1][j][i][3] * tmp1)
                - DT_DEFAULT * TZ1 * c34 * tmp1
                - DT_DEFAULT * TZ1 * DZ2;
            a[j][i][2][1] = 0.0;
            a[j][i][3][1] = -DT_DEFAULT * TZ2 * (u[k - 1][j][i][1] * tmp1);
            a[j][i][4][1] = 0.0;
            a[j][i][0][2] = -DT_DEFAULT * TZ2 * (-(u[k - 1][j][i][2] * u[k - 1][j][i][3]) * tmp2)
                - DT_DEFAULT * TZ1 * (-c34 * tmp2 * u[k - 1][j][i][2]);
            a[j][i][1][2] = 0.0;
            a[j][i][2][2] = -DT_DEFAULT * TZ2 * (u[k - 1][j][i][3] * tmp1)
                - DT_DEFAULT * TZ1 * (c34 * tmp1)
                - DT_DEFAULT * TZ1 * DZ3;
            a[j][i][3][2] = -DT_DEFAULT * TZ2 * (u[k - 1][j][i][2] * tmp1);
            a[j][i][4][2] = 0.0;
            a[j][i][0][3] = -DT_DEFAULT
                * TZ2
                * (-(u[k - 1][j][i][3] * tmp1) * (u[k - 1][j][i][3] * tmp1)
                    + C2 * qs[k - 1][j][i] * tmp1)
                - DT_DEFAULT * TZ1 * (-r43 * c34 * tmp2 * u[k - 1][j][i][3]);
            a[j][i][1][3] = -DT_DEFAULT * TZ2 * (-C2 * (u[k - 1][j][i][1] * tmp1));
            a[j][i][2][3] = -DT_DEFAULT * TZ2 * (-C2 * (u[k - 1][j][i][2] * tmp1));
            a[j][i][3][3] = -DT_DEFAULT * TZ2 * (2.0 - C2) * (u[k - 1][j][i][3] * tmp1)
                - DT_DEFAULT * TZ1 * (r43 * c34 * tmp1)
                - DT_DEFAULT * TZ1 * DZ4;
            a[j][i][4][3] = -DT_DEFAULT * TZ2 * C2;
            a[j][i][0][4] = -DT_DEFAULT
                * TZ2
                * ((C2 * 2.0 * qs[k - 1][j][i] - C1 * u[k - 1][j][i][4])
                    * u[k - 1][j][i][3]
                    * tmp2)
                - DT_DEFAULT
                    * TZ1
                    * (-(c34 - c1345) * tmp3 * (u[k - 1][j][i][1] * u[k - 1][j][i][1])
                        - (c34 - c1345) * tmp3 * (u[k - 1][j][i][2] * u[k - 1][j][i][2])
                        - (r43 * c34 - c1345) * tmp3 * (u[k - 1][j][i][3] * u[k - 1][j][i][3])
                        - c1345 * tmp2 * u[k - 1][j][i][4]);
            a[j][i][1][4] =
                -DT_DEFAULT * TZ2 * (-C2 * (u[k - 1][j][i][1] * u[k - 1][j][i][3]) * tmp2)
                    - DT_DEFAULT * TZ1 * (c34 - c1345) * tmp2 * u[k - 1][j][i][1];
            a[j][i][2][4] =
                -DT_DEFAULT * TZ2 * (-C2 * (u[k - 1][j][i][2] * u[k - 1][j][i][3]) * tmp2)
                    - DT_DEFAULT * TZ1 * (c34 - c1345) * tmp2 * u[k - 1][j][i][2];
            a[j][i][3][4] = -DT_DEFAULT
                * TZ2
                * (C1 * (u[k - 1][j][i][4] * tmp1)
                    - C2 * (qs[k - 1][j][i] * tmp1 + u[k - 1][j][i][3] * u[k - 1][j][i][3] * tmp2))
                - DT_DEFAULT * TZ1 * (r43 * c34 - c1345) * tmp2 * u[k - 1][j][i][3];
            a[j][i][4][4] = -DT_DEFAULT * TZ2 * (C1 * (u[k - 1][j][i][3] * tmp1))
                - DT_DEFAULT * TZ1 * c1345 * tmp1
                - DT_DEFAULT * TZ1 * DZ5;
            /*
             * ---------------------------------------------------------------------
             * form the second block sub-diagonal
             * ---------------------------------------------------------------------
             */
            tmp1 = rho_i[k][j - 1][i];
            tmp2 = tmp1 * tmp1;
            tmp3 = tmp1 * tmp2;
            b[j][i][0][0] = -DT_DEFAULT * TY1 * DY1;
            b[j][i][1][0] = 0.0;
            b[j][i][2][0] = -DT_DEFAULT * TY2;
            b[j][i][3][0] = 0.0;
            b[j][i][4][0] = 0.0;
            b[j][i][0][1] = -DT_DEFAULT * TY2 * (-(u[k][j - 1][i][1] * u[k][j - 1][i][2]) * tmp2)
                - DT_DEFAULT * TY1 * (-c34 * tmp2 * u[k][j - 1][i][1]);
            b[j][i][1][1] = -DT_DEFAULT * TY2 * (u[k][j - 1][i][2] * tmp1)
                - DT_DEFAULT * TY1 * (c34 * tmp1)
                - DT_DEFAULT * TY1 * DY2;
            b[j][i][2][1] = -DT_DEFAULT * TY2 * (u[k][j - 1][i][1] * tmp1);
            b[j][i][3][1] = 0.0;
            b[j][i][4][1] = 0.0;
            b[j][i][0][2] = -DT_DEFAULT
                * TY2
                * (-(u[k][j - 1][i][2] * tmp1) * (u[k][j - 1][i][2] * tmp1)
                    + C2 * (qs[k][j - 1][i] * tmp1))
                - DT_DEFAULT * TY1 * (-r43 * c34 * tmp2 * u[k][j - 1][i][2]);
            b[j][i][1][2] = -DT_DEFAULT * TY2 * (-C2 * (u[k][j - 1][i][1] * tmp1));
            b[j][i][2][2] = -DT_DEFAULT * TY2 * ((2.0 - C2) * (u[k][j - 1][i][2] * tmp1))
                - DT_DEFAULT * TY1 * (r43 * c34 * tmp1)
                - DT_DEFAULT * TY1 * DY3;
            b[j][i][3][2] = -DT_DEFAULT * TY2 * (-C2 * (u[k][j - 1][i][3] * tmp1));
            b[j][i][4][2] = -DT_DEFAULT * TY2 * C2;
            b[j][i][0][3] = -DT_DEFAULT * TY2 * (-(u[k][j - 1][i][2] * u[k][j - 1][i][3]) * tmp2)
                - DT_DEFAULT * TY1 * (-c34 * tmp2 * u[k][j - 1][i][3]);
            b[j][i][1][3] = 0.0;
            b[j][i][2][3] = -DT_DEFAULT * TY2 * (u[k][j - 1][i][3] * tmp1);
            b[j][i][3][3] = -DT_DEFAULT * TY2 * (u[k][j - 1][i][2] * tmp1)
                - DT_DEFAULT * TY1 * (c34 * tmp1)
                - DT_DEFAULT * TY1 * DY4;
            b[j][i][4][3] = 0.0;
            b[j][i][0][4] = -DT_DEFAULT
                * TY2
                * ((C2 * 2.0 * qs[k][j - 1][i] - C1 * u[k][j - 1][i][4])
                    * (u[k][j - 1][i][2] * tmp2))
                - DT_DEFAULT
                    * TY1
                    * (-(c34 - c1345) * tmp3 * (u[k][j - 1][i][1] * u[k][j - 1][i][1])
                        - (r43 * c34 - c1345) * tmp3 * (u[k][j - 1][i][2] * u[k][j - 1][i][2])
                        - (c34 - c1345) * tmp3 * (u[k][j - 1][i][3] * u[k][j - 1][i][3])
                        - c1345 * tmp2 * u[k][j - 1][i][4]);
            b[j][i][1][4] =
                -DT_DEFAULT * TY2 * (-C2 * (u[k][j - 1][i][1] * u[k][j - 1][i][2]) * tmp2)
                    - DT_DEFAULT * TY1 * (c34 - c1345) * tmp2 * u[k][j - 1][i][1];
            b[j][i][2][4] = -DT_DEFAULT
                * TY2
                * (C1 * (u[k][j - 1][i][4] * tmp1)
                    - C2 * (qs[k][j - 1][i] * tmp1 + u[k][j - 1][i][2] * u[k][j - 1][i][2] * tmp2))
                - DT_DEFAULT * TY1 * (r43 * c34 - c1345) * tmp2 * u[k][j - 1][i][2];
            b[j][i][3][4] =
                -DT_DEFAULT * TY2 * (-C2 * (u[k][j - 1][i][2] * u[k][j - 1][i][3]) * tmp2)
                    - DT_DEFAULT * TY1 * (c34 - c1345) * tmp2 * u[k][j - 1][i][3];
            b[j][i][4][4] = -DT_DEFAULT * TY2 * (C1 * (u[k][j - 1][i][2] * tmp1))
                - DT_DEFAULT * TY1 * c1345 * tmp1
                - DT_DEFAULT * TY1 * DY5;
            /*
             * ---------------------------------------------------------------------
             * form the third block sub-diagonal
             * ---------------------------------------------------------------------
             */
            tmp1 = rho_i[k][j][i - 1];
            tmp2 = tmp1 * tmp1;
            tmp3 = tmp1 * tmp2;
            c[j][i][0][0] = -DT_DEFAULT * TX1 * DX1;
            c[j][i][1][0] = -DT_DEFAULT * TX2;
            c[j][i][2][0] = 0.0;
            c[j][i][3][0] = 0.0;
            c[j][i][4][0] = 0.0;
            c[j][i][0][1] = -DT_DEFAULT
                * TX2
                * (-(u[k][j][i - 1][1] * tmp1) * (u[k][j][i - 1][1] * tmp1)
                    + C2 * qs[k][j][i - 1] * tmp1)
                - DT_DEFAULT * TX1 * (-r43 * c34 * tmp2 * u[k][j][i - 1][1]);
            c[j][i][1][1] = -DT_DEFAULT * TX2 * ((2.0 - C2) * (u[k][j][i - 1][1] * tmp1))
                - DT_DEFAULT * TX1 * (r43 * c34 * tmp1)
                - DT_DEFAULT * TX1 * DX2;
            c[j][i][2][1] = -DT_DEFAULT * TX2 * (-C2 * (u[k][j][i - 1][2] * tmp1));
            c[j][i][3][1] = -DT_DEFAULT * TX2 * (-C2 * (u[k][j][i - 1][3] * tmp1));
            c[j][i][4][1] = -DT_DEFAULT * TX2 * C2;
            c[j][i][0][2] = -DT_DEFAULT * TX2 * (-(u[k][j][i - 1][1] * u[k][j][i - 1][2]) * tmp2)
                - DT_DEFAULT * TX1 * (-c34 * tmp2 * u[k][j][i - 1][2]);
            c[j][i][1][2] = -DT_DEFAULT * TX2 * (u[k][j][i - 1][2] * tmp1);
            c[j][i][2][2] = -DT_DEFAULT * TX2 * (u[k][j][i - 1][1] * tmp1)
                - DT_DEFAULT * TX1 * (c34 * tmp1)
                - DT_DEFAULT * TX1 * DX3;
            c[j][i][3][2] = 0.0;
            c[j][i][4][2] = 0.0;
            c[j][i][0][3] = -DT_DEFAULT * TX2 * (-(u[k][j][i - 1][1] * u[k][j][i - 1][3]) * tmp2)
                - DT_DEFAULT * TX1 * (-c34 * tmp2 * u[k][j][i - 1][3]);
            c[j][i][1][3] = -DT_DEFAULT * TX2 * (u[k][j][i - 1][3] * tmp1);
            c[j][i][2][3] = 0.0;
            c[j][i][3][3] = -DT_DEFAULT * TX2 * (u[k][j][i - 1][1] * tmp1)
                - DT_DEFAULT * TX1 * (c34 * tmp1)
                - DT_DEFAULT * TX1 * DX4;
            c[j][i][4][3] = 0.0;
            c[j][i][0][4] = -DT_DEFAULT
                * TX2
                * ((C2 * 2.0 * qs[k][j][i - 1] - C1 * u[k][j][i - 1][4])
                    * u[k][j][i - 1][1]
                    * tmp2)
                - DT_DEFAULT
                    * TX1
                    * (-(r43 * c34 - c1345) * tmp3 * (u[k][j][i - 1][1] * u[k][j][i - 1][1])
                        - (c34 - c1345) * tmp3 * (u[k][j][i - 1][2] * u[k][j][i - 1][2])
                        - (c34 - c1345) * tmp3 * (u[k][j][i - 1][3] * u[k][j][i - 1][3])
                        - c1345 * tmp2 * u[k][j][i - 1][4]);
            c[j][i][1][4] = -DT_DEFAULT
                * TX2
                * (C1 * (u[k][j][i - 1][4] * tmp1)
                    - C2 * (u[k][j][i - 1][1] * u[k][j][i - 1][1] * tmp2 + qs[k][j][i - 1] * tmp1))
                - DT_DEFAULT * TX1 * (r43 * c34 - c1345) * tmp2 * u[k][j][i - 1][1];
            c[j][i][2][4] =
                -DT_DEFAULT * TX2 * (-C2 * (u[k][j][i - 1][2] * u[k][j][i - 1][1]) * tmp2)
                    - DT_DEFAULT * TX1 * (c34 - c1345) * tmp2 * u[k][j][i - 1][2];
            c[j][i][3][4] =
                -DT_DEFAULT * TX2 * (-C2 * (u[k][j][i - 1][3] * u[k][j][i - 1][1]) * tmp2)
                    - DT_DEFAULT * TX1 * (c34 - c1345) * tmp2 * u[k][j][i - 1][3];
            c[j][i][4][4] = -DT_DEFAULT * TX2 * (C1 * (u[k][j][i - 1][1] * tmp1))
                - DT_DEFAULT * TX1 * c1345 * tmp1
                - DT_DEFAULT * TX1 * DX5;
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * compute the upper triangular part of the jacobian matrix
 * ---------------------------------------------------------------------
 */
fn jacu(
    k: usize,
    d: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    c: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    b: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    a: &mut [[[[f64; 5]; 5]; ISIZ1 + 1]],
    u: &[[[[f64; 5]; ISIZ1 + 1]; ISIZ2 + 1]],
    qs: &[[[f64; ISIZ1 + 1]; ISIZ2 + 1]],
    rho_i: &[[[f64; ISIZ1 + 1]; ISIZ2 + 1]],
) {
    /*
     * ---------------------------------------------------------------------
     * local variables
     * ---------------------------------------------------------------------
     */
    let (r43, c1345, c34): (f64, f64, f64);
    let (mut tmp1, mut tmp2, mut tmp3): (f64, f64, f64);
    r43 = 4.0 / 3.0;
    c1345 = C1 * C3 * C4 * C5;
    c34 = C3 * C4;
    for j in JST..JEND {
        for i in IST..IEND {
            /*
             * ---------------------------------------------------------------------
             * form the block daigonal
             * ---------------------------------------------------------------------
             */
            tmp1 = rho_i[k][j][i];
            tmp2 = tmp1 * tmp1;
            tmp3 = tmp1 * tmp2;
            d[j][i][0][0] = 1.0 + DT_DEFAULT * 2.0 * (TX1 * DX1 + TY1 * DY1 + TZ1 * DZ1);
            d[j][i][1][0] = 0.0;
            d[j][i][2][0] = 0.0;
            d[j][i][3][0] = 0.0;
            d[j][i][4][0] = 0.0;
            d[j][i][0][1] =
                DT_DEFAULT * 2.0 * (-TX1 * r43 - TY1 - TZ1) * (c34 * tmp2 * u[k][j][i][1]);
            d[j][i][1][1] = 1.0
                + DT_DEFAULT * 2.0 * c34 * tmp1 * (TX1 * r43 + TY1 + TZ1)
                + DT_DEFAULT * 2.0 * (TX1 * DX2 + TY1 * DY2 + TZ1 * DZ2);
            d[j][i][2][1] = 0.0;
            d[j][i][3][1] = 0.0;
            d[j][i][4][1] = 0.0;
            d[j][i][0][2] =
                DT_DEFAULT * 2.0 * (-TX1 - TY1 * r43 - TZ1) * (c34 * tmp2 * u[k][j][i][2]);
            d[j][i][1][2] = 0.0;
            d[j][i][2][2] = 1.0
                + DT_DEFAULT * 2.0 * c34 * tmp1 * (TX1 + TY1 * r43 + TZ1)
                + DT_DEFAULT * 2.0 * (TX1 * DX3 + TY1 * DY3 + TZ1 * DZ3);
            d[j][i][3][2] = 0.0;
            d[j][i][4][2] = 0.0;
            d[j][i][0][3] =
                DT_DEFAULT * 2.0 * (-TX1 - TY1 - TZ1 * r43) * (c34 * tmp2 * u[k][j][i][3]);
            d[j][i][1][3] = 0.0;
            d[j][i][2][3] = 0.0;
            d[j][i][3][3] = 1.0
                + DT_DEFAULT * 2.0 * c34 * tmp1 * (TX1 + TY1 + TZ1 * r43)
                + DT_DEFAULT * 2.0 * (TX1 * DX4 + TY1 * DY4 + TZ1 * DZ4);
            d[j][i][4][3] = 0.0;
            d[j][i][0][4] = -DT_DEFAULT
                * 2.0
                * (((TX1 * (r43 * c34 - c1345) + TY1 * (c34 - c1345) + TZ1 * (c34 - c1345))
                    * (u[k][j][i][1] * u[k][j][i][1])
                    + (TX1 * (c34 - c1345) + TY1 * (r43 * c34 - c1345) + TZ1 * (c34 - c1345))
                        * (u[k][j][i][2] * u[k][j][i][2])
                    + (TX1 * (c34 - c1345) + TY1 * (c34 - c1345) + TZ1 * (r43 * c34 - c1345))
                        * (u[k][j][i][3] * u[k][j][i][3]))
                    * tmp3
                    + (TX1 + TY1 + TZ1) * c1345 * tmp2 * u[k][j][i][4]);
            d[j][i][1][4] = DT_DEFAULT
                * 2.0
                * (TX1 * (r43 * c34 - c1345) + TY1 * (c34 - c1345) + TZ1 * (c34 - c1345))
                * tmp2
                * u[k][j][i][1];
            d[j][i][2][4] = DT_DEFAULT
                * 2.0
                * (TX1 * (c34 - c1345) + TY1 * (r43 * c34 - c1345) + TZ1 * (c34 - c1345))
                * tmp2
                * u[k][j][i][2];
            d[j][i][3][4] = DT_DEFAULT
                * 2.0
                * (TX1 * (c34 - c1345) + TY1 * (c34 - c1345) + TZ1 * (r43 * c34 - c1345))
                * tmp2
                * u[k][j][i][3];
            d[j][i][4][4] = 1.0
                + DT_DEFAULT * 2.0 * (TX1 + TY1 + TZ1) * c1345 * tmp1
                + DT_DEFAULT * 2.0 * (TX1 * DX5 + TY1 * DY5 + TZ1 * DZ5);
            /*
             * ---------------------------------------------------------------------
             * form the first block sub-diagonal
             * ---------------------------------------------------------------------
             */
            tmp1 = rho_i[k][j][i + 1];
            tmp2 = tmp1 * tmp1;
            tmp3 = tmp1 * tmp2;
            a[j][i][0][0] = -DT_DEFAULT * TX1 * DX1;
            a[j][i][1][0] = DT_DEFAULT * TX2;
            a[j][i][2][0] = 0.0;
            a[j][i][3][0] = 0.0;
            a[j][i][4][0] = 0.0;
            a[j][i][0][1] = DT_DEFAULT
                * TX2
                * (-(u[k][j][i + 1][1] * tmp1) * (u[k][j][i + 1][1] * tmp1)
                    + C2 * qs[k][j][i + 1] * tmp1)
                - DT_DEFAULT * TX1 * (-r43 * c34 * tmp2 * u[k][j][i + 1][1]);
            a[j][i][1][1] = DT_DEFAULT * TX2 * ((2.0 - C2) * (u[k][j][i + 1][1] * tmp1))
                - DT_DEFAULT * TX1 * (r43 * c34 * tmp1)
                - DT_DEFAULT * TX1 * DX2;
            a[j][i][2][1] = DT_DEFAULT * TX2 * (-C2 * (u[k][j][i + 1][2] * tmp1));
            a[j][i][3][1] = DT_DEFAULT * TX2 * (-C2 * (u[k][j][i + 1][3] * tmp1));
            a[j][i][4][1] = DT_DEFAULT * TX2 * C2;
            a[j][i][0][2] = DT_DEFAULT * TX2 * (-(u[k][j][i + 1][1] * u[k][j][i + 1][2]) * tmp2)
                - DT_DEFAULT * TX1 * (-c34 * tmp2 * u[k][j][i + 1][2]);
            a[j][i][1][2] = DT_DEFAULT * TX2 * (u[k][j][i + 1][2] * tmp1);
            a[j][i][2][2] = DT_DEFAULT * TX2 * (u[k][j][i + 1][1] * tmp1)
                - DT_DEFAULT * TX1 * (c34 * tmp1)
                - DT_DEFAULT * TX1 * DX3;
            a[j][i][3][2] = 0.0;
            a[j][i][4][2] = 0.0;
            a[j][i][0][3] = DT_DEFAULT * TX2 * (-(u[k][j][i + 1][1] * u[k][j][i + 1][3]) * tmp2)
                - DT_DEFAULT * TX1 * (-c34 * tmp2 * u[k][j][i + 1][3]);
            a[j][i][1][3] = DT_DEFAULT * TX2 * (u[k][j][i + 1][3] * tmp1);
            a[j][i][2][3] = 0.0;
            a[j][i][3][3] = DT_DEFAULT * TX2 * (u[k][j][i + 1][1] * tmp1)
                - DT_DEFAULT * TX1 * (c34 * tmp1)
                - DT_DEFAULT * TX1 * DX4;
            a[j][i][4][3] = 0.0;
            a[j][i][0][4] = DT_DEFAULT
                * TX2
                * ((C2 * 2.0 * qs[k][j][i + 1] - C1 * u[k][j][i + 1][4])
                    * (u[k][j][i + 1][1] * tmp2))
                - DT_DEFAULT
                    * TX1
                    * (-(r43 * c34 - c1345) * tmp3 * (u[k][j][i + 1][1] * u[k][j][i + 1][1])
                        - (c34 - c1345) * tmp3 * (u[k][j][i + 1][2] * u[k][j][i + 1][2])
                        - (c34 - c1345) * tmp3 * (u[k][j][i + 1][3] * u[k][j][i + 1][3])
                        - c1345 * tmp2 * u[k][j][i + 1][4]);
            a[j][i][1][4] = DT_DEFAULT
                * TX2
                * (C1 * (u[k][j][i + 1][4] * tmp1)
                    - C2 * (u[k][j][i + 1][1] * u[k][j][i + 1][1] * tmp2 + qs[k][j][i + 1] * tmp1))
                - DT_DEFAULT * TX1 * (r43 * c34 - c1345) * tmp2 * u[k][j][i + 1][1];
            a[j][i][2][4] =
                DT_DEFAULT * TX2 * (-C2 * (u[k][j][i + 1][2] * u[k][j][i + 1][1]) * tmp2)
                    - DT_DEFAULT * TX1 * (c34 - c1345) * tmp2 * u[k][j][i + 1][2];
            a[j][i][3][4] =
                DT_DEFAULT * TX2 * (-C2 * (u[k][j][i + 1][3] * u[k][j][i + 1][1]) * tmp2)
                    - DT_DEFAULT * TX1 * (c34 - c1345) * tmp2 * u[k][j][i + 1][3];
            a[j][i][4][4] = DT_DEFAULT * TX2 * (C1 * (u[k][j][i + 1][1] * tmp1))
                - DT_DEFAULT * TX1 * c1345 * tmp1
                - DT_DEFAULT * TX1 * DX5;
            /*
             * ---------------------------------------------------------------------
             * form the second block sub-diagonal
             * ---------------------------------------------------------------------
             */
            tmp1 = rho_i[k][j + 1][i];
            tmp2 = tmp1 * tmp1;
            tmp3 = tmp1 * tmp2;
            b[j][i][0][0] = -DT_DEFAULT * TY1 * DY1;
            b[j][i][1][0] = 0.0;
            b[j][i][2][0] = DT_DEFAULT * TY2;
            b[j][i][3][0] = 0.0;
            b[j][i][4][0] = 0.0;
            b[j][i][0][1] = DT_DEFAULT * TY2 * (-(u[k][j + 1][i][1] * u[k][j + 1][i][2]) * tmp2)
                - DT_DEFAULT * TY1 * (-c34 * tmp2 * u[k][j + 1][i][1]);
            b[j][i][1][1] = DT_DEFAULT * TY2 * (u[k][j + 1][i][2] * tmp1)
                - DT_DEFAULT * TY1 * (c34 * tmp1)
                - DT_DEFAULT * TY1 * DY2;
            b[j][i][2][1] = DT_DEFAULT * TY2 * (u[k][j + 1][i][1] * tmp1);
            b[j][i][3][1] = 0.0;
            b[j][i][4][1] = 0.0;
            b[j][i][0][2] = DT_DEFAULT
                * TY2
                * (-(u[k][j + 1][i][2] * tmp1) * (u[k][j + 1][i][2] * tmp1)
                    + C2 * (qs[k][j + 1][i] * tmp1))
                - DT_DEFAULT * TY1 * (-r43 * c34 * tmp2 * u[k][j + 1][i][2]);
            b[j][i][1][2] = DT_DEFAULT * TY2 * (-C2 * (u[k][j + 1][i][1] * tmp1));
            b[j][i][2][2] = DT_DEFAULT * TY2 * ((2.0 - C2) * (u[k][j + 1][i][2] * tmp1))
                - DT_DEFAULT * TY1 * (r43 * c34 * tmp1)
                - DT_DEFAULT * TY1 * DY3;
            b[j][i][3][2] = DT_DEFAULT * TY2 * (-C2 * (u[k][j + 1][i][3] * tmp1));
            b[j][i][4][2] = DT_DEFAULT * TY2 * C2;
            b[j][i][0][3] = DT_DEFAULT * TY2 * (-(u[k][j + 1][i][2] * u[k][j + 1][i][3]) * tmp2)
                - DT_DEFAULT * TY1 * (-c34 * tmp2 * u[k][j + 1][i][3]);
            b[j][i][1][3] = 0.0;
            b[j][i][2][3] = DT_DEFAULT * TY2 * (u[k][j + 1][i][3] * tmp1);
            b[j][i][3][3] = DT_DEFAULT * TY2 * (u[k][j + 1][i][2] * tmp1)
                - DT_DEFAULT * TY1 * (c34 * tmp1)
                - DT_DEFAULT * TY1 * DY4;
            b[j][i][4][3] = 0.0;
            b[j][i][0][4] = DT_DEFAULT
                * TY2
                * ((C2 * 2.0 * qs[k][j + 1][i] - C1 * u[k][j + 1][i][4])
                    * (u[k][j + 1][i][2] * tmp2))
                - DT_DEFAULT
                    * TY1
                    * (-(c34 - c1345) * tmp3 * (u[k][j + 1][i][1] * u[k][j + 1][i][1])
                        - (r43 * c34 - c1345) * tmp3 * (u[k][j + 1][i][2] * u[k][j + 1][i][2])
                        - (c34 - c1345) * tmp3 * (u[k][j + 1][i][3] * u[k][j + 1][i][3])
                        - c1345 * tmp2 * u[k][j + 1][i][4]);
            b[j][i][1][4] =
                DT_DEFAULT * TY2 * (-C2 * (u[k][j + 1][i][1] * u[k][j + 1][i][2]) * tmp2)
                    - DT_DEFAULT * TY1 * (c34 - c1345) * tmp2 * u[k][j + 1][i][1];
            b[j][i][2][4] = DT_DEFAULT
                * TY2
                * (C1 * (u[k][j + 1][i][4] * tmp1)
                    - C2 * (qs[k][j + 1][i] * tmp1 + u[k][j + 1][i][2] * u[k][j + 1][i][2] * tmp2))
                - DT_DEFAULT * TY1 * (r43 * c34 - c1345) * tmp2 * u[k][j + 1][i][2];
            b[j][i][3][4] =
                DT_DEFAULT * TY2 * (-C2 * (u[k][j + 1][i][2] * u[k][j + 1][i][3]) * tmp2)
                    - DT_DEFAULT * TY1 * (c34 - c1345) * tmp2 * u[k][j + 1][i][3];
            b[j][i][4][4] = DT_DEFAULT * TY2 * (C1 * (u[k][j + 1][i][2] * tmp1))
                - DT_DEFAULT * TY1 * c1345 * tmp1
                - DT_DEFAULT * TY1 * DY5;
            /*
             * ---------------------------------------------------------------------
             * form the third block sub-diagonal
             * ---------------------------------------------------------------------
             */
            tmp1 = rho_i[k + 1][j][i];
            tmp2 = tmp1 * tmp1;
            tmp3 = tmp1 * tmp2;
            c[j][i][0][0] = -DT_DEFAULT * TZ1 * DZ1;
            c[j][i][1][0] = 0.0;
            c[j][i][2][0] = 0.0;
            c[j][i][3][0] = DT_DEFAULT * TZ2;
            c[j][i][4][0] = 0.0;
            c[j][i][0][1] = DT_DEFAULT * TZ2 * (-(u[k + 1][j][i][1] * u[k + 1][j][i][3]) * tmp2)
                - DT_DEFAULT * TZ1 * (-c34 * tmp2 * u[k + 1][j][i][1]);
            c[j][i][1][1] = DT_DEFAULT * TZ2 * (u[k + 1][j][i][3] * tmp1)
                - DT_DEFAULT * TZ1 * c34 * tmp1
                - DT_DEFAULT * TZ1 * DZ2;
            c[j][i][2][1] = 0.0;
            c[j][i][3][1] = DT_DEFAULT * TZ2 * (u[k + 1][j][i][1] * tmp1);
            c[j][i][4][1] = 0.0;
            c[j][i][0][2] = DT_DEFAULT * TZ2 * (-(u[k + 1][j][i][2] * u[k + 1][j][i][3]) * tmp2)
                - DT_DEFAULT * TZ1 * (-c34 * tmp2 * u[k + 1][j][i][2]);
            c[j][i][1][2] = 0.0;
            c[j][i][2][2] = DT_DEFAULT * TZ2 * (u[k + 1][j][i][3] * tmp1)
                - DT_DEFAULT * TZ1 * (c34 * tmp1)
                - DT_DEFAULT * TZ1 * DZ3;
            c[j][i][3][2] = DT_DEFAULT * TZ2 * (u[k + 1][j][i][2] * tmp1);
            c[j][i][4][2] = 0.0;
            c[j][i][0][3] = DT_DEFAULT
                * TZ2
                * (-(u[k + 1][j][i][3] * tmp1) * (u[k + 1][j][i][3] * tmp1)
                    + C2 * (qs[k + 1][j][i] * tmp1))
                - DT_DEFAULT * TZ1 * (-r43 * c34 * tmp2 * u[k + 1][j][i][3]);
            c[j][i][1][3] = DT_DEFAULT * TZ2 * (-C2 * (u[k + 1][j][i][1] * tmp1));
            c[j][i][2][3] = DT_DEFAULT * TZ2 * (-C2 * (u[k + 1][j][i][2] * tmp1));
            c[j][i][3][3] = DT_DEFAULT * TZ2 * (2.0 - C2) * (u[k + 1][j][i][3] * tmp1)
                - DT_DEFAULT * TZ1 * (r43 * c34 * tmp1)
                - DT_DEFAULT * TZ1 * DZ4;
            c[j][i][4][3] = DT_DEFAULT * TZ2 * C2;
            c[j][i][0][4] = DT_DEFAULT
                * TZ2
                * ((C2 * 2.0 * qs[k + 1][j][i] - C1 * u[k + 1][j][i][4])
                    * (u[k + 1][j][i][3] * tmp2))
                - DT_DEFAULT
                    * TZ1
                    * (-(c34 - c1345) * tmp3 * (u[k + 1][j][i][1] * u[k + 1][j][i][1])
                        - (c34 - c1345) * tmp3 * (u[k + 1][j][i][2] * u[k + 1][j][i][2])
                        - (r43 * c34 - c1345) * tmp3 * (u[k + 1][j][i][3] * u[k + 1][j][i][3])
                        - c1345 * tmp2 * u[k + 1][j][i][4]);
            c[j][i][1][4] =
                DT_DEFAULT * TZ2 * (-C2 * (u[k + 1][j][i][1] * u[k + 1][j][i][3]) * tmp2)
                    - DT_DEFAULT * TZ1 * (c34 - c1345) * tmp2 * u[k + 1][j][i][1];
            c[j][i][2][4] =
                DT_DEFAULT * TZ2 * (-C2 * (u[k + 1][j][i][2] * u[k + 1][j][i][3]) * tmp2)
                    - DT_DEFAULT * TZ1 * (c34 - c1345) * tmp2 * u[k + 1][j][i][2];
            c[j][i][3][4] = DT_DEFAULT
                * TZ2
                * (C1 * (u[k + 1][j][i][4] * tmp1)
                    - C2 * (qs[k + 1][j][i] * tmp1 + u[k + 1][j][i][3] * u[k + 1][j][i][3] * tmp2))
                - DT_DEFAULT * TZ1 * (r43 * c34 - c1345) * tmp2 * u[k + 1][j][i][3];
            c[j][i][4][4] = DT_DEFAULT * TZ2 * (C1 * (u[k + 1][j][i][3] * tmp1))
                - DT_DEFAULT * TZ1 * c1345 * tmp1
                - DT_DEFAULT * TZ1 * DZ5;
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * verification routine
 * ---------------------------------------------------------------------
 */
fn verify(xcr: &[f64], xce: &[f64], xci: f64, verified: &mut i8) {
    let mut xcrref: [f64; 5] = [1.0; 5];
    let mut xceref: [f64; 5] = [1.0; 5];
    let mut xcrdif: [f64; 5] = [0.0; 5];
    let mut xcedif: [f64; 5] = [0.0; 5];
    let mut xciref: f64 = 1.0;
    let xcidif: f64;
    let mut dtref: f64 = 0.0;

    *verified = 1;
    if CLASS == 'S' {
        dtref = 5.0e-1;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual, for the (12X12X12) grid,
         * after 50 time steps, with DT = 5.0d-01
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 1.6196343210976702e-02;
        xcrref[1] = 2.1976745164821318e-03;
        xcrref[2] = 1.5179927653399185e-03;
        xcrref[3] = 1.5029584435994323e-03;
        xcrref[4] = 3.4264073155896461e-02;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error, for the (12X12X12) grid,
         * after 50 time steps, with DT = 5.0d-01
         * ---------------------------------------------------------------------
         */
        xceref[0] = 6.4223319957960924e-04;
        xceref[1] = 8.4144342047347926e-05;
        xceref[2] = 5.8588269616485186e-05;
        xceref[3] = 5.8474222595157350e-05;
        xceref[4] = 1.3103347914111294e-03;
        /*
         * ---------------------------------------------------------------------
         * reference value of surface integral, for the (12X12X12) grid,
         * after 50 time steps, with DT = 5.0d-01
         * ---------------------------------------------------------------------
         */
        xciref = 7.8418928865937083e+00;
    } else if CLASS == 'W' {
        dtref = 1.5e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual, for the (33x33x33) grid,
         * after 300 time steps, with DT = 1.5d-3
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.1236511638192e+02;
        xcrref[1] = 0.1317228477799e+01;
        xcrref[2] = 0.2550120713095e+01;
        xcrref[3] = 0.2326187750252e+01;
        xcrref[4] = 0.2826799444189e+02;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error, for the (33X33X33) grid,
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.4867877144216e+00;
        xceref[1] = 0.5064652880982e-01;
        xceref[2] = 0.9281818101960e-01;
        xceref[3] = 0.8570126542733e-01;
        xceref[4] = 0.1084277417792e+01;
        /*
         * ---------------------------------------------------------------------
         * rReference value of surface integral, for the (33X33X33) grid,
         * after 300 time steps, with DT = 1.5d-3
         * ---------------------------------------------------------------------
         */
        xciref = 0.1161399311023e+02;
    } else if CLASS == 'A' {
        dtref = 2.0e+0;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual, for the (64X64X64) grid,
         * after 250 time steps, with DT = 2.0d+00
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 7.7902107606689367e+02;
        xcrref[1] = 6.3402765259692870e+01;
        xcrref[2] = 1.9499249727292479e+02;
        xcrref[3] = 1.7845301160418537e+02;
        xcrref[4] = 1.8384760349464247e+03;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error, for the (64X64X64) grid,
         * after 250 time steps, with DT = 2.0d+00
         * ---------------------------------------------------------------------
         */
        xceref[0] = 2.9964085685471943e+01;
        xceref[1] = 2.8194576365003349e+00;
        xceref[2] = 7.3473412698774742e+00;
        xceref[3] = 6.7139225687777051e+00;
        xceref[4] = 7.0715315688392578e+01;
        /*
         * ---------------------------------------------------------------------
         * reference value of surface integral, for the (64X64X64) grid,
         * after 250 time steps, with DT = 2.0d+00
         * ---------------------------------------------------------------------
         */
        xciref = 2.6030925604886277e+01;
    } else if CLASS == 'B' {
        dtref = 2.0e+0;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual, for the (102X102X102) grid,
         * after 250 time steps, with DT = 2.0d+00
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 3.5532672969982736e+03;
        xcrref[1] = 2.6214750795310692e+02;
        xcrref[2] = 8.8333721850952190e+02;
        xcrref[3] = 7.7812774739425265e+02;
        xcrref[4] = 7.3087969592545314e+03;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error, for the (102X102X102)
         * grid, after 250 time steps, with DT = 2.0d+00
         * ---------------------------------------------------------------------
         */
        xceref[0] = 1.1401176380212709e+02;
        xceref[1] = 8.1098963655421574e+00;
        xceref[2] = 2.8480597317698308e+01;
        xceref[3] = 2.5905394567832939e+01;
        xceref[4] = 2.6054907504857413e+02;
        /*
          c---------------------------------------------------------------------
        * reference value of surface integral, for the (102X102X102) grid,
        * after 250 time steps, with DT = 2.0d+00
        * ---------------------------------------------------------------------
        */
        xciref = 4.7887162703308227e+01;
    } else if CLASS == 'C' {
        dtref = 2.0e+0;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual, for the (162X162X162) grid,
         * after 250 time steps, with DT = 2.0d+00
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 1.03766980323537846e+04;
        xcrref[1] = 8.92212458801008552e+02;
        xcrref[2] = 2.56238814582660871e+03;
        xcrref[3] = 2.19194343857831427e+03;
        xcrref[4] = 1.78078057261061185e+04;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error, for the (162X162X162)
         * grid, after 250 time steps, with DT = 2.0d+00
         * ---------------------------------------------------------------------
         */
        xceref[0] = 2.15986399716949279e+02;
        xceref[1] = 1.55789559239863600e+01;
        xceref[2] = 5.41318863077207766e+01;
        xceref[3] = 4.82262643154045421e+01;
        xceref[4] = 4.55902910043250358e+02;
        /*
         * ---------------------------------------------------------------------
         * reference value of surface integral, for the (162X162X162) grid,
         * after 250 time steps, with DT = 2.0d+00
         * ---------------------------------------------------------------------
         */
        xciref = 6.66404553572181300e+01;
    } else if CLASS == 'D' {
        dtref = 1.0e+0;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual, for the (408X408X408) grid,
         * after 300 time steps, with DT = 1.0d+00
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.4868417937025e+05;
        xcrref[1] = 0.4696371050071e+04;
        xcrref[2] = 0.1218114549776e+05;
        xcrref[3] = 0.1033801493461e+05;
        xcrref[4] = 0.7142398413817e+05;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error, for the (408X408X408)
         * grid, after 300 time steps, with DT = 1.0d+00
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.3752393004482e+03;
        xceref[1] = 0.3084128893659e+02;
        xceref[2] = 0.9434276905469e+02;
        xceref[3] = 0.8230686681928e+02;
        xceref[4] = 0.7002620636210e+03;
        /*
         * ---------------------------------------------------------------------
         * reference value of surface integral, for the (408X408X408) grid,
         * after 300 time steps, with DT = 1.0d+00
         * ---------------------------------------------------------------------
         */
        xciref = 0.8334101392503e+02;
    } else if CLASS == 'E' {
        dtref = 0.5e+0;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual, for the (1020X1020X1020) grid,
         * after 300 time steps, with DT = 0.5d+00
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.2099641687874e+06;
        xcrref[1] = 0.2130403143165e+05;
        xcrref[2] = 0.5319228789371e+05;
        xcrref[3] = 0.4509761639833e+05;
        xcrref[4] = 0.2932360006590e+06;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error, for the (1020X1020X1020)
         * grid, after 300 time steps, with DT = 0.5d+00
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.4800572578333e+03;
        xceref[1] = 0.4221993400184e+02;
        xceref[2] = 0.1210851906824e+03;
        xceref[3] = 0.1047888986770e+03;
        xceref[4] = 0.8363028257389e+03;
        /*
         * ---------------------------------------------------------------------
         * reference value of surface integral, for the (1020X1020X1020) grid,
         * after 300 time steps, with DT = 0.5d+00
         * ---------------------------------------------------------------------
         */
        xciref = 0.9512163272273e+02;
    } else {
        *verified = 0;
    }
    /*
     * ---------------------------------------------------------------------
     * verification test for residuals if gridsize is one of
     * the defined grid sizes above (class .ne. 'U')
     * ---------------------------------------------------------------------
     * compute the difference of solution values and the known reference values.
     * ---------------------------------------------------------------------
     */
    for m in 0..5 {
        xcrdif[m] = ((xcr[m] - xcrref[m]) / xcrref[m]).abs();
        xcedif[m] = ((xce[m] - xceref[m]) / xceref[m]).abs();
    }
    xcidif = ((xci - xciref) / xciref).abs();
    /*
     * ---------------------------------------------------------------------
     * output the comparison of computed results to known cases.
     * ---------------------------------------------------------------------
     */
    if CLASS != 'U' {
        println!("\n Verification being performed for class_npb {}", CLASS);
        println!(" accuracy setting for epsilon = {:>20.13e}", EPSILON);
        *verified = {
            let x;
            if (DT_DEFAULT - dtref).abs() <= EPSILON {
                x = 1
            } else {
                x = 0
            }
            x
        };
        if *verified == 0 {
            println!(" DT does not match the reference value of {:>15.8e}", dtref);
        }
        println!(" Comparison of RMS-norms of residual");
        for m in 0..5 {
            if xcedif[m] <= EPSILON {
                println!(
                    "          {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xcr[m],
                    xcrref[m],
                    xcrdif[m]
                );
            } else {
                *verified = 0;
                println!(
                    " FAILURE: {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xcr[m],
                    xcrref[m],
                    xcrdif[m]
                );
            }
        }
        println!(" Comparison of RMS-norms of solution error");
        for m in 0..5 {
            if xcedif[m] <= EPSILON {
                println!(
                    "          {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xce[m],
                    xceref[m],
                    xcedif[m]
                );
            } else {
                *verified = 0;
                println!(
                    " FAILURE: {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xce[m],
                    xceref[m],
                    xcedif[m]
                );
            }
        }
        println!(" Comparison of surface integral");
        for m in 0..5 {
            if xcidif <= EPSILON {
                println!(
                    "          {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xci,
                    xciref,
                    xcidif
                );
            } else {
                *verified = 0;
                println!(
                    " FAILURE: {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xci,
                    xciref,
                    xcidif
                );
            }
        }
        if *verified == 1 {
            println!(" Verification Successful");
        } else {
            println!(" Verification failed");
        }
    }
}
