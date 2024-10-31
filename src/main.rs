use {
    std::{
        env::args,
        io::{ BufRead, BufReader },
        process::{ Command, Stdio }
    },
    anyhow::{ Result, anyhow },
    htb_vpn::tray::AppTray,
    colored::Colorize
};

fn main() -> Result<()> {
    let mut switch: &str = "";
    
    let arguments = &args().collect::<Vec<String>>(); 
    match arguments.get(1) {
        Some(arg) => { switch = arg; },
        None => { print_usage(); return Err(anyhow!("failure getting first argument.")) }
    }

    match switch {
        "-c" | "--connect" => {
            #[allow(unused)] let tray = AppTray::establish()?;

            let config: String;
            match arguments.get(2) {
                Some(path) => { config = path.clone(); },
                None => { print_usage(); return Err(anyhow!("No configuration file was specified.")) }
            }

            let mut openvpn = Command::new("sudo")
                .args(["openvpn", &config])
                .stdout(Stdio::piped())
                .spawn().expect("failed to start openvpn.");

            let openvpn_stdout = openvpn.stdout.as_mut().expect("Failure in capturing openvpn's stdout.");
            let openvpn_stdout = BufReader::new(openvpn_stdout);

            openvpn_stdout.lines().for_each(htb_log);

            openvpn.wait()?;
        },

        "-d" | "--disconnect" => {
            Command::new("sudo")
                .args(["killall", "htb-vpn", "openvpn"])
                .spawn().expect("running killall failed.")
                .wait().unwrap();
        },

        other => { print_usage(); return Err(anyhow!("Unrecognized option {other}")) }
    }

    Ok(())
}

fn htb_log(line: std::io::Result<String>) {
    println!("{}{} {}",
        " HTB ".truecolor(17, 27, 43).on_truecolor(159, 239, 0),
        " OpenVPN ".truecolor(159, 239, 0).on_truecolor(17, 27, 43),
        line.unwrap()
    );
}

fn print_usage() {
    eprintln!("usage: htb-vpn [OPTION] [CONFIG_PATH]");
    eprintln!(" OPTIONS:");
    eprintln!("    -c, --connect          Connect to the HTB network with your configuration file.");
    eprintln!("    -d, --disconnect       Disconnect from the HTB network.");
}
