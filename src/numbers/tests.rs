// Copyright 2018 Vlad Yermakov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[test]
fn test_naturals() {
    use super::Natural;

    fn check_macro() {
        let a = natural!(3);
        let b = Natural::new(3);

        assert_eq!(a, b);
    }

    fn check_ops() {
        let a = natural!(3);
        let b = natural!(2);
        let i = integer!(2);

        let c = a + b;
        let d = a - b;
        let e = a * b;
        let f = a / b;
        let g = a % b;
        let h = a % i;
        let m = -a;

        assert_eq!(c, natural!(5));
        assert_eq!(d, natural!(1));
        assert_eq!(e, natural!(6));
        assert_eq!(f, natural!(1));
        assert_eq!(g, natural!(1));
        assert_eq!(h, integer!(1));
        assert_eq!(m, integer!(-3));
    }

    fn check_cmp() {
        let a = natural!(3);
        let b = natural!(2);

        assert_eq!(a < b, false);
        assert_eq!(a > b, true);
        assert_eq!(a >= b, true);
        assert_eq!(a <= b, false);
        assert_eq!(a == b, false);
        assert_eq!(a != b, true);
    }

    fn check_format() {
        let a = natural!(13);

        assert_eq!(format!("{}", a), "13");
    }

    check_macro();
    check_ops();
    check_cmp();
    check_format();
}

#[test]
fn test_integers() {
    use super::Integer;

    fn check_macro() {
        let a = integer!(3);
        let b = Integer::new(3);

        assert_eq!(a, b);
    }

    fn check_ops() {
        let a = integer!(3);
        let b = integer!(4);
        let n = natural!(2);

        let c = a + b;
        let d = a - b;
        let e = a * b;
        let f = a / b;
        let g = a % b;
        let h = a % n;
        let m = -a;

        assert_eq!(c, integer!(7));
        assert_eq!(d, integer!(-1));
        assert_eq!(e, integer!(12));
        assert_eq!(f, integer!(0));
        assert_eq!(g, integer!(3));
        assert_eq!(h, integer!(1));
        assert_eq!(m, integer!(-3));
    }

    fn check_cmp() {
        let a = integer!(3);
        let b = integer!(4);

        assert_eq!(a > b, false);
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a >= b, false);
        assert_eq!(a == b, false);
        assert_eq!(a != b, true);
    }

    fn check_format() {
        let a = integer!(-13);

        assert_eq!(format!("{}", a), "-13");
    }

    check_macro();
    check_ops();
    check_cmp();
    check_format();
}

#[test]
fn test_rationals() {
    use super::Rational;

    fn check_macro() {
        let a = rational!(3 / 4);
        assert_eq!(a, Rational::new(3, 4));
    }

    fn check_norm() {
        let a = rational!(4 / 12);
        assert_eq!(a, rational!(1 / 3));
        assert_eq!(a, rational!(5 / 15));
    }

    fn check_ops() {
        let a = rational!(3 / 4);
        let b = rational!(4 / 12);

        let c = a + b;
        let d = a - b;
        let e = a * b;
        let f = a / b;
        let m = -a;

        assert_eq!(c, rational!(13 / 12));
        assert_eq!(d, rational!(5 / 12));
        assert_eq!(e, rational!(1 / 4));
        assert_eq!(f, rational!(9 / 4));
        assert_eq!(m, rational!(-3 / 4));
    }

    fn check_inv() {
        let a = rational!(1 / 3);
        let a = a.inv();
        assert_eq!(a, rational!(3));
    }

    fn check_cmp() {
        let a = rational!(3 / 4);
        let b = rational!(1 / 3);

        assert_eq!(a < b, false);
        assert_eq!(a > b, true);
        assert_eq!(a >= b, true);
        assert_eq!(a <= b, false);
        assert_eq!(a == b, false);
        assert_eq!(a != b, true);
    }

    fn check_format() {
        let a = rational!(2 / 3);

        assert_eq!(format!("{}", a), "2 / 3");
    }

    check_macro();
    check_norm();
    check_ops();
    check_inv();
    check_cmp();
    check_format();
}

