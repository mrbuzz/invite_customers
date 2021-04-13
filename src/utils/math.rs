const DUBLIN_HQ_LATITUDE: f64 = 53.339428_f64;
const DUBLIN_HQ_LONGITUDE: f64 = -6.257664_f64;
const EARTH_RADIUS_KILOMETERS: f64 = 6371.0_f64;
///
/// Computes the distance from Dublin HQ, result is rounded to the next greater value. 
///
pub fn compute_distance_from_dublin_hq(latitude: f64, longitude: f64) -> f64 {
  let ( dublin_latitude_rad, self_latitude_rad ) = (DUBLIN_HQ_LATITUDE.to_radians(), latitude.to_radians());
  let ( dublin_longitude_rad, self_longitude_rad ) = (DUBLIN_HQ_LONGITUDE.to_radians(), longitude.to_radians());

  let delta_longitude = (dublin_longitude_rad - self_longitude_rad).abs();

  let latitude_sin_product = dublin_latitude_rad.sin() * self_latitude_rad.sin();
  let latitude_cos_product = dublin_latitude_rad.cos() * self_latitude_rad.cos();


  let central_angle = (latitude_sin_product + (latitude_cos_product * delta_longitude.cos())).acos();
  return (EARTH_RADIUS_KILOMETERS * central_angle).round();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn calculates_the_correct_distance_from_dublin_when_latitude_is_below_dublin() {
    let (latitude, longitude) = (51.50853_f64, -0.12574_f64);

    assert_eq!(compute_distance_from_dublin_hq(latitude, longitude), 463.0_f64);
  }

  #[test]
  fn calculates_the_correct_distance_from_dublin_when_latitude_is_above_dublin() {
    let (latitude, longitude) = (54.50853_f64, -0.12574_f64);

    assert_eq!(compute_distance_from_dublin_hq(latitude, longitude), 422.0_f64);
  }

  #[test]
  fn calculates_the_correct_distance_from_dublin_when_longitude_is_positive() {
    let (latitude, longitude) = (38.17225598712151_f64, 23.72175331668833_f64);

    assert_eq!(compute_distance_from_dublin_hq(latitude, longitude), 2839.0_f64);
  }
}