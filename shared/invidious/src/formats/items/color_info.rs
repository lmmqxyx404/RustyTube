use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorInfo {
	pub primaries: String,
	pub transfer_characteristics: String,
	pub matrix_coefficients: String,
}