#[test]
fn test_real() {
    use super::Real;

    fn check_macro() {
        let a = real!(3);
        let b = real!(-3);

        assert_eq!(a, Real::new(3));
        assert_eq!(b, Real::new(-3));
    }

    fn check_ops() {
        let a = real!(2.5);
        let b = real!(1.5);

        let c = a + b;
        let d = a - b;
        let e = a * b;
        let f = a / b;
        let m = -a;

        assert_eq!(c, real!(4));
        assert_eq!(d, real!(1));
        assert_eq!(e, real!(3.75));
        assert_eq!(f, real!(1.6666666666666667));
        assert_eq!(m, real!(-2.5));
    }

    fn check_sqrt() {
        use super::Complex;
        use super::Sqrt;

        let a = real!(4);
        let b = real!(-4);

        let c = <Real as Sqrt<Real>>::try_sqrt(&a);
        let d = <Real as Sqrt<Real>>::try_sqrt(&b);

        let e = <Real as Sqrt<Complex>>::sqrt(&a);
        let f = <Real as Sqrt<Complex>>::sqrt(&b);

        assert_eq!(c, Some(real!(2)));
        assert_eq!(d, None);

        assert_eq!(e, complex!(2));
        assert_eq!(f, complex!(2.i));
    }

    fn check_format() {
        let a = real!(1.33);

        assert_eq!(format!("{}", a), "1.33");
    }

    check_macro();
    check_ops();
    check_sqrt();
    check_format();
}

#[test]
fn test_complex() {
    use super::Complex;

    fn check_macro() {
        let a = complex!(2 + 3.i);
        let b = complex!(2 - 3.i);
        let c = complex!(2);
        let d = complex!(1.i);
        let e = complex!(-2);
        let f = complex!(-3.i);

        assert_eq!(a, Complex::new(2, 3));
        assert_eq!(b, Complex::new(2, -3));
        assert_eq!(c, Complex::new(2, 0));
        assert_eq!(d, Complex::new(0, 1));
        assert_eq!(e, Complex::new(-2, 0));
        assert_eq!(f, Complex::new(0, -3));
    }

    fn check_ops() {
        let a = complex!(4 + 3.i);
        let b = complex!(4 - 3.i);

        let d = a + b;
        let e = a - b;
        let f = a * b;
        let g = a / b;
        let m = -a;

        assert_eq!(d, complex!(8));
        assert_eq!(e, complex!(6.i));
        assert_eq!(f, complex!(25));
        assert_eq!(g, complex!(0.28 + 0.96.i));
        assert_eq!(m, complex!(-4 - 3.i));
    }

    fn check_conj() {
        let b = complex!(2 - 3.i).conj();
        assert_eq!(b, complex!(2 + 3.i));
    }

    fn check_i2() {
        let i = complex!(1.i);
        assert_eq!(i * i, complex!(-1));
    }

    fn check_inv() {
        let a = complex!(3 + 4.i).inv();
        assert_eq!(a, complex!(0.12 - 0.16.i));
    }

    fn check_format() {
        let a = complex!(2 + 3.i);

        assert_eq!(format!("{}", a), "2 + 3i");
    }

    check_macro();
    check_ops();
    check_inv();
    check_conj();
    check_i2();
    check_format();
}

