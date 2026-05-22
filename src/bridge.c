#include "clooptools.h"

// ltini and ltexi

void ltini_bridge()
{
    ltini();
}

void ltexi_bridge()
{
    ltexi();
}

// Cache Mechanism

void clearcache_bridge()
{
    clearcache();
}

void restorecache_bridge()
{
    restorecache();
}

void setcmpbits_bridge(int b)
{
    setcmpbits(b);
}

void getcmpbits_bridge(int *b)
{
    *b = getcmpbits();
}

// Versions and Debugging

void setversionkey_bridge(int k)
{
    setversionkey(k);
}

void getversionkey_bridge(int *k)
{
    *k = getversionkey();
}

void setmaxdev_bridge(cRealType eps)
{
    setmaxdev(eps);
}

void getmaxdev_bridge(RealType *eps)
{
    *eps = getmaxdev();
}

void setdebugkey_bridge(int k)
{
    setdebugkey(k);
}

void getdebugkey_bridge(int *k)
{
    *k = getdebugkey();
}

void setdebugrange_bridge(int f, int t)
{
    setdebugrange(f, t);
}

// Warning Messages and Checking Results

void setwarndigits_bridge(int d)
{
    setwarndigits(d);
}

void getwarndigits_bridge(int *d)
{
    *d = getwarndigits();
}

void seterrdigits_bridge(int d)
{
    seterrdigits(d);
}

void geterrdigits_bridge(int *d)
{
    *d = geterrdigits();
}

// Ultraviolet, Infrared, and Collinear Divergences

void setdelta_bridge(cRealType del)
{
    setdelta(del);
}

void getdelta_bridge(RealType *del)
{
    *del = getdelta();
}

void setmudim_bridge(cRealType mu2)
{
    setmudim(mu2);
}

void getmudim_bridge(RealType *mu2)
{
    *mu2 = getmudim();
}

void setlambda_bridge(cRealType lam2)
{
    setlambda(lam2);
}

void getlambda_bridge(RealType *lam2)
{
    *lam2 = getlambda();
}

void setminmass_bridge(cRealType m2min)
{
    setminmass(m2min);
}

void getminmass_bridge(RealType *m2min)
{
    *m2min = getminmass();
}

void setuvdiv_bridge(cRealType x)
{
    setuvdiv(x);
}

void getuvdiv_bridge(RealType *x)
{
    *x = getuvdiv();
}

// Accuracy

void setzeroeps_bridge(cRealType eps)
{
    setzeroeps(eps);
}

void getzeroeps_bridge(RealType *eps)
{
    *eps = getzeroeps();
}

void setdiffeps_bridge(cRealType eps)
{
    setdiffeps(eps);
}

void getdiffeps_bridge(RealType *eps)
{
    *eps = getdiffeps();
}

// Loop Functions

void aput_bridge(ComplexType *res, cRealType m)
{
    Aput(res, m);
}

void bput_bridge(ComplexType *res, cRealType p, cRealType m1, cRealType m2)
{
    Bput(res, p, m1, m2);
}

void cput_bridge(ComplexType *res, cRealType p1, cRealType p2, cRealType p1p2, cRealType m1, cRealType m2, cRealType m3)
{
    Cput(res, p1, p2, p1p2, m1, m2, m3);
}

void dput_bridge(ComplexType *res, cRealType p1, cRealType p2, cRealType p3, cRealType p4, cRealType p1p2, cRealType p2p3, cRealType m1, cRealType m2, cRealType m3, cRealType m4)
{
    Dput(res, p1, p2, p3, p4, p1p2, p2p3, m1, m2, m3, m4);
}

void eput_bridge(ComplexType *res, cRealType p1, cRealType p2, cRealType p3, cRealType p4, cRealType p5, cRealType p1p2, cRealType p2p3, cRealType p3p4, cRealType p4p5, cRealType p5p1, cRealType m1, cRealType m2, cRealType m3, cRealType m4, cRealType m5)
{
    Eput(res, p1, p2, p3, p4, p5, p1p2, p2p3, p3p4, p4p5, p5p1, m1, m2, m3, m4, m5);
}