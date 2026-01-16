use crate::data::*;

unsafe extern "C" {
    fn ltini_bridge();
    fn ltexi_bridge();
    fn clearcache_bridge();
    fn restorecache_bridge();
    fn setcmpbits_bridge(b: CIntType);
    fn getcmpbits_bridge(b: &mut CIntType);
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

pub fn setcmpbits(b: i32) {
    unsafe {
        setcmpbits_bridge(b);
    }
}

pub fn getcmpbits() -> i32 {
    let mut b = 0;
    unsafe {
        getcmpbits_bridge(&mut b);
    }
    b
}

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
