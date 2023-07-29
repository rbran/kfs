use core::array;

use alloc::sync::Arc;
use kfs_macro::context;

use crate::{file::File, sync::locked::Locked};

const FDTABLE_SIZE: usize = 256;

#[derive(Debug)]
pub struct Fd(usize);

impl Fd {
	#[inline(always)]
	pub fn index(&self) -> usize {
		self.0
	}

	pub fn from(v: usize) -> Option<Self> {
		(v < FDTABLE_SIZE).then(|| Self(v))
	}
}

pub struct FdTable(Locked<[Option<Arc<File>>; FDTABLE_SIZE]>);

impl FdTable {
	pub fn new() -> Self {
		Self(Locked::new(array::from_fn(|_| None)))
	}

	pub fn clone_for_fork(&self) -> Self {
		Self(self.0.clone())
	}

	#[context(irq_disabled)]
	pub fn get_file(&self, fd: Fd) -> Option<Arc<File>> {
		self.0.lock()[fd.index()].clone()
	}

	#[context(irq_disabled)]
	pub fn alloc_fd(&self, file: Arc<File>) -> Option<Fd> {
		let mut table = self.0.lock();

		let (fd, _) = table.iter().enumerate().find(|(_, x)| x.is_none())?;
		table[fd] = Some(file);
		Some(Fd(fd))
	}
}