#[test]
fn test_cross_types_add() {
    let a = natural!(3);
    let b = integer!(-5);
    let c = rational!(2 / 3);
    let d = real!(15);
    let e = complex!(3 + 5.i);

    let aa = a + a;
    let ab = a + b;
    let ac = a + c;
    let ad = a + d;
    let ae = a + e;

    let ba = b + a;
    let bb = b + b;
    let bc = b + c;
    let bd = b + d;
    let be = b + e;

    let ca = c + a;
    let cb = c + b;
    let cc = c + c;
    let cd = c + d;
    let ce = c + e;

    let da = d + a;
    let db = d + b;
    let dc = d + c;
    let dd = d + d;
    let de = d + e;

    let ea = e + a;
    let eb = e + b;
    let ec = e + c;
    let ed = e + d;
    let ee = e + e;

    assert_eq!(aa, natural!(6));
    assert_eq!(ab, integer!(-2));
    assert_eq!(ac, rational!(11 / 3));
    assert_eq!(ad, real!(18));
    assert_eq!(ae, complex!(6 + 5.i));

    assert_eq!(ba, integer!(-2));
    assert_eq!(bb, integer!(-10));
    assert_eq!(bc, rational!(-13 / 3));
    assert_eq!(bd, real!(10));
    assert_eq!(be, complex!(-2 + 5.i));

    assert_eq!(ca, rational!(11 / 3));
    assert_eq!(cb, rational!(-13 / 3));
    assert_eq!(cc, rational!(4 / 3));
    assert_eq!(cd, real!(15.666666666666666));
    assert_eq!(ce, complex!(3.6666666666666665 + 5.i));

    assert_eq!(da, real!(18));
    assert_eq!(db, real!(10));
    assert_eq!(dc, real!(15.666666666666666));
    assert_eq!(dd, real!(30));
    assert_eq!(de, complex!(18 + 5.i));

    assert_eq!(ea, complex!(6 + 5.i));
    assert_eq!(eb, complex!(-2 + 5.i));
    assert_eq!(ec, complex!(3.6666666666666665 + 5.i));
    assert_eq!(ed, complex!(18 + 5.i));
    assert_eq!(ee, complex!(6 + 10.i));
}

#[test]
fn test_cross_types_sub() {
    let a = natural!(3);
    let b = integer!(-5);
    let c = rational!(2 / 3);
    let d = real!(15);
    let e = complex!(3 + 5.i);

    let aa = a - a;
    let ab = a - b;
    let ac = a - c;
    let ad = a - d;
    let ae = a - e;

    let ba = b - a;
    let bb = b - b;
    let bc = b - c;
    let bd = b - d;
    let be = b - e;

    let ca = c - a;
    let cb = c - b;
    let cc = c - c;
    let cd = c - d;
    let ce = c - e;

    let da = d - a;
    let db = d - b;
    let dc = d - c;
    let dd = d - d;
    let de = d - e;

    let ea = e - a;
    let eb = e - b;
    let ec = e - c;
    let ed = e - d;
    let ee = e - e;

    assert_eq!(aa, natural!(0));
    assert_eq!(ab, integer!(8));
    assert_eq!(ac, rational!(7 / 3));
    assert_eq!(ad, real!(-12));
    assert_eq!(ae, complex!(-5.i));

    assert_eq!(ba, integer!(-8));
    assert_eq!(bb, integer!(0));
    assert_eq!(bc, rational!(-17 / 3));
    assert_eq!(bd, real!(-20));
    assert_eq!(be, complex!(-8 - 5.i));

    assert_eq!(ca, rational!(-7 / 3));
    assert_eq!(cb, rational!(17 / 3));
    assert_eq!(cc, rational!(0));
    assert_eq!(cd, real!(-14.333333333333334));
    assert_eq!(ce, complex!(-2.3333333333333335 - 5.i));

    assert_eq!(da, real!(12));
    assert_eq!(db, real!(20));
    assert_eq!(dc, real!(14.333333333333334));
    assert_eq!(dd, real!(0));
    assert_eq!(de, complex!(12 - 5.i));

    assert_eq!(ea, complex!(5.i));
    assert_eq!(eb, complex!(8 + 5.i));
    assert_eq!(ec, complex!(2.3333333333333335 + 5.i));
    assert_eq!(ed, complex!(-12 + 5.i));
    assert_eq!(ee, complex!(0));
}

