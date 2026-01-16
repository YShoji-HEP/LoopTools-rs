#include "clooptools.h"

void ltini_bridge()
{
    ltini();
}

void ltexi_bridge()
{
    ltexi();
}

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