use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::{
            curves::bls12_381::{curve::BLS12381Curve, field_extension::BLS12381PrimeField},
            point::{Endianness, PointFormat},
        },
        traits::IsEllipticCurve,
    },
    field::element::FieldElement,
    traits::ByteConversion, unsigned_integer::element::U256,
};

type FEE = FieldElement<BLS12381PrimeField>;
fn main() {
    //method one
    let generator = BLS12381Curve::generator();
    let private_key_field = FEE::from_hex("0x6C616D6264617370").unwrap();
    let private_key_value = private_key_field.representative();
    let public_key_bytes = generator
        .operate_with_self(private_key_value)
        .serialize(PointFormat::Uncompressed, Endianness::BigEndian);
    let public_key_method_one = FEE::from_bytes_be(&public_key_bytes).unwrap();
    println!("public_key {:?}", public_key_method_one.to_hex());

    //method two
    let pk = U256::from_hex_unchecked("0x6C616D6264617370");
    let public_key_bytes = generator
        .operate_with_self(pk)
        .serialize(PointFormat::Uncompressed, Endianness::BigEndian);
    let public_key_method_two = FEE::from_bytes_be(&public_key_bytes).unwrap();
    println!("public_key {:?}", public_key_method_two.to_hex());

    assert_eq!(public_key_method_one, public_key_method_two)
}
