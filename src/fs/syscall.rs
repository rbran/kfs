mod chmod;
mod chown;
mod close;
mod cwd;
mod fcntl;
mod getcwd;
mod getdents;
mod ioctl;
mod lseek;
mod mkdir;
mod mount;
mod open;
mod read;
mod stat;
mod symlink;
mod truncate;
mod unlink;
mod write;

pub use chmod::sys_chmod;
pub use chown::sys_chown;
pub use close::sys_close;
pub use cwd::*;
pub use fcntl::sys_fcntl;
pub use getcwd::sys_getcwd;
pub use getdents::sys_getdents;
pub use ioctl::sys_ioctl;
pub use lseek::sys_lseek;
pub use mkdir::sys_mkdir;
pub use mount::{sys_mount, sys_umount};
pub use open::{sys_creat, sys_open};
pub use read::sys_read;
pub use stat::{sys_fstat, sys_stat};
pub use symlink::sys_symlink;
pub use truncate::sys_truncate;
pub use unlink::{sys_rmdir, sys_unlink};
pub use write::{sys_write, sys_writev};

use crate::process::{fd_table::Fd, task::CURRENT};
use crate::syscall::errno::Errno;

use super::vfs::VfsHandle;

pub fn get_file(fd: isize) -> Result<VfsHandle, Errno> {
	let fd = Fd::from(fd as usize).ok_or(Errno::EBADF)?;
	let fd_table = unsafe { CURRENT.get_mut() }
		.get_user_ext()
		.expect("user task")
		.lock_fd_table();

	fd_table.get_file(fd).ok_or(Errno::EBADF)
}
