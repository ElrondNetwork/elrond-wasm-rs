elrond_wasm::imports!();
type EllipticCurveComponents<BigUint> = (BigUint, BigUint, BigUint, BigUint, BigUint, u32);

/// All elliptic curve functions provided by Arwen exposed here
#[elrond_wasm_derive::module]
pub trait EllipticCurveFeatures {
 	#[endpoint]
	fn compute_get_values(
		&self,
		curve_bitsize: u32,
	) -> EllipticCurveComponents<Self::BigUint> {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize){
            Some(ec) => ec.get_values(),
            None => (Self::BigUint::zero(),Self::BigUint::zero(),Self::BigUint::zero(),Self::BigUint::zero(),Self::BigUint::zero(),0),//Self::EllipticCurve::p224_ec().get_values()
        }
	}

	#[endpoint]
	fn compute_p224_ec(&self) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::p224_ec().get_values()
	}

	#[endpoint]
	fn compute_p256_ec(&self) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::p256_ec().get_values()
	}

	#[endpoint]
	fn compute_p384_ec(&self) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::p384_ec().get_values()
	}

	#[endpoint]
	fn compute_p521_ec(&self) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::p521_ec().get_values()
	}

    #[endpoint]
    fn compute_get_ec_length(&self, curve_bitsize: u32) -> u32 {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.get_ec_length(),
            None => curve_bitsize,
        }
    }

    #[endpoint]
    fn compute_get_priv_key_byte_length(&self, curve_bitsize: u32,) -> u32 {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.get_priv_key_byte_length(),
            None => 0,
        }
    }

	#[endpoint]
	fn compute_add_ec(
		&self,
		curve_bitsize: u32,
		x_first_point: Self::BigUint,
		y_first_point: Self::BigUint,
		x_second_point: Self::BigUint,
		y_second_point: Self::BigUint,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.add_ec(x_first_point, y_first_point, x_second_point, y_second_point).into(),
            None => (Self::BigUint::zero(),Self::BigUint::zero()).into(),
        }
	}

	#[endpoint]
	fn compute_double_ec(
		&self,
		curve_bitsize: u32,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.double_ec(x_point,y_point).into(),
            None => (Self::BigUint::zero(),Self::BigUint::zero()).into(),
        }
	}

	#[endpoint]
	fn compute_is_on_curve_ec(
		&self,
		curve_bitsize: u32,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> bool {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.is_on_curve_ec(x_point,y_point),
            None => false,
        }
	}

	#[endpoint]
	fn compute_scalar_mult(
		&self,
		curve_bitsize: u32,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
		data: BoxedBytes,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.scalar_mult(x_point,y_point,data).into(),
            None => (Self::BigUint::zero(),Self::BigUint::zero()).into(),
        }
	}

	#[endpoint]
	fn compute_scalar_base_mult(
		&self,
		curve_bitsize: u32,
		data: BoxedBytes,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.scalar_base_mult(data).into(),
            None => (Self::BigUint::zero(),Self::BigUint::zero()).into(),
        }
	}

	#[endpoint]
	fn compute_marshal_ec(
		&self,
		curve_bitsize: u32,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.marshal_ec(x_pair,y_pair),
            None => BoxedBytes::zeros(0),
        }
	}

	#[endpoint]
	fn compute_marshal_compressed_ec(
		&self,
		curve_bitsize: u32,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.marshal_compressed_ec(x_pair,y_pair),
            None => BoxedBytes::zeros(0),
        }
	}

	#[endpoint]
	fn compute_unmarshal_ec(
		&self,
		curve_bitsize: u32,
		data: BoxedBytes,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.unmarshal_ec(data).into(),
            None => (Self::BigUint::zero(),Self::BigUint::zero()).into(),
        }
	}

	#[endpoint]
	fn compute_unmarshal_compressed_ec(
		&self,
		curve_bitsize: u32,
		data: BoxedBytes,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
		match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.unmarshal_compressed_ec(data).into(),
            None => (Self::BigUint::zero(),Self::BigUint::zero()).into(),
        }
	}

	#[endpoint]
	fn compute_generate_key_ec(
		&self,
		curve_bitsize: u32,
	) -> MultiResult3<Self::BigUint,Self::BigUint,BoxedBytes> {
        match Self::EllipticCurve::from_bitsize_ec(curve_bitsize) {
            Some(ec) => ec.generate_key_ec().into(),
            None => (Self::BigUint::zero(),Self::BigUint::zero(),BoxedBytes::zeros(0)).into(),
        }
	}  
}
