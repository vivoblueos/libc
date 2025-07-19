/// BlueOS's libc is supposed to support POSIX.1-2001.
pub type clock_t = ::c_long;

pub type c_char = i8;
pub type wchar_t = u32;
pub type c_long = i32;
pub type c_ulong = u32;

pub type sigset_t = ::c_ulong;

s! {
    pub struct timespec {
        pub tv_sec: ::time_t,
        pub tv_nsec: ::c_long,
    }

    pub struct dirent {
        pub d_ino: ::ino_t,
        pub d_off: ::off_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 0],
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::socklen_t,
        pub msg_flags: ::c_int,
    }

    pub struct sockaddr {
        pub sa_len: u8,
        pub sa_family: ::sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in6 {
        pub sin6_len: u8,
        pub sin6_family: ::sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_vport: ::in_port_t,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: ::sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_vport: ::in_port_t,
        pub sin_zero: [u8; 6],
    }

    pub struct sockaddr_un {
        pub ss_len: u8,
        pub sun_family: ::sa_family_t,
        pub sun_path: [::c_char; 108usize],
    }

    pub struct sockaddr_storage {
        pub ss_len: u8,
        pub ss_family: ::sa_family_t,
        pub __ss_pad1: [u8; 2],
        pub __ss_align: i64,
        pub __ss_pad2: [u8; 116],
    }

    pub struct sched_param {
        pub sched_priority: ::c_int,
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_spare4: [::c_long; 2usize],
    }

    pub struct statfs {
        pub f_type: ::c_ulong,
        pub f_bsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_fsid: u64,
        pub f_namelen: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_flags: ::c_ulong,
        pub f_spare: [::c_ulong; 4],
    }

    pub struct pthread_barrier_t {
        #[cfg(target_pointer_width = "32")]
        pub __librs_internal_size: [::c_uchar; 24],
        #[cfg(target_pointer_width = "64")]
        pub __librs_internal_size: [::c_uchar; 40],
        pub __librs_internal_align: [::c_ulonglong; 0],
    }

    pub struct pthread_barrierattr_t {
        pub __librs_internal_size: [::c_uchar; 4],
        pub __librs_internal_align: [::c_int; 0],
    }
}

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/sys_socket.h.html
pub const AF_UNIX: ::c_int = 1;
pub const AF_INET6: ::c_int = 24;
pub const SOCK_RAW: ::c_int = 3;
pub const SOCK_RDM: ::c_int = 4;
pub const SOCK_SEQPACKET: ::c_int = 5;
pub const SOMAXCONN: ::c_int = 128;
pub const SO_NONBLOCK: ::c_int = 0x1100;
pub const IPPROTO_RAW: ::c_int = 255;
pub const MSG_OOB: ::c_int = 0x1;
pub const MSG_PEEK: ::c_int = 0x2;
pub const MSG_DONTROUTE: ::c_int = 0x4;
pub const MSG_EOR: ::c_int = 0x8;
pub const MSG_TRUNC: ::c_int = 0x10;
pub const MSG_CTRUNC: ::c_int = 0x20;
pub const MSG_WAITALL: ::c_int = 0x40;
pub const MSG_DONTWAIT: ::c_int = 0x80;
pub const MSG_BCAST: ::c_int = 0x100;
pub const MSG_MCAST: ::c_int = 0x200;

// Refer to FreeBSD.
pub const SOL_SOCKET: ::c_int = 0xffff;
pub const SO_TYPE: ::c_int = 0x1008;
pub const SO_PROTOCOL: ::c_int = 0x1016;
pub const SO_DOMAIN: ::c_int = 0x1019;

pub const FIONBIO: ::c_ulong = 1;

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/poll.h.html
pub const POLLIN: ::c_short = 0x0001;
pub const POLLPRI: ::c_short = POLLIN;
pub const POLLOUT: ::c_short = 0x0004;
pub const POLLRDNORM: ::c_short = POLLIN;
pub const POLLRDBAND: ::c_short = POLLIN;
pub const POLLWRNORM: ::c_short = POLLOUT;
pub const POLLWRBAND: ::c_short = POLLOUT;
pub const POLLERR: ::c_short = 0x0008;
pub const POLLHUP: ::c_short = 0x0010;
pub const POLLNVAL: ::c_short = 0x0020;

