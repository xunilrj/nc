
use super::errno::*;
use super::sysno::*;
use super::types::*;
use super::{syscall0, syscall1, syscall2, syscall3};

fn c_str(s: &str) -> [u8; 128]{
    let mut buf: [u8; 128] = [42; 128];
    for (i, b) in s.bytes().enumerate() {
        buf[i] = b;
    }
    // TODO(Shaohua): Assert length
    buf[s.len()] = 0;
    return buf;
}

/// Close a file descriptor.
pub fn close(fd: isize) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let ret = syscall1(SYS_CLOSE, fd);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            return Ok(());
        }
    }
}

/// Terminate current process.
pub fn exit(status: u8) {
    unsafe {
        syscall1(SYS_EXIT, status as usize);
    }
}

/// Create a child process.
pub fn fork() -> Result<pid_t, Errno> {
    unsafe {
        let ret = syscall0(SYS_FORK);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            return Ok(ret as pid_t);
        }
    }
}

/// Get the effective group ID of the calling process.
pub fn getegid() -> gid_t {
    unsafe {
        return syscall0(SYS_GETEGID) as gid_t;
    }
}

/// Get the effective user ID of the calling process.
pub fn geteuid() -> uid_t {
    unsafe {
        return syscall0(SYS_GETEUID) as uid_t;
    }
}

/// Get the real group ID of the calling process.
pub fn getgid() -> gid_t {
    unsafe {
        return syscall0(SYS_GETGID) as gid_t;
    }
}

/// Get process group.
pub fn getpgrp() -> pid_t {
    unsafe {
        return syscall0(SYS_GETPGRP) as pid_t;
    }
}

/// Get the process ID (PID) of the calling process.
pub fn getpid() -> pid_t {
    unsafe {
        return syscall0(SYS_GETPID) as pid_t;
    }
}

/// Get the process group ID of the calling process.
pub fn getpgrp() -> pid_t {
    unsafe {
        return syscall0(SYS_GETPGRP) as pid_t;
    }
}

/// Get the process ID of the parent of the calling process.
pub fn getppid() -> pid_t {
    unsafe {
        return syscall0(SYS_GETPPID) as pid_t;
    }
}

/// Get the real user ID of the calling process.
pub fn getuid() -> uid_t {
    unsafe {
        return syscall0(SYS_GETUID) as uid_t;
    }
}

pub fn open(path: &str, flags: i32, mode: mode_t) -> Result<isize, Errno> {
    unsafe {
        let path = c_str(path).as_ptr() as usize;
        let flags = flags as usize;
        let mode = mode as usize;
        let ret = syscall3(SYS_OPEN, path, flags, mode);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            return Ok(reti);
        }
    }
}

pub fn rename(oldpath: &str, newpath: &str) -> Result<(), Errno> {
    unsafe {
        let oldpath = c_str(oldpath).as_ptr() as usize;
        let newpath = c_str(newpath).as_ptr() as usize;
        let ret = syscall2(SYS_RENAME, oldpath, newpath);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            return Ok(());
        }
    }
}

/// Set the group ID of the calling process to `gid`.
pub fn setgid(gid: gid_t) -> Result<(), Errno> {
    unsafe {
        let gid = gid as usize;
        let ret = syscall1(SYS_SETGID, gid);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            return Ok(());
        }
    }
}

/// Set the process group ID (PGID) of the process specified by `pid` to `pgid`.
pub fn setpgid(pid: pid_t, pgid: pid_t) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let pgid = pgid as usize;
        let ret = syscall2(SYS_SETPGID, pid, pgid);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            return Ok(());
        }
    }
}

/// Create a new session if the calling process is not a process group leader.
pub fn setsid() -> Result<pid_t, Errno> {
    unsafe {
        let ret = syscall0(SYS_SETSID);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            let ret = ret as pid_t;
            return Ok(ret);
        }
    }
}

/// Set the effective user ID of the calling process to `uid`.
pub fn setuid(uid: uid_t) -> Result<(), Errno> {
    unsafe {
        let uid = uid as usize;
        let ret = syscall1(SYS_SETUID, uid);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            return Ok(());
        }
    }
}

pub fn write(fd: isize, buf: &[u8]) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let buf_len = buf.len();
        let buf = buf.as_ptr() as usize;
        let ret = syscall3(SYS_WRITE, fd, buf, buf_len);
        let reti = ret as isize;
        if reti < 0 && reti >= -256 {
            return Err(-reti);
        } else {
            return Ok(ret as ssize_t);
        }
    }
}
