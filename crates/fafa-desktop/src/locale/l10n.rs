pub const APP_NAME: &str = "FaFa";

pub struct L10n {
    pub app_name: &'static str,
    pub menu_about: &'static str,
    pub menu_quit: &'static str,

    // Common
    pub error_hint: &'static str,
    pub action_clear: &'static str,
    pub copy_success: &'static str,
    pub label_result: &'static str,

    // About window
    pub about_window_title: &'static str,
    pub about_version_prefix: &'static str,
    pub about_description: &'static str,

    // Sidebar
    pub nav_hash: &'static str,
    pub nav_password_hash: &'static str,
    pub nav_qrcode: &'static str,
    pub nav_timestamp: &'static str,
    pub nav_url: &'static str,
    pub nav_number_base: &'static str,
    pub nav_uuid: &'static str,
    pub nav_base64: &'static str,
    pub nav_color: &'static str,
    pub nav_text_case: &'static str,

    // Hash
    pub hash_input_placeholder: &'static str,
    pub hash_md5: &'static str,
    pub hash_sha1: &'static str,
    pub hash_sha256: &'static str,
    pub hash_sha512: &'static str,
    pub hash_sha3_256: &'static str,
    pub hash_sha3_512: &'static str,
    pub hash_blake3: &'static str,

    // Password Hash
    pub hash_password_input: &'static str,
    pub hash_password_hash: &'static str,
    pub hash_password_result: &'static str,
    pub hash_password_error: &'static str,
    pub hash_password_algorithm: &'static str,
    pub hash_password_argon2: &'static str,
    pub hash_password_bcrypt: &'static str,

    // QR Code
    pub qrcode_input_placeholder: &'static str,
    pub qrcode_generate: &'static str,
    pub qrcode_hint: &'static str,
    pub qrcode_ec_caption: &'static str,
    pub qrcode_ec_l: &'static str,
    pub qrcode_ec_m: &'static str,
    pub qrcode_ec_q: &'static str,
    pub qrcode_ec_h: &'static str,
    pub qrcode_error_too_long: &'static str,

    // Timestamp
    pub timestamp_label: &'static str,
    pub timestamp_placeholder: &'static str,
    pub timestamp_unit_seconds: &'static str,
    pub timestamp_unit_milliseconds: &'static str,
    pub timestamp_timezone: &'static str,
    pub timestamp_convert_to_datetime: &'static str,
    pub timestamp_datetime_label: &'static str,
    pub timestamp_datetime_placeholder: &'static str,
    pub timestamp_convert_to_timestamp: &'static str,
    pub timestamp_format: &'static str,
    pub timestamp_now: &'static str,
    pub timestamp_error_timestamp: &'static str,
    pub timestamp_error_datetime: &'static str,
    pub timestamp_error_datetime_tz: &'static str,
    pub timestamp_unit_label: &'static str,

    // URL
    pub url_input_placeholder: &'static str,
    pub url_encode: &'static str,
    pub url_decode: &'static str,

    // Number Base
    pub number_base_input_placeholder: &'static str,
    pub number_base_input_base: &'static str,
    pub number_base_binary: &'static str,
    pub number_base_octal: &'static str,
    pub number_base_decimal: &'static str,
    pub number_base_hex: &'static str,
    pub number_base_convert: &'static str,

    // UUID
    pub uuid_v4: &'static str,
    pub uuid_v7: &'static str,
    pub uuid_new: &'static str,
    pub uuid_style: &'static str,
    pub uuid_version: &'static str,
    pub uuid_dashed: &'static str,
    pub uuid_simple: &'static str,
    pub uuid_count: &'static str,
    pub uuid_count_placeholder: &'static str,
    pub uuid_count_hint: &'static str,
    pub uuid_unit: &'static str,

    // Base64
    pub base64_input_placeholder: &'static str,
    pub base64_encode: &'static str,
    pub base64_decode: &'static str,
    pub base64_error: &'static str,
    pub base64_alphabet: &'static str,
    pub base64_standard: &'static str,
    pub base64_url_safe: &'static str,
    pub base64_padding: &'static str,
    pub base64_padded: &'static str,
    pub base64_unpadded: &'static str,

    // Color
    pub color_input_placeholder: &'static str,
    pub color_convert: &'static str,
    pub color_hex: &'static str,
    pub color_rgb: &'static str,
    pub color_hsl: &'static str,
    pub color_hsla: &'static str,
    pub color_rgba: &'static str,
    pub color_rgba_hex: &'static str,
    pub color_argb_hex: &'static str,
    pub color_error: &'static str,

    // Text Case
    pub text_case_input_placeholder: &'static str,
    pub text_case_convert: &'static str,
    pub text_case_camel: &'static str,
    pub text_case_pascal: &'static str,
    pub text_case_snake: &'static str,
    pub text_case_shouty_snake: &'static str,
    pub text_case_kebab: &'static str,
    pub text_case_shouty_kebab: &'static str,
    pub text_case_train: &'static str,
    pub text_case_title_case: &'static str,
}