/// https://pubs.opengroup.org/onlinepubs/9799919799/functions/dlsym.html
pub const RTLD_DEFAULT: *mut ::c_void = 0 as *mut ::c_void;

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/sys_stat.h.html
pub const UTIME_OMIT: c_long = -1;

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/fcntl.h.html
pub const AT_FDCWD: ::c_int = -2;
pub const O_DIRECTORY: ::c_int = 0x200000;
pub const O_NOFOLLOW: ::c_int = 0x100000;
pub const AT_EACCESS: ::c_int = 1;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 2;
pub const AT_SYMLINK_FOLLOW: ::c_int = 4;
pub const AT_REMOVEDIR: ::c_int = 8;

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/signal.h.html
pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGTRAP: ::c_int = 5;
pub const SIGABRT: ::c_int = 6;
pub const SIGEMT: ::c_int = 7;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGBUS: ::c_int = 10;
pub const SIGSEGV: ::c_int = 11;
pub const SIGSYS: ::c_int = 12;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/netdb.h.html
pub const EAI_BADFLAGS: ::c_int = -1;
pub const EAI_NONAME: ::c_int = -2;
pub const EAI_AGAIN: ::c_int = -3;
pub const EAI_FAIL: ::c_int = -4;
pub const EAI_NODATA: ::c_int = -5;
pub const EAI_FAMILY: ::c_int = -6;
pub const EAI_SOCKTYPE: ::c_int = -7;
pub const EAI_SERVICE: ::c_int = -8;
pub const EAI_ADDRFAMILY: ::c_int = -9;
pub const EAI_MEMORY: ::c_int = -10;
pub const EAI_SYSTEM: ::c_int = -11;
pub const EAI_OVERFLOW: ::c_int = -12;

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/unistd.h.html
pub const _SC_PAGESIZE: ::c_int = 8;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 51;

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/limits.h.html
pub const PTHREAD_STACK_MIN: ::size_t = 4 * 1024;

/// https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/pthread.h.html
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = ::PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_MUTEX_STALLED: ::c_int = 0;
pub const PTHREAD_MUTEX_ROBUST: ::c_int = 1;
pub const PTHREAD_PRIO_NONE: ::c_int = 0;
pub const PTHREAD_PROCESS_PRIVATE: ::c_int = 0;

pub const IP_HDRINCL: ::c_int = 2;

extern "C" {
    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
    pub fn writev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;
    pub fn readv(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;

    pub fn sendmsg(s: ::c_int, msg: *const ::msghdr, flags: ::c_int) -> ::ssize_t;
    pub fn recvmsg(s: ::c_int, msg: *mut ::msghdr, flags: ::c_int) -> ::ssize_t;

    pub fn pthread_create(
        native: *mut ::pthread_t,
        attr: *const ::pthread_attr_t,
        f: extern "C" fn(_: *mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;

    pub fn pthread_attr_getschedparam(
        attr: *const ::pthread_attr_t,
        param: *mut sched_param,
    ) -> ::c_int;

    pub fn pthread_attr_setschedparam(
        attr: *mut ::pthread_attr_t,
        param: *const sched_param,
    ) -> ::c_int;

    pub fn pthread_attr_getprocessorid_np(
        attr: *const ::pthread_attr_t,
        processor_id: *mut ::c_int,
    ) -> ::c_int;

    pub fn pthread_attr_setprocessorid_np(
        attr: *mut ::pthread_attr_t,
        processor_id: ::c_int,
    ) -> ::c_int;

    pub fn pthread_getschedparam(
        native: ::pthread_t,
        policy: *mut ::c_int,
        param: *mut ::sched_param,
    ) -> ::c_int;

    pub fn pthread_setschedparam(
        native: ::pthread_t,
        policy: ::c_int,
        param: *const ::sched_param,
    ) -> ::c_int;

    pub fn pthread_condattr_getclock(
        attr: *const ::pthread_condattr_t,
        clock_id: *mut ::clockid_t,
    ) -> ::c_int;

    pub fn pthread_condattr_setclock(
        attr: *mut ::pthread_condattr_t,
        clock_id: ::clockid_t,
    ) -> ::c_int;

    pub fn pthread_getprocessorid_np() -> ::c_int;

    pub fn getentropy(buf: *mut ::c_void, buflen: ::size_t) -> ::c_int;

    pub fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int;
}