#[test]
fn test_cross_types_mul() {
    let a = natural!(3);
    let b = integer!(-5);
    let c = rational!(2 / 3);
    let d = real!(15);
    let e = complex!(3 + 5.i);

    let aa = a * a;
    let ab = a * b;
    let ac = a * c;
    let ad = a * d;
    let ae = a * e;

    let ba = b * a;
    let bb = b * b;
    let bc = b * c;
    let bd = b * d;
    let be = b * e;

    let ca = c * a;
    let cb = c * b;
    let cc = c * c;
    let cd = c * d;
    let ce = c * e;

    let da = d * a;
    let db = d * b;
    let dc = d * c;
    let dd = d * d;
    let de = d * e;

    let ea = e * a;
    let eb = e * b;
    let ec = e * c;
    let ed = e * d;
    let ee = e * e;

    assert_eq!(aa, natural!(9));
    assert_eq!(ab, integer!(-15));
    assert_eq!(ac, rational!(2));
    assert_eq!(ad, real!(45));
    assert_eq!(ae, complex!(9 + 15.i));

    assert_eq!(ba, integer!(-15));
    assert_eq!(bb, integer!(25));
    assert_eq!(bc, rational!(-10 / 3));
    assert_eq!(bd, real!(-75));
    assert_eq!(be, complex!(-15 - 25.i));

    assert_eq!(ca, rational!(2));
    assert_eq!(cb, rational!(-10 / 3));
    assert_eq!(cc, rational!(4 / 9));
    assert_eq!(cd, real!(10));
    assert_eq!(ce, complex!(2 + 3.333333333333333.i));

    assert_eq!(da, real!(45));
    assert_eq!(db, real!(-75));
    assert_eq!(dc, real!(10));
    assert_eq!(dd, real!(225));
    assert_eq!(de, complex!(45 + 75.i));

    assert_eq!(ea, complex!(9 + 15.i));
    assert_eq!(eb, complex!(-15 - 25.i));
    assert_eq!(ec, complex!(2 + 3.333333333333333.i));
    assert_eq!(ed, complex!(45 + 75.i));
    assert_eq!(ee, complex!(-16 + 30.i));
}

#[test]
fn test_cross_types_div() {
    let a = natural!(3);
    let b = integer!(-5);
    let c = rational!(2 / 3);
    let d = real!(15);
    let e = complex!(3 + 5.i);

    let aa = a / a;
    let ab = a / b;
    let ac = a / c;
    let ad = a / d;
    let ae = a / e;

    let ba = b / a;
    let bb = b / b;
    let bc = b / c;
    let bd = b / d;
    let be = b / e;

    let ca = c / a;
    let cb = c / b;
    let cc = c / c;
    let cd = c / d;
    let ce = c / e;

    let da = d / a;
    let db = d / b;
    let dc = d / c;
    let dd = d / d;
    let de = d / e;

    let ea = e / a;
    let eb = e / b;
    let ec = e / c;
    let ed = e / d;
    let ee = e / e;

    assert_eq!(aa, natural!(1));
    assert_eq!(ab, integer!(0));
    assert_eq!(ac, rational!(9 / 2));
    assert_eq!(ad, real!(0.2));
    assert_eq!(ae, complex!(0.2647058823529412 - 0.4411764705882353.i));

    assert_eq!(ba, integer!(-1));
    assert_eq!(bb, integer!(1));
    assert_eq!(bc, rational!(-15 / 2));
    assert_eq!(bd, real!(-0.3333333333333333));
    assert_eq!(be, complex!(-0.4411764705882353 + 0.7352941176470589.i));

    assert_eq!(ca, rational!(2 / 9));
    assert_eq!(cb, rational!(-2 / 15));
    assert_eq!(cc, rational!(1));
    assert_eq!(cd, real!(0.04444444444444444));
    assert_eq!(ce, complex!(0.058823529411764705 - 0.0980392156862745.i));

    assert_eq!(da, real!(5));
    assert_eq!(db, real!(-3));
    assert_eq!(dc, real!(22.5));
    assert_eq!(dd, real!(1));
    assert_eq!(de, complex!(1.3235294117647058 - 2.2058823529411766.i));

    assert_eq!(ea, complex!(1 + 1.6666666666666667.i));
    assert_eq!(eb, complex!(-0.6 - 1.i));
    assert_eq!(ec, complex!(4.5 + 7.5.i));
    assert_eq!(ed, complex!(0.2 + 0.3333333333333333.i));
    assert_eq!(ee, complex!(1));
}

