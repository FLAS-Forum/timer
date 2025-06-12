use std::env;
use std::thread;
use std::time::{Duration, Instant};
use regex::Regex;
use std::process::exit;

fn parse_duration(input: &str) -> Option<Duration> {
    let re = Regex::new(r"(?i)(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?").unwrap();

    if let Some(caps) = re.captures(input) {
        let hours = caps.get(1).map_or(0, |h| h.as_str().parse().unwrap_or(0));
        let minutes = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
        let seconds = caps.get(3).map_or(0, |s| s.as_str().parse().unwrap_or(0));

        let total_secs = hours * 3600 + minutes * 60 + seconds;
        if total_secs == 0 {
            return None;
        }

        Some(Duration::from_secs(total_secs))
    } else {
        None
    }
}

fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("⏳ Nutzung: timer <Dauer> (z. B. 1h30m20s, 10s, 2m)");
        exit(1);
    }

    let input = &args[1];
    let duration = parse_duration(input);

    if duration.is_none() {
        eprintln!("❌ Ungültige Eingabe. Beispiel: 5m30s oder 45s");
        exit(1);
    }

    let duration = duration.unwrap();
    let start = Instant::now();

    loop {
        let elapsed = start.elapsed();
        if elapsed >= duration {
            break;
        }

        let remaining = duration - elapsed;
        print!("\r⏱️  Noch {} ", format_duration(remaining));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    println!("\r✅ Zeit abgelaufen!                \x07"); // \x07 = Terminal-Beep
}

