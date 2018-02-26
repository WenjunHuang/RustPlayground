mod git;

extern crate libc;

use git::raw;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ptr;

fn check(activity: &'static str, status: c_int) -> c_int {
    if status < 0 {
        unsafe {
            let error = &*raw::giterr_last();
            println!("error while {}: {}({})",
                     activity,
                     CStr::from_ptr(error.message).to_string_lossy(),
                     error.klass);
            std::process::exit(1);
        }
    }
    status
}

unsafe fn show_commit(commit: *const raw::git_commit) {
    let author = raw::git_commit_author(commit);

    let name = CStr::from_ptr((*author).name).to_string_lossy();
    let email = CStr::from_ptr((*author).email).to_string_lossy();
    println!("{} <{}>\n", name, email);

    let message = raw::git_commit_message(commit);
    println!("{}", CStr::from_ptr(message).to_string_lossy());
}

fn main() {
    let path = std::env::args().skip(1)
        .next()
        .expect("usage: git-toy PATH");

    let repo = git::Repository::open(&path).expect("opening repository");
    let commit_oid = repo.reference_name_to_id("HEAD").expect("looking up 'HEAD' reference");
    let commit = repo.find_commit(&commit_oid).expect("looking up commit");
    let author = commit.author();

    println!("{} <{}>\n",
             author.name().unwrap_or("(none)"),
             author.email().unwrap_or("none"));
    println!("{}",commit.message().unwrap_or("(none)"));
}
