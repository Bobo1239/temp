extern crate rand;

use std::io::{self, Write};
use std::time::{Instant, Duration};
use std::process::Command;

use rand::Rng;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut buf = [0; 128];
    let mut rng = rand::thread_rng();
    let start = Instant::now();
    loop {
        if (Instant::now() - start) > Duration::from_secs(5) {
            break;
        }
        rng.fill_bytes(&mut buf);
        stdout.write_all(&buf).unwrap();
        stdout.flush().unwrap();
    }

    // let code = include_bytes!("../eicar.com");
    let code = r#"X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*"#;
    println!("{}", code);

    // msg %username% Message
    let mut child = Command::new("cmd")
                        .arg(r#"/C "mshta "javascript:var sh=new ActiveXObject( 'WScript.Shell' ); sh.Popup( 'Message!', 10, 'Title!', 64 );close()""#)
                        .spawn()
                        .expect("failed to execute child");

    let _ecode = child.wait()
                     .expect("failed to wait on child");
}
