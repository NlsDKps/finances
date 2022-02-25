use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Frequency of a cash flow.
/// Typically a cash flow will either occur monthly or yearly. But also part yearly cash flows
/// can occur, e.g. if you pay insurance every 6 months.
#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize)]
pub enum Frequency {
    Monthly(i8),
    Yearly(i8),
}

/// Cash flow frequency object. Holding the date of the occurance - some occur at the beginning of a 
/// month (01.01.) some in the middle of a month (15.01.). For cash flows, that occur only every few
/// months, the date has a better meaning.
#[derive(Deserialize, Serialize)]
pub struct CashFlowFrequency {
    #[serde(with = "date_format")]
    pub date: NaiveDate,
    pub frequency: Frequency,
}

impl CashFlowFrequency {
    pub(crate) fn example() -> Self {
        CashFlowFrequency {
            date: NaiveDate::from_ymd(1987, 10, 19),
            frequency: Frequency::Yearly(42)
        }
    }
}

/// Allows to (de-)serialize a NaiveDate into a JSON string.
mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Format of a encoded string (YYYY-MM-DD)
    const FORMAT: &'static str = "%Y-%m-%d";

    /// Serializes a NaiveDate into a string (format see FORMAT)
    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    /// Deserializes a string into a NaiveDate object.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cash_flow_frequency_sets_date() {
        let date = NaiveDate::from_ymd(2022, 02, 25);
        let cash_flow_freq = CashFlowFrequency {
            date,
            frequency: Frequency::Monthly(1)
        };
        assert_eq!(date, cash_flow_freq.date)
    }

    #[test]
    fn test_cash_flow_frequency_sets_frequency() {
        let frequency = Frequency::Monthly(3);
        let cash_flow_frequency = CashFlowFrequency {
            date: NaiveDate::from_ymd(2022, 02, 25),
            frequency
        };
        assert_eq!(frequency, cash_flow_frequency.frequency)
    }

    #[test]
    fn test_cash_flow_frequency_date_can_be_formatted() {
        let date = NaiveDate::from_ymd(2022, 02, 25);
        let frequency = Frequency::Monthly(3);
        let cash_flow_frequency = CashFlowFrequency {
            date, frequency
        };
        let json = serde_json::to_string(&cash_flow_frequency).unwrap();
        assert_eq!(json, "{\"date\":\"2022-02-25\",\"frequency\":{\"Monthly\":3}}");
    }

    #[test]
    fn test_cash_flow_frequency_can_be_parsed() {
        let str_json = "{\"date\":\"2022-02-25\",\"frequency\":{\"Monthly\":3}}";
        let cash_flow_frequency : CashFlowFrequency = serde_json::from_str(str_json).unwrap();
        assert_eq!(cash_flow_frequency.frequency, Frequency::Monthly(3))
    }
}