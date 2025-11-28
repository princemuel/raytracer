mod common;

use proptest::prelude::*;
use raytracer::prelude::*;

#[test]
fn it_converts_point_to_tuple4_point_representation() {
    let p = point(1, 2, 3);
    let t: Tuple4 = p.into();

    assert_eq!(t.x(), 1.0);
    assert_eq!(t.y(), 2.0);
    assert_eq!(t.z(), 3.0);
    assert_eq!(t.w(), 1.0);
    assert!(t.is_point());
}

#[test]
fn it_converts_vector_to_tuple4_vector_representation() {
    let v = vector(1, 2, 3);
    let t: Tuple4 = v.into();

    assert_eq!(t.x(), 1.0);
    assert_eq!(t.y(), 2.0);
    assert_eq!(t.z(), 3.0);
    assert_eq!(t.w(), 0.0);
    assert!(t.is_vector());
}

#[test]
fn it_converts_tuple4_with_w_equals_1_into_point3() {
    let t = tuple(1, 2, 3, 1);
    let p: Result<Point3, _> = t.try_into();

    assert!(p.is_ok());
    let p = p.unwrap();
    assert_eq!(p.x(), 1.0);
    assert_eq!(p.y(), 2.0);
    assert_eq!(p.z(), 3.0);
}

#[test]
fn it_rejects_tuple4_point_conversion_with_wrong_w() {
    let t = tuple(1, 2, 3, 0);
    let p: Result<Point3, _> = t.try_into();

    assert!(p.is_err());
}

#[test]
fn it_converts_tuple4_with_w_equals_0_into_vec3() {
    let t = tuple(1, 2, 3, 0);
    let v: Result<Vec3, _> = t.try_into();

    assert!(v.is_ok());
    let v = v.unwrap();
    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);
}

#[test]
fn it_rejects_tuple4_vector_conversion_with_wrong_w() {
    let t = tuple(1, 2, 3, 1);
    let v: Result<Vec3, _> = t.try_into();

    assert!(v.is_err());
}

#[test]
fn that_point_vector_arithmetic_roundtrip_behaves_consistently() {
    let p1 = point(3, 2, 1);
    let v = vector(1, 1, 1);

    let p2 = p1 + v;
    assert_eq!(p2, point(4, 3, 2));

    let v_back = p2 - p1;
    assert_eq!(v_back, v);
}

proptest! {
    #[test]
    fn that_vector_normalization_produces_unit_length(
        x in -1000.0..1000.0,
        y in -1000.0..1000.0,
        z in -1000.0..1000.0
    ) {
        let v = vector(x, y, z);
        if let Some(n) = v.try_normalize() {
            prop_assert!(is_equal(n.length(), 1.0));
        }
    }

    #[test]
    fn that_cross_product_is_perpendicular(
        ax in -100.0..100.0, ay in -100.0..100.0, az in -100.0..100.0,
        bx in -100.0..100.0, by in -100.0..100.0, bz in -100.0..100.0
    ) {
        let a = vector(ax, ay, az);
        let b = vector(bx, by, bz);
        let c = a.cross(b);

        // Cross product should be perpendicular to both inputs
        prop_assert!(is_equal(c.dot(a), 0.0));
        prop_assert!(is_equal(c.dot(b), 0.0));
    }
}
