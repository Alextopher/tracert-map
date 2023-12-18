/// Parses a string that looks like a traceroute / tracert output and extracts
/// the ip addresses.
pub fn parse_trace(trace: &str) -> Vec<&str> {
    // ipv4 address regex
    let re = regex::Regex::new(r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();

    let ips: Vec<&str> = trace
        .lines()
        .flat_map(|line| re.find(line))
        .map(|ip| ip.as_str())
        .collect();

    // If the first ip address is the last ip address, remove it
    if ips.first() == ips.last() {
        ips[1..].to_vec()
    } else {
        ips
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_trace() {
        let trace = r#"traceroute to ox.ac.uk (151.101.66.216), 30 hops max, 60 byte packets
        1  _gateway (192.168.1.1)  2.400 ms  2.376 ms  2.373 ms
        2  142-254-211-185.inf.spectrum.com (142.254.211.185)  16.898 ms  20.250 ms  20.248 ms
        3  lag-61.troynydv01h.netops.charter.com (24.58.201.49)  31.687 ms  31.685 ms  33.625 ms
        4  lag-34.albynyyf02r.netops.charter.com (24.58.35.16)  20.243 ms  20.242 ms  20.240 ms
        5  lag-26.rcr01albynyyf.netops.charter.com (24.58.32.56)  20.238 ms  20.237 ms  25.927 ms
        6  lag-26.nycmny837aw-bcr00.netops.charter.com (24.30.201.130)  33.618 ms  30.965 ms  30.937 ms
        7  lag-1.pr2.nyc20.netops.charter.com (66.109.9.5)  32.011 ms  33.845 ms  33.798 ms
        8  103.244.50.226 (103.244.50.226)  33.798 ms  34.921 ms  34.913 ms
        9  151.101.66.216 (151.101.66.216)  37.207 ms  38.330 ms *"#;

        let expected = &[
            "192.168.1.1",
            "142.254.211.185",
            "24.58.201.49",
            "24.58.35.16",
            "24.58.32.56",
            "24.30.201.130",
            "66.109.9.5",
            "103.244.50.226",
            "151.101.66.216",
        ];

        assert_eq!(parse_trace(trace), *expected);
    }
}
