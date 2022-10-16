use std::collections::HashMap;

pub fn convert_tz(tz: &str) -> &str {
    let mut map = HashMap::new();
    map.insert("Dateline Standard Time", "Etc/GMT+12");
    map.insert("UTC-11", "Etc/GMT+11");
    map.insert("Aleutian Standard Time", "America/Adak");
    map.insert("Hawaiian Standard Time", "Pacific/Honolulu");
    map.insert("Marquesas Standard Time", "Pacific/Marquesas");
    map.insert("Alaskan Standard Time", "America/Anchorage");
    map.insert("UTC-09", "Etc/GMT+9");
    map.insert("Pacific Standard Time (Mexico)", "America/Tijuana");
    map.insert("UTC-08", "Etc/GMT+8");
    map.insert("Pacific Standard Time", "America/Los_Angeles");
    map.insert("US Mountain Standard Time", "America/Phoenix");
    map.insert("Mountain Standard Time (Mexico)", "America/Chihuahua");
    map.insert("Mountain Standard Time", "America/Denver");
    map.insert("Yukon Standard Time", "America/Whitehorse");
    map.insert("Central America Standard Time", "America/Guatemala");
    map.insert("Central Standard Time", "America/Chicago");
    map.insert("Easter Island Standard Time", "Pacific/Easter");
    map.insert("Central Standard Time (Mexico)", "America/Mexico_City");
    map.insert("Canada Central Standard Time", "America/Regina");
    map.insert("SA Pacific Standard Time", "America/Bogota");
    map.insert("Eastern Standard Time (Mexico)", "America/Cancun");
    map.insert("Eastern Standard Time", "America/New_York");
    map.insert("Haiti Standard Time", "America/Port-au-Prince");
    map.insert("Cuba Standard Time", "America/Havana");
    map.insert("US Eastern Standard Time", "America/Indianapolis");
    map.insert("Turks And Caicos Standard Time", "America/Grand_Turk");
    map.insert("Paraguay Standard Time", "America/Asuncion");
    map.insert("Atlantic Standard Time", "America/Halifax");
    map.insert("Venezuela Standard Time", "America/Caracas");
    map.insert("Central Brazilian Standard Time", "America/Cuiaba");
    map.insert("SA Western Standard Time", "America/La_Paz");
    map.insert("Pacific SA Standard Time", "America/Santiago");
    map.insert("Newfoundland Standard Time", "America/St_Johns");
    map.insert("Tocantins Standard Time", "America/Araguaina");
    map.insert("E. South America Standard Time", "America/Sao_Paulo");
    map.insert("SA Eastern Standard Time", "America/Cayenne");
    map.insert("Argentina Standard Time", "America/Buenos_Aires");
    map.insert("Greenland Standard Time", "America/Godthab");
    map.insert("Montevideo Standard Time", "America/Montevideo");
    map.insert("Magallanes Standard Time", "America/Punta_Arenas");
    map.insert("Saint Pierre Standard Time", "America/Miquelon");
    map.insert("Bahia Standard Time", "America/Bahia");
    map.insert("UTC-02", "Etc/GMT+2");
    map.insert("Azores Standard Time", "Atlantic/Azores");
    map.insert("Cape Verde Standard Time", "Atlantic/Cape_Verde");
    map.insert("UTC", "Etc/UTC");
    map.insert("GMT Standard Time", "Europe/London");
    map.insert("Greenwich Standard Time", "Atlantic/Reykjavik");
    map.insert("Sao Tome Standard Time", "Africa/Sao_Tome");
    map.insert("Morocco Standard Time", "Africa/Casablanca");
    map.insert("W. Europe Standard Time", "Europe/Berlin");
    map.insert("Central Europe Standard Time", "Europe/Budapest");
    map.insert("Romance Standard Time", "Europe/Paris");
    map.insert("Central European Standard Time", "Europe/Warsaw");
    map.insert("W. Central Africa Standard Time", "Africa/Lagos");
    map.insert("Jordan Standard Time", "Asia/Amman");
    map.insert("GTB Standard Time", "Europe/Bucharest");
    map.insert("Middle East Standard Time", "Asia/Beirut");
    map.insert("Egypt Standard Time", "Africa/Cairo");
    map.insert("E. Europe Standard Time", "Europe/Chisinau");
    map.insert("Syria Standard Time", "Asia/Damascus");
    map.insert("West Bank Standard Time", "Asia/Hebron");
    map.insert("South Africa Standard Time", "Africa/Johannesburg");
    map.insert("FLE Standard Time", "Europe/Kiev");
    map.insert("Israel Standard Time", "Asia/Jerusalem");
    map.insert("South Sudan Standard Time", "Africa/Juba");
    map.insert("Kaliningrad Standard Time", "Europe/Kaliningrad");
    map.insert("Sudan Standard Time", "Africa/Khartoum");
    map.insert("Libya Standard Time", "Africa/Tripoli");
    map.insert("Namibia Standard Time", "Africa/Windhoek");
    map.insert("Arabic Standard Time", "Asia/Baghdad");
    map.insert("Turkey Standard Time", "Europe/Istanbul");
    map.insert("Arab Standard Time", "Asia/Riyadh");
    map.insert("Belarus Standard Time", "Europe/Minsk");
    map.insert("Russian Standard Time", "Europe/Moscow");
    map.insert("E. Africa Standard Time", "Africa/Nairobi");
    map.insert("Iran Standard Time", "Asia/Tehran");
    map.insert("Arabian Standard Time", "Asia/Dubai");
    map.insert("Astrakhan Standard Time", "Europe/Astrakhan");
    map.insert("Azerbaijan Standard Time", "Asia/Baku");
    map.insert("Russia Time Zone 3", "Europe/Samara");
    map.insert("Mauritius Standard Time", "Indian/Mauritius");
    map.insert("Saratov Standard Time", "Europe/Saratov");
    map.insert("Georgian Standard Time", "Asia/Tbilisi");
    map.insert("Volgograd Standard Time", "Europe/Volgograd");
    map.insert("Caucasus Standard Time", "Asia/Yerevan");
    map.insert("Afghanistan Standard Time", "Asia/Kabul");
    map.insert("West Asia Standard Time", "Asia/Tashkent");
    map.insert("Ekaterinburg Standard Time", "Asia/Yekaterinburg");
    map.insert("Pakistan Standard Time", "Asia/Karachi");
    map.insert("Qyzylorda Standard Time", "Asia/Qyzylorda");
    map.insert("India Standard Time", "Asia/Calcutta");
    map.insert("Sri Lanka Standard Time", "Asia/Colombo");
    map.insert("Nepal Standard Time", "Asia/Katmandu");
    map.insert("Central Asia Standard Time", "Asia/Almaty");
    map.insert("Bangladesh Standard Time", "Asia/Dhaka");
    map.insert("Omsk Standard Time", "Asia/Omsk");
    map.insert("Myanmar Standard Time", "Asia/Rangoon");
    map.insert("SE Asia Standard Time", "Asia/Bangkok");
    map.insert("Altai Standard Time", "Asia/Barnaul");
    map.insert("W. Mongolia Standard Time", "Asia/Hovd");
    map.insert("North Asia Standard Time", "Asia/Krasnoyarsk");
    map.insert("N. Central Asia Standard Time", "Asia/Novosibirsk");
    map.insert("Tomsk Standard Time", "Asia/Tomsk");
    map.insert("China Standard Time", "Asia/Shanghai");
    map.insert("North Asia East Standard Time", "Asia/Irkutsk");
    map.insert("Singapore Standard Time", "Asia/Singapore");
    map.insert("W. Australia Standard Time", "Australia/Perth");
    map.insert("Taipei Standard Time", "Asia/Taipei");
    map.insert("Ulaanbaatar Standard Time", "Asia/Ulaanbaatar");
    map.insert("Aus Central W. Standard Time", "Australia/Eucla");
    map.insert("Transbaikal Standard Time", "Asia/Chita");
    map.insert("Tokyo Standard Time", "Asia/Tokyo");
    map.insert("North Korea Standard Time", "Asia/Pyongyang");
    map.insert("Korea Standard Time", "Asia/Seoul");
    map.insert("Yakutsk Standard Time", "Asia/Yakutsk");
    map.insert("Cen. Australia Standard Time", "Australia/Adelaide");
    map.insert("AUS Central Standard Time", "Australia/Darwin");
    map.insert("E. Australia Standard Time", "Australia/Brisbane");
    map.insert("AUS Eastern Standard Time", "Australia/Sydney");
    map.insert("West Pacific Standard Time", "Pacific/Port_Moresby");
    map.insert("Tasmania Standard Time", "Australia/Hobart");
    map.insert("Vladivostok Standard Time", "Asia/Vladivostok");
    map.insert("Lord Howe Standard Time", "Australia/Lord_Howe");
    map.insert("Bougainville Standard Time", "Pacific/Bougainville");
    map.insert("Russia Time Zone 10", "Asia/Srednekolymsk");
    map.insert("Magadan Standard Time", "Asia/Magadan");
    map.insert("Norfolk Standard Time", "Pacific/Norfolk");
    map.insert("Sakhalin Standard Time", "Asia/Sakhalin");
    map.insert("Central Pacific Standard Time", "Pacific/Guadalcanal");
    map.insert("Russia Time Zone 11", "Asia/Kamchatka");
    map.insert("New Zealand Standard Time", "Pacific/Auckland");
    map.insert("UTC+12", "Etc/GMT-12");
    map.insert("Fiji Standard Time", "Pacific/Fiji");
    map.insert("Chatham Islands Standard Time", "Pacific/Chatham");
    map.insert("UTC+13", "Etc/GMT-13");
    map.insert("Tonga Standard Time", "Pacific/Tongatapu");
    map.insert("Samoa Standard Time", "Pacific/Apia");
    map.insert("Line Islands Standard Time", "Pacific/Kiritimati");
    if !map.contains_key(tz) {
        tz
    } else {
        map.get(tz).unwrap()
    }
}
#[cfg(test)]
mod convertion_tz {
    use crate::tz_checker::unix_tz_converter::convert_tz;

    #[test]
    fn samoa_to_pacific() {
        assert_eq!(convert_tz("Samoa Standard Time"), "Pacific/Apia");
    }
    #[test]
    fn eastern_to_ny() {
        assert_eq!(convert_tz("Eastern Standard Time"), "America/New_York");
    }

    #[test]
    fn easternmx_to_cancun() {
        assert_eq!(
            convert_tz("Eastern Standard Time (Mexico)"),
            "America/Cancun"
        );
    }
    // map.insert("Eastern Standard Time (Mexico)", "America/Cancun");
    // map.insert("Eastern Standard Time", "America/New_York");
}
