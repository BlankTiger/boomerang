use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use tracing::info;
use tracing_subscriber::EnvFilter;

const SERVER_URL: &str = "https://github.com/BlankTiger/boomerang/releases/download/master/boomerang-server-linux-aarch64";
const FILENAME: &str = "bin/boomerang_server";
const TEMP_FILENAME: &str = "bin/boomerang_server-new";
const TIMEOUT: u64 = 2 * 60;

pub fn setup() -> color_eyre::Result<()> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

struct Server {
    proc: Option<std::process::Child>,
}

impl Server {
    fn new() -> Self {
        Self { proc: None }
    }

    fn restart(&mut self) {
        self.stop();
        self.start();
    }

    fn is_running(&self) -> bool {
        self.proc.is_some()
    }

    fn stop(&mut self) {
        if let Some(ref mut proc) = self.proc {
            proc.kill().unwrap();
            self.proc = None;
        }
    }

    fn start(&mut self) {
        if self.is_running() {
            info!("server is already running");
            return;
        }
        info!("starting server");
        let mut perms = std::fs::metadata(FILENAME).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(FILENAME, perms).unwrap();

        let proc = std::process::Command::new("./bin/boomerang_server")
            .spawn()
            .unwrap();
        info!("started server");
        self.proc = Some(proc);
    }
}

fn main() -> color_eyre::Result<()> {
    setup()?;
    let mut server = Server::new();

    loop {
        //create bin directory if doesnt exist
        if !std::path::Path::new("bin").exists() {
            std::fs::create_dir("bin")?;
        }

        // download server binary
        info!("downloading server binary");
        let resp = reqwest::blocking::get(SERVER_URL)?;
        let mut file = std::fs::File::create(TEMP_FILENAME)?;
        let mut content = std::io::Cursor::new(resp.bytes()?);
        std::io::copy(&mut content, &mut file)?;
        drop(file);
        info!("downloaded server binary");

        // if main file doesnt exist, then place the downloaded file
        // and restart the server
        if !std::path::Path::new(FILENAME).exists() {
            info!("main file doesnt exist, placing downloaded file");
            std::fs::rename(TEMP_FILENAME, FILENAME)?;
            info!("placed downloaded file");
            info!("restarting server");
            server.start();
            continue;
        }

        // get md5 hash of the downloaded file
        info!("getting md5 hash of the downloaded file");
        let mut new_file = std::fs::File::open(TEMP_FILENAME)?;
        let mut new_file_bytes = vec![];
        new_file.read_to_end(&mut new_file_bytes)?;
        new_file.set_permissions(std::fs::Permissions::from_mode(0o755))?;
        drop(new_file);
        let hash = md5::compute(new_file_bytes);
        let hash = format!("{:x}", hash);
        info!("hash: {}", hash);

        // get md5 hash of the current file
        // if the hashes are different, then replace the current file
        // and restart the server
        info!("getting md5 hash of the current file");
        let mut current_file = std::fs::File::open(FILENAME)?;
        let mut current_file_bytes = vec![];
        current_file.read_to_end(&mut current_file_bytes)?;
        drop(current_file);
        let current_hash = md5::compute(current_file_bytes);
        let current_hash = format!("{:x}", current_hash);
        info!("current hash: {}", current_hash);

        if hash != current_hash {
            info!("hashes are different, replacing current file");
            std::fs::rename(TEMP_FILENAME, FILENAME)?;
            info!("replaced current file");
            info!("restarting server");
            server.restart();
            info!("restarted server");
        } else {
            info!("hashes are the same, not replacing current file");
            std::fs::remove_file(TEMP_FILENAME)?;
            if server.is_running() {
                info!("server is running");
            } else {
                info!("server is not running");
                server.start();
            }
        }

        info!("sleeping for {} seconds", TIMEOUT);
        std::thread::sleep(std::time::Duration::from_secs(TIMEOUT));
    }
}
