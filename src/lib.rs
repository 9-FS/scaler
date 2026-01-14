// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
mod format;
// mod from_str;
pub mod options;
pub use options::*;
pub mod round;
pub use round::*;


/// # Summary
/// A convenient formatter to scale, round, and display numbers. More information about available options and can be found at the setter functions and the format function itself.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Formatter
{
    decimal_separator: String,
    group_separator:   String,
    rounding:          Rounding,
    scaling:           Scaling,
    sign:              Sign,
    trailing_zeros:    bool,
}


impl Formatter
{
    /// # Summary
    /// Constructs default Formatter with only sign when negative, decimal scaling, rounding to 4 significant digits, "." as thousand separator, "," as decimal separator, and trailing zeros enabled.
    ///
    /// # Returns
    /// - Formatter
    pub fn new() -> Self
    {
        return Self {
            decimal_separator: ",".to_string(),
            group_separator:   ".".to_string(),
            rounding:          Rounding::SignificantDigits(4),
            scaling:           Scaling::Decimal(true),
            sign:              Sign::OnlyMinus,
            trailing_zeros:    true,
        };
    }


    /// # Summary
    /// Sets the rounding mode and precision.
    ///
    /// # Arguments
    /// - `rounding_mode`: new rounding mode, contains precision
    ///     - `Magnitude`
    ///         - Round to digit at magnitude 10^m.
    ///         - Contains m.
    ///     - `SignificantDigits`
    ///         - Round to n significant numbers.
    ///         - Contains n.
    ///
    /// # Returns
    /// - modified self
    ///
    /// # Examples
    ///
    /// Examples have scaling disabled for easier understanding.
    ///
    /// ## Magnitude
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::None)
    ///     .set_rounding(scaler::Rounding::Magnitude(-2));
    /// assert_eq!(f.format(123.456), "123,46");
    /// assert_eq!(f.format(0.789), "0,79");
    /// assert_eq!(f.format(42069), "42.069,00");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::None)
    ///     .set_rounding(scaler::Rounding::Magnitude(-1));
    /// assert_eq!(f.format(123.456), "123,5");
    /// assert_eq!(f.format(0.789), "0,8");
    /// assert_eq!(f.format(42069), "42.069,0");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::Magnitude(0));
    /// assert_eq!(f.format(123.456), "123");
    /// assert_eq!(f.format(0.789), "1");
    /// assert_eq!(f.format(42069), "42.069");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::Magnitude(1));
    /// assert_eq!(f.format(123.456), "120");
    /// assert_eq!(f.format(0.789), "0");
    /// assert_eq!(f.format(42069), "42.070");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::Magnitude(2));
    /// assert_eq!(f.format(123.456), "100");
    /// assert_eq!(f.format(0.789), "0");
    /// assert_eq!(f.format(42069), "42.100");
    /// ```
    ///
    /// ## Significant Digits
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::SignificantDigits(0));
    /// assert_eq!(f.format(123.456), "0");
    /// assert_eq!(f.format(0.789), "0");
    /// assert_eq!(f.format(42069), "0");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::SignificantDigits(1));
    /// assert_eq!(f.format(123.456), "100");
    /// assert_eq!(f.format(0.789), "0,8");
    /// assert_eq!(f.format(42069), "40.000");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::SignificantDigits(2));
    /// assert_eq!(f.format(123.456), "120");
    /// assert_eq!(f.format(0.789), "0,79");
    /// assert_eq!(f.format(42069), "42.000");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::SignificantDigits(3));
    /// assert_eq!(f.format(123.456), "123");
    /// assert_eq!(f.format(0.789), "0,789");
    /// assert_eq!(f.format(42069), "42.100");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::SignificantDigits(4));
    /// assert_eq!(f.format(123.456), "123,5");
    /// assert_eq!(f.format(0.789), "0,7890");
    /// assert_eq!(f.format(42069), "42.070");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_rounding(scaler::Rounding::SignificantDigits(5));
    /// assert_eq!(f.format(123.456), "123,46");
    /// assert_eq!(f.format(0.789), "0,78900");
    /// assert_eq!(f.format(42069), "42.069");
    /// ```
    pub fn set_rounding(mut self, rounding: Rounding) -> Self
    {
        self.rounding = rounding;
        return self;
    }


    /// # Summary
    /// Sets the scaling mode.
    ///
    /// # Arguments
    /// - `scaling`: new scaling mode
    ///     - `Binary`
    ///         - Scales by factor 2^(10) = 1024.
    ///         - If no prefix for that magnitude defined: Fallback to scientific notation.
    ///         - Contains whether or not to put space between number and unit.
    ///     - `Decimal`
    ///         - Scales by factor 10^(3) = 1000.
    ///         - If no prefix for that magnitude defined: Fallback to scientific notation.
    ///         - Contains whether or not to put space between number and unit.
    ///     - `None`
    ///         - no scaling
    ///         - no fallback to scientific notation
    ///     - `Scientific`
    ///         - always scientific notation
    ///
    /// # Returns
    /// - modified self
    ///
    /// # Examples
    /// ## Binary
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::Binary(true));
    /// assert_eq!(f.format(0.5), "1,000 * 2^(-1) ");
    /// assert_eq!(f.format(1), "1,000 ");
    /// assert_eq!(f.format(64), "64,00 ");
    /// assert_eq!(f.format(128), "128,0 ");
    /// assert_eq!(f.format(1023), "1.023 ");
    /// assert_eq!(f.format(1024), "1,000 Ki");
    /// assert_eq!(f.format(2_f64.powi(10)), "1,000 Ki");
    /// assert_eq!(f.format(2_f64.powi(20)), "1,000 Mi");
    /// assert_eq!(f.format(2_f64.powi(30)), "1,000 Gi");
    /// assert_eq!(f.format(2_f64.powi(40)), "1,000 Ti");
    /// assert_eq!(f.format(2_f64.powi(50)), "1,000 Pi");
    /// assert_eq!(f.format(2_f64.powi(60)), "1,000 Ei");
    /// assert_eq!(f.format(2_f64.powi(70)), "1,000 Zi");
    /// assert_eq!(f.format(2_f64.powi(80)), "1,000 Yi");
    /// assert_eq!(f.format(2_f64.powi(90)), "1,000 * 2^(90) ");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::Binary(true));
    /// assert_eq!(f.format(-0.5), "-1,000 * 2^(-1) ");
    /// assert_eq!(f.format(-1), "-1,000 ");
    /// assert_eq!(f.format(-64), "-64,00 ");
    /// assert_eq!(f.format(-128), "-128,0 ");
    /// assert_eq!(f.format(-1023), "-1.023 ");
    /// assert_eq!(f.format(-1024), "-1,000 Ki");
    /// assert_eq!(f.format(-2_f64.powi(10)), "-1,000 Ki");
    /// assert_eq!(f.format(-2_f64.powi(20)), "-1,000 Mi");
    /// assert_eq!(f.format(-2_f64.powi(30)), "-1,000 Gi");
    /// assert_eq!(f.format(-2_f64.powi(40)), "-1,000 Ti");
    /// assert_eq!(f.format(-2_f64.powi(50)), "-1,000 Pi");
    /// assert_eq!(f.format(-2_f64.powi(60)), "-1,000 Ei");
    /// assert_eq!(f.format(-2_f64.powi(70)), "-1,000 Zi");
    /// assert_eq!(f.format(-2_f64.powi(80)), "-1,000 Yi");
    /// assert_eq!(f.format(-2_f64.powi(90)), "-1,000 * 2^(90) ");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::Binary(false));
    /// assert_eq!(f.format(1024), "1,000Ki");
    /// ```
    ///
    /// ## Decimal
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::Decimal(true));
    /// assert_eq!(f.format(1e-31), "1,000 * 10^(-31) ");
    /// assert_eq!(f.format(1e-30), "1,000 q");
    /// assert_eq!(f.format(1e-27), "1,000 r");
    /// assert_eq!(f.format(1e-24), "1,000 y");
    /// assert_eq!(f.format(1e-21), "1,000 z");
    /// assert_eq!(f.format(1e-18), "1,000 a");
    /// assert_eq!(f.format(1e-15), "1,000 f");
    /// assert_eq!(f.format(1e-12), "1,000 p");
    /// assert_eq!(f.format(1e-9), "1,000 n");
    /// assert_eq!(f.format(1e-6), "1,000 µ");
    /// assert_eq!(f.format(1e-3), "1,000 m");
    /// assert_eq!(f.format(1), "1,000 ");
    /// assert_eq!(f.format(10), "10,00 ");
    /// assert_eq!(f.format(100), "100,0 ");
    /// assert_eq!(f.format(999), "999,0 ");
    /// assert_eq!(f.format(1000), "1,000 k");
    /// assert_eq!(f.format(1e3), "1,000 k");
    /// assert_eq!(f.format(1e6), "1,000 M");
    /// assert_eq!(f.format(1e9), "1,000 G");
    /// assert_eq!(f.format(1e12), "1,000 T");
    /// assert_eq!(f.format(1e15), "1,000 P");
    /// assert_eq!(f.format(1e18), "1,000 E");
    /// assert_eq!(f.format(1e21), "1,000 Z");
    /// assert_eq!(f.format(1e24), "1,000 Y");
    /// assert_eq!(f.format(1e27), "1,000 R");
    /// assert_eq!(f.format(1e30), "1,000 Q");
    /// assert_eq!(f.format(1e33), "1,000 * 10^(33) ");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::Decimal(true));
    /// assert_eq!(f.format(-1e-31), "-1,000 * 10^(-31) ");
    /// assert_eq!(f.format(-1e-30), "-1,000 q");
    /// assert_eq!(f.format(-1e-27), "-1,000 r");
    /// assert_eq!(f.format(-1e-24), "-1,000 y");
    /// assert_eq!(f.format(-1e-21), "-1,000 z");
    /// assert_eq!(f.format(-1e-18), "-1,000 a");
    /// assert_eq!(f.format(-1e-15), "-1,000 f");
    /// assert_eq!(f.format(-1e-12), "-1,000 p");
    /// assert_eq!(f.format(-1e-9), "-1,000 n");
    /// assert_eq!(f.format(-1e-6), "-1,000 µ");
    /// assert_eq!(f.format(-1e-3), "-1,000 m");
    /// assert_eq!(f.format(-1), "-1,000 ");
    /// assert_eq!(f.format(-10), "-10,00 ");
    /// assert_eq!(f.format(-100), "-100,0 ");
    /// assert_eq!(f.format(-999), "-999,0 ");
    /// assert_eq!(f.format(-1000), "-1,000 k");
    /// assert_eq!(f.format(-1e3), "-1,000 k");
    /// assert_eq!(f.format(-1e6), "-1,000 M");
    /// assert_eq!(f.format(-1e9), "-1,000 G");
    /// assert_eq!(f.format(-1e12), "-1,000 T");
    /// assert_eq!(f.format(-1e15), "-1,000 P");
    /// assert_eq!(f.format(-1e18), "-1,000 E");
    /// assert_eq!(f.format(-1e21), "-1,000 Z");
    /// assert_eq!(f.format(-1e24), "-1,000 Y");
    /// assert_eq!(f.format(-1e27), "-1,000 R");
    /// assert_eq!(f.format(-1e30), "-1,000 Q");
    /// assert_eq!(f.format(-1e33), "-1,000 * 10^(33) ");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::Decimal(false));
    /// assert_eq!(f.format(1000), "1,000k");
    /// ```
    ///
    /// ## None
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None);
    /// assert_eq!(f.format(1e-10), "0,0000000001000");
    /// assert_eq!(f.format(0.1), "0,1000");
    /// assert_eq!(f.format(1), "1,000");
    /// assert_eq!(f.format(10), "10,00");
    /// assert_eq!(f.format(100), "100,0");
    /// assert_eq!(f.format(1000), "1.000");
    /// assert_eq!(f.format(1e10), "10.000.000.000");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None);
    /// assert_eq!(f.format(-1e-10), "-0,0000000001000");
    /// assert_eq!(f.format(-0.1), "-0,1000");
    /// assert_eq!(f.format(-1), "-1,000");
    /// assert_eq!(f.format(-10), "-10,00");
    /// assert_eq!(f.format(-100), "-100,0");
    /// assert_eq!(f.format(-1000), "-1.000");
    /// assert_eq!(f.format(-1e10), "-10.000.000.000");
    /// ```
    ///
    /// ## Scientific
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::Scientific);
    /// assert_eq!(f.format(0.01), "1,000 * 10^(-2)");
    /// assert_eq!(f.format(0.1), "1,000 * 10^(-1)");
    /// assert_eq!(f.format(1), "1,000 * 10^(0)");
    /// assert_eq!(f.format(10), "1,000 * 10^(1)");
    /// assert_eq!(f.format(100), "1,000 * 10^(2)");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::Scientific);
    /// assert_eq!(f.format(-0.01), "-1,000 * 10^(-2)");
    /// assert_eq!(f.format(-0.1), "-1,000 * 10^(-1)");
    /// assert_eq!(f.format(-1), "-1,000 * 10^(0)");
    /// assert_eq!(f.format(-10), "-1,000 * 10^(1)");
    /// assert_eq!(f.format(-100), "-1,000 * 10^(2)");
    /// ```
    pub fn set_scaling(mut self, scaling: Scaling) -> Self
    {
        self.scaling = scaling;
        return self;
    }


    /// # Summary
    /// Sets the 1000 group and decimal separator. Warns via `log::warn!` if decimal separator is empty, if they are the same, or if they contain digits.
    ///
    /// # Arguments
    /// - `group_separator`
    ///     - Separates groups every 3 digits before the decimal separator.
    /// - `decimal_separator`
    ///     - Separates the integer and fractional parts of a number.
    ///
    /// # Returns
    /// - modified self
    ///
    /// # Examples
    ///
    /// Examples have scaling disabled for easier understanding.
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_separators(".", ",");
    /// assert_eq!(f.format(1), "1,000");
    /// assert_eq!(f.format(10), "10,00");
    /// assert_eq!(f.format(100), "100,0");
    /// assert_eq!(f.format(1000), "1.000");
    /// assert_eq!(f.format(10000), "10.000");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_separators("", ",");
    /// assert_eq!(f.format(1), "1,000");
    /// assert_eq!(f.format(10), "10,00");
    /// assert_eq!(f.format(100), "100,0");
    /// assert_eq!(f.format(1000), "1000");
    /// assert_eq!(f.format(10000), "10000");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_scaling(scaler::Scaling::None)
    ///    .set_separators(",", ".");
    /// assert_eq!(f.format(1), "1.000");
    /// assert_eq!(f.format(10), "10.00");
    /// assert_eq!(f.format(100), "100.0");
    /// assert_eq!(f.format(1000), "1,000");
    /// assert_eq!(f.format(10000), "10,000");
    /// ```
    pub fn set_separators(mut self, group_separator: &str, decimal_separator: &str) -> Self
    {
        #[cfg(feature = "warn_about_problematic_separators")] // warn if feature is enabled
        {
            if decimal_separator == ""
            {
                log::warn!("Decimal separator is empty. This may lead to ambiguous formatting.");
            }
            else if group_separator == decimal_separator
            {
                log::warn!(
                    "Group separator \"{}\" and decimal separator \"{}\" are the same. This may lead to ambiguous formatting.",
                    group_separator,
                    decimal_separator
                );
            }
            else if group_separator.contains("0")
                || group_separator.contains("1")
                || group_separator.contains("2")
                || group_separator.contains("3")
                || group_separator.contains("4")
                || group_separator.contains("5")
                || group_separator.contains("6")
                || group_separator.contains("7")
                || group_separator.contains("8")
                || group_separator.contains("9")
            {
                log::warn!("Group separator \"{}\" contains a digit. This may lead to ambiguous formatting.", group_separator);
            }
            else if decimal_separator.contains("0")
                || decimal_separator.contains("1")
                || decimal_separator.contains("2")
                || decimal_separator.contains("3")
                || decimal_separator.contains("4")
                || decimal_separator.contains("5")
                || decimal_separator.contains("6")
                || decimal_separator.contains("7")
                || decimal_separator.contains("8")
                || decimal_separator.contains("9")
            {
                log::warn!("Decimal separator \"{}\" contains a digit. This may lead to ambiguous formatting.", decimal_separator);
            }
        }

        self.group_separator = group_separator.to_string();
        self.decimal_separator = decimal_separator.to_string();

        return self;
    }


    /// # Summary
    /// Sets the sign mode.
    ///
    /// # Arguments
    /// - `sign`: new sign mode
    ///     - Always: Always show sign, even when number is positive.
    ///     - OnlyMinus: Only show sign when number is negative.
    ///
    /// # Returns
    /// - modified self
    ///
    /// # Examples
    /// ## Always
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_sign(scaler::Sign::Always);
    /// assert_eq!(f.format(std::f64::NEG_INFINITY), "-∞ ");
    /// assert_eq!(f.format(-1), "-1,000 ");
    /// assert_eq!(f.format(0), "+0,000 ");
    /// assert_eq!(f.format(1), "+1,000 ");
    /// assert_eq!(f.format(std::f64::INFINITY), "+∞ ");
    /// ```
    ///
    /// ## OnlyMinus
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///    .set_sign(scaler::Sign::OnlyMinus);
    /// assert_eq!(f.format(std::f64::NEG_INFINITY), "-∞ ");
    /// assert_eq!(f.format(-1), "-1,000 ");
    /// assert_eq!(f.format(0), "0,000 ");
    /// assert_eq!(f.format(1), "1,000 ");
    /// assert_eq!(f.format(std::f64::INFINITY), "∞ ");
    /// ```
    pub fn set_sign(mut self, sign: Sign) -> Self
    {
        self.sign = sign;
        return self;
    }


    /// # Summary
    /// Sets whether or not to display trailing zeros.
    ///
    /// # Arguments
    /// - `trailing_zeros`: whether or not to display trailing zeros
    ///
    /// # Returns
    /// - modified self
    ///
    /// # Examples
    /// ## true
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::Binary(true))
    ///     .set_trailing_zeros(true);
    /// assert_eq!(f.format(1), "1,000 ");
    /// assert_eq!(f.format(1.2), "1,200 ");
    /// assert_eq!(f.format(1.23), "1,230 ");
    /// assert_eq!(f.format(1.234), "1,234 ");
    /// assert_eq!(f.format(1.2345), "1,234 ");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::Decimal(true))
    ///     .set_trailing_zeros(true);
    /// assert_eq!(f.format(1), "1,000 ");
    /// assert_eq!(f.format(1.2), "1,200 ");
    /// assert_eq!(f.format(1.23), "1,230 ");
    /// assert_eq!(f.format(1.234), "1,234 ");
    /// assert_eq!(f.format(1.2345), "1,234 ");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::None)
    ///     .set_trailing_zeros(true);
    /// assert_eq!(f.format(1), "1,000");
    /// assert_eq!(f.format(1.2), "1,200");
    /// assert_eq!(f.format(1.23), "1,230");
    /// assert_eq!(f.format(1.234), "1,234");
    /// assert_eq!(f.format(1.2345), "1,234");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::Scientific)
    ///     .set_trailing_zeros(true);
    /// assert_eq!(f.format(1), "1,000 * 10^(0)");
    /// assert_eq!(f.format(1.2), "1,200 * 10^(0)");
    /// assert_eq!(f.format(1.23), "1,230 * 10^(0)");
    /// assert_eq!(f.format(1.234), "1,234 * 10^(0)");
    /// assert_eq!(f.format(1.2345), "1,234 * 10^(0)");
    /// ```
    ///
    /// ## false
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::Binary(true))
    ///     .set_trailing_zeros(false);
    /// assert_eq!(f.format(1), "1 ");
    /// assert_eq!(f.format(1.2), "1,2 ");
    /// assert_eq!(f.format(1.23), "1,23 ");
    /// assert_eq!(f.format(1.234), "1,234 ");
    /// assert_eq!(f.format(1.2345), "1,234 ");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::Decimal(true))
    ///     .set_trailing_zeros(false);
    /// assert_eq!(f.format(1), "1 ");
    /// assert_eq!(f.format(1.2), "1,2 ");
    /// assert_eq!(f.format(1.23), "1,23 ");
    /// assert_eq!(f.format(1.234), "1,234 ");
    /// assert_eq!(f.format(1.2345), "1,234 ");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::None)
    ///     .set_trailing_zeros(false);
    /// assert_eq!(f.format(1), "1");
    /// assert_eq!(f.format(1.2), "1,2");
    /// assert_eq!(f.format(1.23), "1,23");
    /// assert_eq!(f.format(1.234), "1,234");
    /// assert_eq!(f.format(1.2345), "1,234");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::Scientific)
    ///     .set_trailing_zeros(false);
    /// assert_eq!(f.format(1), "1 * 10^(0)");
    /// assert_eq!(f.format(1.2), "1,2 * 10^(0)");
    /// assert_eq!(f.format(1.23), "1,23 * 10^(0)");
    /// assert_eq!(f.format(1.234), "1,234 * 10^(0)");
    /// assert_eq!(f.format(1.2345), "1,234 * 10^(0)");
    /// ```
    pub fn set_trailing_zeros(mut self, trailing_zeros: bool) -> Self
    {
        self.trailing_zeros = trailing_zeros;
        return self;
    }
}


impl Default for Formatter
{
    /// # Summary
    /// Constructs default Formatter with only sign when negative, decimal scaling, rounding to 4 significant digits, "." as thousand separator, and "," as decimal separator.
    ///
    /// # Returns
    /// - default Formatter
    fn default() -> Self
    {
        return Self::new();
    }
}


// inspiration: https://github.com/kurtlawrence/numfmt/
