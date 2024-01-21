use std::io::Read;
use std::os::unix::fs::PermissionsExt;

const SERVER_URL: &str = "https://github.com/BlankTiger/boomerang/releases/download/master/boomerang-server-linux-aarch64";
const FILENAME: &str = "bin/boomerang_server";
const TEMP_FILENAME: &str = "bin/boomerang_server-new";
const TIMEOUT: u64 = 3 * 60;

fn main() {
    loop {
        //create bin directory if doesnt exist
        if !std::path::Path::new("bin").exists() {
            std::fs::create_dir("bin").unwrap();
        }

        // download server binary
        println!("downloading server binary");
        let resp = reqwest::blocking::get(SERVER_URL).unwrap();
        let mut file = std::fs::File::create(TEMP_FILENAME).unwrap();
        let mut content = std::io::Cursor::new(resp.bytes().unwrap());
        std::io::copy(&mut content, &mut file).unwrap();
        println!("downloaded server binary");

        // if main file doesnt exist, then place the downloaded file
        // and restart the server
        if !std::path::Path::new(FILENAME).exists() {
            println!("main file doesnt exist, placing downloaded file");
            std::fs::rename(TEMP_FILENAME, FILENAME).unwrap();
            println!("placed downloaded file");
            println!("restarting server");
            start_server();
            continue;
        }

        // get md5 hash of the downloaded file
        println!("getting md5 hash of the downloaded file");
        let mut new_file = std::fs::File::open(TEMP_FILENAME).unwrap();
        let mut new_file_bytes = vec![];
        new_file.read_to_end(&mut new_file_bytes).unwrap();
        let hash = md5::compute(new_file_bytes);
        let hash = format!("{:x}", hash);
        println!("hash: {}", hash);

        // get md5 hash of the current file
        // if the hashes are different, then replace the current file
        // and restart the server
        println!("getting md5 hash of the current file");
        let mut current_file = std::fs::File::open(FILENAME).unwrap();
        let mut current_file_bytes = vec![];
        current_file.read_to_end(&mut current_file_bytes).unwrap();
        let current_hash = md5::compute(current_file_bytes);
        let current_hash = format!("{:x}", current_hash);
        println!("current hash: {}", current_hash);

        if !is_running() {
            start_server();
        }

        if hash != current_hash {
            println!("hashes are different, replacing current file");
            std::fs::rename(TEMP_FILENAME, FILENAME).unwrap();
            println!("replaced current file");
            println!("restarting server");
            std::process::Command::new("systemctl")
                .arg("restart")
                .arg("boomerang-server")
                .output()
                .unwrap();
            println!("restarted server");
            restart_server();
        } else {
            println!("hashes are the same, not replacing current file");
            std::fs::remove_file(TEMP_FILENAME).unwrap();
        }

        println!("sleeping for {} seconds", TIMEOUT);
        std::thread::sleep(std::time::Duration::from_secs(TIMEOUT));
    }
}

fn restart_server() {
    if is_running() {
        std::process::Command::new("killall")
            .arg("boomerang_server")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
    start_server();
}

fn is_running() -> bool {
    let pid = std::process::Command::new("pgrep")
        .arg("-f")
        .arg("\"boomerang_server\"")
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    !pid.stdout.is_empty()
}

fn start_server() {
    println!("starting server");
    let mut perms = std::fs::metadata(FILENAME).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(FILENAME, perms).unwrap();
    std::process::Command::new("./bin/boomerang_server")
        .spawn()
        .unwrap();
    println!("started server");
}
