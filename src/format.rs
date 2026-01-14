// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::*;


impl Formatter
{
    /// # Summary
    /// Intermediate collection of formatting options to then scale, round, and display numbers.
    ///
    /// # Arguments
    /// - `x`: the number to format
    ///     - must be copy convertable to f64, from into expects lossless conversion
    ///     - lossy conversion must be explicitly handled by caller beforehand to avoid unexpected behaviour
    ///
    /// # Returns
    /// - the formatted number
    ///
    /// # Examples
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new(); // calculation results
    /// assert_eq!(f.format(123.456), "123,5 ");
    /// assert_eq!(f.format(0.789), "789,0 m");
    /// assert_eq!(f.format(42069), "42,07 k");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_rounding(scaler::Rounding::SignificantDigits(3)); // general display
    /// assert_eq!(f.format(123.456), "123 ");
    /// assert_eq!(f.format(0.789), "789 m");
    /// assert_eq!(f.format(42069), "42,1 k");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::None)
    ///     .set_rounding(scaler::Rounding::Magnitude(0)); // absolute values
    /// assert_eq!(f.format(123.456), "123");
    /// assert_eq!(f.format(0.789), "1");
    /// assert_eq!(f.format(42069), "42.069");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new()
    ///     .set_scaling(scaler::Scaling::Binary(true))
    ///     .set_rounding(scaler::Rounding::SignificantDigits(3)); // data sizes
    /// assert_eq!(f.format(123.456), "123 ");
    /// assert_eq!(f.format(0.789), "1,58 * 2^(-1) ");
    /// assert_eq!(f.format(42069), "41,1 Ki");
    /// ```
    ///
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new(); // edge cases
    /// assert_eq!(f.format(f64::NEG_INFINITY), "-∞ ");
    /// assert_eq!(f.format(f64::INFINITY), "∞ ");
    /// assert_eq!(f.format(f64::NAN), "NaN ");
    /// ```
    pub fn format<T>(&self, x: T) -> String
    where
        T: Clone + Into<f64>, // T must be copy convertable to f64
    {
        const BINARY_PREFIXES: [(i16, i16, &str); 9] = [
            (0, 10, ""),
            (10, 20, "Ki"),
            (20, 30, "Mi"),
            (30, 40, "Gi"),
            (40, 50, "Ti"),
            (50, 60, "Pi"),
            (60, 70, "Ei"),
            (70, 80, "Zi"),
            (80, 90, "Yi"),
        ]; // unit prefixes for binary mode, [lower bound magnitude; upper bound magnitude[, unit prefix
        const DECIMAL_PREFIXES: [(i16, i16, &str); 21] = [
            (-30, -27, "q"),
            (-27, -24, "r"),
            (-24, -21, "y"),
            (-21, -18, "z"),
            (-18, -15, "a"),
            (-15, -12, "f"),
            (-12, -9, "p"),
            (-9, -6, "n"),
            (-6, -3, "µ"),
            (-3, 0, "m"),
            (0, 3, ""),
            (3, 6, "k"),
            (6, 9, "M"),
            (9, 12, "G"),
            (12, 15, "T"),
            (15, 18, "P"),
            (18, 21, "E"),
            (21, 24, "Z"),
            (24, 27, "Y"),
            (27, 30, "R"),
            (30, 33, "Q"),
        ]; // SI unit prefixes for decimal mode, [lower bound magnitude; upper bound magnitude[, unit prefix
        let mut dec_places: i16; // number of decimal places to use, i16 instead of u16 to allow negative values during intermediate steps
        let magnitude: f64; // magnitude of the number, decimal 10^magnitude or binary 2^magnitude, exact f64 instead of floored i16 to enable scaling binary with rounding significant digits correctly when number is [1.000; 1.024[
        let mut s: String; // formatted number string, result


        let mut x: f64 = x.into(); // &T -> f64
        if x.is_infinite() && x.is_sign_positive() // edge cases
        {
            s = "∞".to_string(); // positive infinity
            if self.sign == Sign::Always
            // if always sign
            {
                s = format!("+{s}"); // manually add plus sign
            }
            if self.scaling == Scaling::Binary(true) || self.scaling == Scaling::Decimal(true) // if scaling enabled with whitespace between number and unit
            {
                s = format!("{s} "); // manually add whitespace
            }
            return s;
        }
        else if x.is_infinite() && x.is_sign_negative()
        {
            s = "-∞".to_string(); // negative infinity
            if self.scaling == Scaling::Binary(true) || self.scaling == Scaling::Decimal(true) // if scaling enabled with whitespace between number and unit
            {
                s = format!("{s} "); // manually add whitespace
            }
            return s;
        }
        else if x.is_nan()
        {
            s = "NaN".to_string(); // not a number
            if self.scaling == Scaling::Binary(true) || self.scaling == Scaling::Decimal(true) // if scaling enabled with whitespace between number and unit
            {
                s = format!("{s} "); // manually add whitespace
            }
            return s;
        }


        x = match self.rounding // rounded here already in case rounding changes magnitude
        {
            Rounding::Magnitude(precision) => x.round_mag(precision), // round statically to digit at 10^magnitude
            Rounding::SignificantDigits(precision) => x.round_sig(precision), // round dynamically to significant numbers
        };

        if x == 0.0
        {
            magnitude = 0.0; // 0 has default magnitude and no unit prefix, here because log(0) would shit itself
        }
        else
        {
            magnitude = match self.scaling // determine magnitude with rounded value in case rounding changes magnitude
            {
                Scaling::Binary(_) => x.abs().log2(), // if scaling binary: binary magnitude 2^magnitude
                _ => x.abs().log10(), // usually: decimal magnitude 10^magnitude
            }
        }

        dec_places = match (&self.scaling, &self.rounding) // decimal places required depending on scaling and rounding mode
        {
            (Scaling::Binary(_), Rounding::Magnitude(precision)) => // convert binary magnitude to decimal magnitude, exact f64 magnitude required for this case, then business as usual
            {
                match BINARY_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find binary unit prefix for magnitude
                {
                    Some(_) =>{2.0_f64.powf(magnitude - magnitude.rem_euclid(10.0)).log10().floor() as i16 - precision - 1},
                    None => {magnitude.floor() as i16} // fallback to scientific notation
                }
            },
            (Scaling::Binary(_), Rounding::SignificantDigits(precision)) => // convert binary magnitude to decimal magnitude, exact f64 magnitude required for this case, then business as usual
            {
                match BINARY_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find binary unit prefix for magnitude
                {
                    Some(_) =>{-1 * (2.0_f64.powf(magnitude.rem_euclid(10.0)).log10().floor()) as i16 + *precision as i16 - 1},
                    None => {*precision as i16 - 1} // fallback to scientific notation
                }
            }
            (Scaling::Decimal(_), Rounding::Magnitude(precision)) =>
            {
                match DECIMAL_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find decimal unit prefix for magnitude
                {
                    Some(_) =>{(magnitude - magnitude.rem_euclid(3.0)).floor() as i16 - precision},
                    None => {magnitude.floor() as i16} // fallback to scientific notation
                }
            },
            (Scaling::Decimal(_), Rounding::SignificantDigits(precision)) =>
            {
                match DECIMAL_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find decimal unit prefix for magnitude
                {
                    Some(_) =>{-1 * magnitude.rem_euclid(3.0).floor() as i16+ *precision as i16 - 1},
                    None => {*precision as i16 - 1} // fallback to scientific notation
                }
            }
            (Scaling::None, Rounding::Magnitude(precision)) => -1 * precision,
            (Scaling::None, Rounding::SignificantDigits(precision)) => -1 * magnitude.floor() as i16 + *precision as i16 - 1,
            (Scaling::Scientific, Rounding::Magnitude(_)) => magnitude.floor() as i16,
            (Scaling::Scientific, Rounding::SignificantDigits(precision)) => *precision as i16 - 1,
        };
        if dec_places < 0
        {
            dec_places = 0; // negative number of decimal places are not allowed
        }

        match self.scaling //  apply magnitude shift for scaling, f64 -> String, optionally remove trailing zeros, append unit prefix
        {
            Scaling::None => // no scaling
            {
                s = format!("{:.*}", dec_places as usize, x);
                if !self.trailing_zeros {s = s.trim_end_matches("0").trim_end_matches(".").to_string();} // remove trailing zeros and bare decimal separator
            }
            Scaling::Binary(whitespace_separation) => // binary scaling
            {
                match BINARY_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find binary unit prefix for magnitude
                {
                    Some((_lower, _upper, prefix)) =>
                    {
                        s = format!("{:.*}", dec_places as usize, x / 2.0_f64.powf(magnitude - magnitude.rem_euclid(10.0))); // divide by 2^magnitude
                        if !self.trailing_zeros {s = s.trim_end_matches("0").trim_end_matches(".").to_string();} // remove trailing zeros and bare decimal separator
                        if whitespace_separation {s += " ";} // add whitespace between number and unit prefix
                        s += prefix; // append binary unit prefix
                    },
                    None => // fallback to base 2 scientific notation
                    {
                        s = format!("{:.*}", dec_places as usize, x / 2.0_f64.powf(magnitude.floor())); // divide by 2^magnitude
                        if !self.trailing_zeros {s = s.trim_end_matches("0").trim_end_matches(".").to_string();} // remove trailing zeros and bare decimal separator
                        s += format!(" * 2^({}) ", magnitude.floor()).as_str();  // append base 2 multiplier and whitespace for commonality with non-fallback case
                    }
                }
            }
            Scaling::Decimal(whitespace_separation) => // decimal scaling
            {
                match DECIMAL_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find decimal unit prefix for magnitude
                {
                    Some((_lower, _upper, prefix)) =>
                    {
                        s = format!("{:.*}", dec_places as usize, x / 10.0_f64.powf(magnitude - magnitude.rem_euclid(3.0))); // divide by 10^magnitude
                        if !self.trailing_zeros {s = s.trim_end_matches("0").trim_end_matches(".").to_string();} // remove trailing zeros and bare decimal separator
                        if whitespace_separation {s += " ";} // add whitespace between number and unit prefix
                        s += prefix; // append decimal unit prefix
                    },
                    None => // fallback to base 10 scientific notation
                    {
                        s = format!("{:.*}", dec_places as usize, x / 10.0_f64.powf(magnitude.floor())); // divide by 10^magnitude
                        if !self.trailing_zeros {s = s.trim_end_matches("0").trim_end_matches(".").to_string();} // remove trailing zeros and bare decimal separator
                        s += format!(" * 10^({}) ", magnitude.floor()).as_str(); // append base 10 multiplier and whitespace for commonality with non-fallback case
                    }
                }
            }
            Scaling::Scientific => // scientific notation
            {
                s = format!("{:.*}", dec_places as usize, x / 10.0_f64.powf(magnitude.floor())); // divide by 10^magnitude
                if !self.trailing_zeros {s = s.trim_end_matches("0").trim_end_matches(".").to_string();} // remove trailing zeros and bare decimal separator
                s += format!(" * 10^({})", magnitude.floor()).as_str(); // append base 10 multiplier, no whitespace
            }
        }

        if self.sign == Sign::Always && x.is_sign_positive()
        // if always sign and positive
        {
            s = format!("+{s}"); // manually add plus sign
        }

        if self.group_separator != ""
        // add thousands separators markers, done here already with default decimal separator "." in case user defined empty or otherwise trolling decimal separator
        {
            let group_separator_i_earliest: usize = s
                .chars()
                .position(|c| c.is_digit(10))
                .expect(format!("Could not find first digit in `s` = \"{s}\", formerly `x` = \"{x}\".").as_str())
                + 1; // earliest possible index of group separator, find first digit
            let mut i: usize = s
                .find(".") // find default decimal separator
                .or_else(|| s.find("*")) // if none assume single digit scientific notation and start at space before multiplication sign, will result in no group separators
                .or_else(|| s.chars().rev().position(|c| c.is_digit(10)).map(|pos| s.len() - pos)) // if none assume no decimal separator and no scientific notation and start at last digit
                .expect(format!("Could not find last digit in `s` = \"{s}\", formerly `x` = \"{x}\".").as_str());

            while group_separator_i_earliest + 3 <= i
            // insert group separators
            {
                i -= 3; // move to previous group of 3 digits
                s.insert_str(i, "{GROUP SEPARATOR}"); // insert group separator marker, not actual separator to avoid confusion with decimal separator replacement
            }
        }
        s = s.replace(".", self.decimal_separator.to_string().as_str()); // replace decimal separator
        s = s.replace("{GROUP SEPARATOR}", self.group_separator.to_string().as_str()); // replace group separator

        return s;
    }
}