#[test]
fn test_number_ops() {
    use super::Number;

    let a = Number::Natural(natural!(3));
    let b = Number::Integer(integer!(5));
    let c = Number::Rational(rational!(1 / 3));
    let d = Number::Real(real!(0.5));
    let e = Number::Complex(complex!(2.i));

    let ab = a + b;
    let bc = b - c;
    let cd = c * d;
    let de = d / e;

    assert_eq!(ab, Number::Integer(integer!(8)));
    assert_eq!(bc, Number::Rational(rational!(14 / 3)));
    assert_eq!(cd, Number::Real(real!(0.16666666666666666)));
    assert_eq!(de, Number::Complex(complex!(-0.25.i)));
}

#[test]
fn test_assign_ops() {
    fn check_natural() {
        let mut a = natural!(1);
        let b = natural!(2);

        a += b;
        assert_eq!(a, natural!(3));

        a -= b;
        assert_eq!(a, natural!(1));

        a *= b;
        assert_eq!(a, natural!(2));

        a /= b;
        assert_eq!(a, natural!(1));
    }

    fn check_integer() {
        let mut a = integer!(1);
        let b = integer!(3);

        a += b;
        assert_eq!(a, integer!(4));

        a -= b;
        assert_eq!(a, integer!(1));

        a *= b;
        assert_eq!(a, integer!(3));

        a /= b;
        assert_eq!(a, integer!(1));
    }

    fn check_rational() {
        let mut a = rational!(1);
        let b = rational!(3);

        a += b;
        assert_eq!(a, rational!(4));

        a -= b;
        assert_eq!(a, rational!(1));

        a *= b;
        assert_eq!(a, rational!(3));

        a /= b;
        assert_eq!(a, rational!(1));
    }

    fn check_real() {
        let mut a = real!(1);
        let b = real!(3);

        a += b;
        assert_eq!(a, real!(4));

        a -= b;
        assert_eq!(a, real!(1));

        a *= b;
        assert_eq!(a, real!(3));

        a /= b;
        assert_eq!(a, real!(1));
    }

    fn check_complex() {
        let mut a = complex!(1);
        let b = complex!(3);

        a += b;
        assert_eq!(a, complex!(4));

        a -= b;
        assert_eq!(a, complex!(1));

        a *= b;
        assert_eq!(a, complex!(3));

        a /= b;
        assert_eq!(a, complex!(1));
    }

    check_natural();
    check_integer();
    check_rational();
    check_real();
    check_complex();
}

