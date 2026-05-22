use crate::datatypes::*;

unsafe extern "C" {
    fn ltini_bridge();
    fn ltexi_bridge();

    fn clearcache_bridge();
    fn restorecache_bridge();
    fn setcmpbits_bridge(b: CIntType);
    fn getcmpbits_bridge(b: &mut CIntType);

    fn setversionkey_bridge(k: CIntType);
    fn getversionkey_bridge(k: &mut CIntType);
    fn setmaxdev_bridge(eps: CRealType);
    fn getmaxdev_bridge(eps: &mut CRealType);
    fn setdebugkey_bridge(k: CIntType);
    fn getdebugkey_bridge(k: &mut CIntType);
    fn setdebugrange_bridge(f: CIntType, t: CIntType);

    fn setwarndigits_bridge(d: CIntType);
    fn getwarndigits_bridge(d: &mut CIntType);
    fn seterrdigits_bridge(d: CIntType);
    fn geterrdigits_bridge(d: &mut CIntType);

    fn setdelta_bridge(del: CRealType);
    fn getdelta_bridge(del: &mut CRealType);
    fn setmudim_bridge(mu2: CRealType);
    fn getmudim_bridge(mu2: &mut CRealType);
    fn setlambda_bridge(lam2: CRealType);
    fn getlambda_bridge(lam2: &mut CRealType);
    fn setminmass_bridge(m2min: CRealType);
    fn getminmass_bridge(m2min: &mut CRealType);
    fn setuvdiv_bridge(x: CRealType);
    fn getuvdiv_bridge(x: &mut CRealType);

    fn setzeroeps_bridge(eps: CRealType);
    fn getzeroeps_bridge(eps: &mut CRealType);
    fn setdiffeps_bridge(eps: CRealType);
    fn getdiffeps_bridge(eps: &mut CRealType);

    fn aput_bridge(res: &mut ACoeffs, m: CRealType);
    fn bput_bridge(res: &mut BCoeffs, p: CRealType, m1: CRealType, m2: CRealType);
    fn cput_bridge(
        res: &mut CCoeffs,
        p1: CRealType,
        p2: CRealType,
        p1p2: CRealType,
        m1: CRealType,
        m2: CRealType,
        m3: CRealType,
    );
    fn dput_bridge(
        res: &mut DCoeffs,
        p1: CRealType,
        p2: CRealType,
        p3: CRealType,
        p4: CRealType,
        p1p2: CRealType,
        p2p3: CRealType,
        m1: CRealType,
        m2: CRealType,
        m3: CRealType,
        m4: CRealType,
    );
    fn eput_bridge(
        res: &mut ECoeffs,
        p1: CRealType,
        p2: CRealType,
        p3: CRealType,
        p4: CRealType,
        p5: CRealType,
        p1p2: CRealType,
        p2p3: CRealType,
        p3p4: CRealType,
        p4p5: CRealType,
        p5p1: CRealType,
        m1: CRealType,
        m2: CRealType,
        m3: CRealType,
        m4: CRealType,
        m5: CRealType,
    );
}

// ltini and ltexi

pub fn ltini() {
    unsafe {
        ltini_bridge();
    }
}

pub fn ltexi() {
    unsafe {
        ltexi_bridge();
    }
}

// Cache Mechanism

pub fn clearcache() {
    unsafe {
        clearcache_bridge();
    }
}

pub fn restorecache() {
    unsafe {
        restorecache_bridge();
    }
}

pub fn setcmpbits(b: CIntType) {
    unsafe {
        setcmpbits_bridge(b);
    }
}

pub fn getcmpbits() -> CIntType {
    let mut b = 0;
    unsafe {
        getcmpbits_bridge(&mut b);
    }
    b
}

// Versions and Debugging

pub fn setversionkey(k: CIntType) {
    unsafe {
        setversionkey_bridge(k);
    }
}

pub fn getversionkey() -> CIntType {
    let mut k = 0;
    unsafe {
        getversionkey_bridge(&mut k);
    }
    k
}

pub fn setmaxdev(eps: CRealType) {
    unsafe {
        setmaxdev_bridge(eps);
    }
}

pub fn getmaxdev() -> CRealType {
    let mut eps = 0.;
    unsafe {
        getmaxdev_bridge(&mut eps);
    }
    eps
}

pub fn setdebugkey(k: CIntType) {
    unsafe {
        setdebugkey_bridge(k);
    }
}

pub fn getdebugkey() -> CIntType {
    let mut k = 0;
    unsafe {
        getdebugkey_bridge(&mut k);
    }
    k
}

pub fn setdebugrange(f: CIntType, t: CIntType) {
    unsafe {
        setdebugrange_bridge(f, t);
    }
}

