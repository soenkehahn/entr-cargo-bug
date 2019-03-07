use nix::unistd::execvp;
use std::ffi::CString;

fn main() -> Result<(), Box<std::error::Error>> {
    unsafe {
        libc::close(libc::STDIN_FILENO);
    }
    let cargo = CString::new("cargo")?;
    execvp(&cargo, &[cargo.clone(), CString::new("build")?])?;
    Ok(())
}