#[test]
fn test_cross_type_assign_ops() {
    fn check_integer_natural() {
        let mut a = integer!(5);
        let b = natural!(2);

        a += b;
        assert_eq!(a, integer!(7));

        a -= b;
        assert_eq!(a, integer!(5));

        a *= b;
        assert_eq!(a, integer!(10));

        a /= b;
        assert_eq!(a, integer!(5));

        a %= b;
        assert_eq!(a, integer!(1));
    }

    fn check_rational_natural() {
        let mut a = rational!(5 / 3);
        let b = natural!(2);

        a += b;
        assert_eq!(a, rational!(11 / 3));

        a -= b;
        assert_eq!(a, rational!(5 / 3));

        a *= b;
        assert_eq!(a, rational!(10 / 3));

        a /= b;
        assert_eq!(a, rational!(5 / 3));
    }

    fn check_real_natural() {
        let mut a = real!(2.5);
        let b = natural!(2);

        a += b;
        assert_eq!(a, real!(4.5));

        a -= b;
        assert_eq!(a, real!(2.5));

        a *= b;
        assert_eq!(a, real!(5));

        a /= b;
        assert_eq!(a, real!(2.5));
    }

    fn check_complex_natural() {
        let mut a = complex!(3 + 2.i);
        let b = natural!(2);

        a += b;
        assert_eq!(a, complex!(5 + 2.i));

        a -= b;
        assert_eq!(a, complex!(3 + 2.i));

        a *= b;
        assert_eq!(a, complex!(6 + 4.i));

        a /= b;
        assert_eq!(a, complex!(3 + 2.i));
    }

    fn check_rational_integer() {
        let mut a = rational!(5 / 2);
        let b = integer!(3);

        a += b;
        assert_eq!(a, rational!(11 / 2));

        a -= b;
        assert_eq!(a, rational!(5 / 2));

        a *= b;
        assert_eq!(a, rational!(15 / 2));

        a /= b;
        assert_eq!(a, rational!(5 / 2));
    }

    fn check_real_integer() {
        let mut a = real!(3.2);
        let b = integer!(5);

        a += b;
        assert_eq!(a, real!(8.2));

        a -= b;
        assert_eq!(a, real!(3.2));

        a *= b;
        assert_eq!(a, real!(16));

        a /= b;
        assert_eq!(a, real!(3.2));
    }

    fn check_complex_integer() {
        let mut a = complex!(3 + 2.i);
        let b = integer!(2);

        a += b;
        assert_eq!(a, complex!(5 + 2.i));

        a -= b;
        assert_eq!(a, complex!(3 + 2.i));

        a *= b;
        assert_eq!(a, complex!(6 + 4.i));

        a /= b;
        assert_eq!(a, complex!(3 + 2.i));
    }

    fn check_real_rational() {
        let mut a = real!(3.2);
        let b = rational!(4 / 5);

        a += b;
        assert_eq!(a, real!(4));

        a -= b;
        assert_eq!(a, real!(3.2));

        a *= b;
        assert_eq!(a, real!(2.56));

        a /= b;
        assert_eq!(a, real!(3.2));
    }

    fn check_complex_rational() {
        let mut a = complex!(1.25 + 2.75.i);
        let b = rational!(3 / 4);

        a += b;
        assert_eq!(a, complex!(2 + 2.75.i));

        a -= b;
        assert_eq!(a, complex!(1.25 + 2.75.i));

        a *= b;
        assert_eq!(a, complex!(0.9375 + 2.0625.i));

        a /= b;
        assert_eq!(a, complex!(1.25 + 2.75.i));
    }

    fn check_complex_real() {
        let mut a = complex!(2.5 + 3.i);
        let b = real!(0.5);

        a += b;
        assert_eq!(a, complex!(3 + 3.i));

        a -= b;
        assert_eq!(a, complex!(2.5 + 3.i));

        a *= b;
        assert_eq!(a, complex!(1.25 + 1.5.i));

        a /= b;
        assert_eq!(a, complex!(2.5 + 3.i));
    }

    check_integer_natural();
    check_rational_natural();
    check_real_natural();
    check_complex_natural();

    check_rational_integer();
    check_real_integer();
    check_complex_integer();

    check_real_rational();
    check_complex_rational();

    check_complex_real();
}

#[test]
fn test_default() {
    use super::{Complex, Integer, Natural, Rational, Real};

    let a = Natural::default();
    let b = Integer::default();
    let c = Rational::default();
    let d = Real::default();
    let e = Complex::default();

    assert_eq!(a, natural!(0));
    assert_eq!(b, integer!(0));
    assert_eq!(c, rational!(0));
    assert_eq!(d, real!(0));
    assert_eq!(e, complex!(0));
}

#[test]
fn test_if() {}