// Warning Messages and Checking Results

pub fn setwarndigits(d: CIntType) {
    unsafe {
        setwarndigits_bridge(d);
    }
}

pub fn getwarndigits() -> CIntType {
    let mut d = 0;
    unsafe {
        getwarndigits_bridge(&mut d);
    }
    d
}

pub fn seterrdigits(d: CIntType) {
    unsafe {
        seterrdigits_bridge(d);
    }
}

pub fn geterrdigits() -> CIntType {
    let mut d = 0;
    unsafe {
        geterrdigits_bridge(&mut d);
    }
    d
}

// Ultraviolet, Infrared, and Collinear Divergences

pub fn setdelta(del: CRealType) {
    unsafe {
        setdelta_bridge(del);
    }
}

pub fn getdelta() -> CRealType {
    let mut del = 0.;
    unsafe {
        getdelta_bridge(&mut del);
    }
    del
}

pub fn setmudim(mu2: CRealType) {
    unsafe {
        setmudim_bridge(mu2);
    }
}

pub fn getmudim() -> CRealType {
    let mut mu2 = 0.;
    unsafe {
        getmudim_bridge(&mut mu2);
    }
    mu2
}

pub fn setlambda(lam2: CRealType) {
    unsafe {
        setlambda_bridge(lam2);
    }
}

pub fn getlambda() -> CRealType {
    let mut lam2 = 0.;
    unsafe {
        getlambda_bridge(&mut lam2);
    }
    lam2
}

pub fn setminmass(m2min: CRealType) {
    unsafe {
        setminmass_bridge(m2min);
    }
}

pub fn getminmass() -> CRealType {
    let mut m2min = 0.;
    unsafe {
        getminmass_bridge(&mut m2min);
    }
    m2min
}

pub fn setuvdiv(x: CRealType) {
    unsafe {
        setuvdiv_bridge(x);
    }
}

pub fn getuvdiv() -> CRealType {
    let mut x = 0.;
    unsafe {
        getuvdiv_bridge(&mut x);
    }
    x
}

// Accuracy

pub fn setzeroeps(eps: CRealType) {
    unsafe {
        setzeroeps_bridge(eps);
    }
}

pub fn getzeroeps() -> CRealType {
    let mut eps = 0.;
    unsafe {
        getzeroeps_bridge(&mut eps);
    }
    eps
}

pub fn setdiffeps(eps: CRealType) {
    unsafe {
        setdiffeps_bridge(eps);
    }
}

pub fn getdiffeps() -> CRealType {
    let mut eps = 0.;
    unsafe {
        getdiffeps_bridge(&mut eps);
    }
    eps
}

// Loop Functions

pub fn aget(m: CRealType) -> ACoeffs {
    let mut res = ACoeffs::default();
    unsafe {
        aput_bridge(&mut res, m);
    }
    res
}

pub fn bget(p: CRealType, m1: CRealType, m2: CRealType) -> BCoeffs {
    let mut res = BCoeffs::default();
    unsafe {
        bput_bridge(&mut res, p, m1, m2);
    }
    res
}

pub fn cget(
    p1: CRealType,
    p2: CRealType,
    p1p2: CRealType,
    m1: CRealType,
    m2: CRealType,
    m3: CRealType,
) -> CCoeffs {
    let mut res = CCoeffs::default();
    unsafe {
        cput_bridge(&mut res, p1, p2, p1p2, m1, m2, m3);
    }
    res
}

pub fn dget(
    p1: CRealType,
    p2: CRealType,
    p3: CRealType,
    p4: CRealType,
    p1p2: CRealType,
    p2p3: CRealType,
    m1: CRealType,
    m2: CRealType,
    m3: CRealType,
    m4: CRealType,
) -> DCoeffs {
    let mut res = DCoeffs::default();
    unsafe {
        dput_bridge(&mut res, p1, p2, p3, p4, p1p2, p2p3, m1, m2, m3, m4);
    }
    res
}

pub fn eget(
    p1: CRealType,
    p2: CRealType,
    p3: CRealType,
    p4: CRealType,
    p5: CRealType,
    p1p2: CRealType,
    p2p3: CRealType,
    p3p4: CRealType,
    p4p5: CRealType,
    p5p1: CRealType,
    m1: CRealType,
    m2: CRealType,
    m3: CRealType,
    m4: CRealType,
    m5: CRealType,
) -> ECoeffs {
    let mut res = ECoeffs::default();
    unsafe {
        eput_bridge(
            &mut res, p1, p2, p3, p4, p5, p1p2, p2p3, p3p4, p4p5, p5p1, m1, m2, m3, m4, m5,
        );
    }
    res
}
