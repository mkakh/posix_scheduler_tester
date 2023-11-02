pub const EOK: usize = 0;

/* errno (POSIX) */
/* source: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/errno.h.html */
pub const EPERM: usize = 1; /* Operation not permitted */
pub const ENOENT: usize = 2; /* No such file or directory */
pub const ESRCH: usize = 3; /* No such process */
pub const EINTR: usize = 4; /* Interrupted function */
pub const EIO: usize = 5; /* I/O error */
pub const ENXIO: usize = 6; /* No such device or address */
pub const E2BIG: usize = 7; /* Argument list too long */
pub const ENOEXEC: usize = 8; /* Executable file format error */
pub const EBADF: usize = 9; /* Bad file descriptor */
pub const ECHILD: usize = 10; /* No child processes */
pub const EAGAIN: usize = 11; /* Resource unavailable, try again (may be the same value as [EWOULDBLOCK]) */
pub const EWOULDBLOCK: usize = 11; /* Operation would block (may be the same value as [EAGAIN]) */
pub const ENOMEM: usize = 12; /* Not enough space */
pub const EACCES: usize = 13; /* Permission denied */
pub const EFAULT: usize = 14; /* Bad address */
pub const EBUSY: usize = 16; /* Device or resource busy */
pub const EEXIST: usize = 17; /* File exists */
pub const EXDEV: usize = 18; /* Cross-device link */
pub const ENODEV: usize = 19; /* No such device */
pub const ENOTDIR: usize = 20; /* Not a directory or a symbolic link to a directory */
pub const EISDIR: usize = 21; /* Is a directory */
pub const EINVAL: usize = 22; /* Invalid argument */
pub const ENFILE: usize = 23; /* Too many files open in system */
pub const EMFILE: usize = 24; /* File descriptor value too large */
pub const ENOTTY: usize = 25; /* Inappropriate I/O control operation */
pub const ETXTBSY: usize = 26; /* Text file busy */
pub const EFBIG: usize = 27; /* File too large */
pub const ENOSPC: usize = 28; /* No space left on device */
pub const ESPIPE: usize = 29; /* Invalid seek */
pub const EROFS: usize = 30; /* Read-only file system */
pub const EMLINK: usize = 31; /* Too many links */
pub const EPIPE: usize = 32; /* Broken pipe */
pub const EDOM: usize = 33; /* Mathematics argument out of domain of function */
pub const ERANGE: usize = 34; /* Result too large */
pub const EDEADLK: usize = 35; /* Resource deadlock would occur */
pub const ENAMETOOLONG: usize = 36; /* Filename too long */
pub const ENOLCK: usize = 37; /* No locks available */
pub const ENOSYS: usize = 38; /* Functionality not supported */
pub const ENOTEMPTY: usize = 39; /* Directory not empty */
pub const ELOOP: usize = 40; /* Too many levels of symbolic links */
pub const ENOMSG: usize = 42; /* No message of the desired type */
pub const EIDRM: usize = 43; /* Identifier removed */
pub const ENOSTR: usize = 60; /* Not a STREAM */
pub const ENODATA: usize = 61; /* No message is available on the STREAM head read queue */
pub const ETIME: usize = 62; /* Stream ioctl() timeout */
pub const ENOSR: usize = 63; /* No STREAM resources */
pub const ENOLINK: usize = 67; /* Reserved */
pub const EPROTO: usize = 71; /* Protocol error */
pub const EMULTIHOP: usize = 72; /* Reserved */
pub const EOVERFLOW: usize = 75; /* Value too large to be stored in data type */
pub const EBADMSG: usize = 74; /* Bad message */
pub const EILSEQ: usize = 84; /* Illegal byte sequence */
pub const ENOTSOCK: usize = 88; /* Not a socket */
pub const EDESTADDRREQ: usize = 89; /* Destination address required */
pub const EMSGSIZE: usize = 90; /* Message too large */
pub const ENOPROTOOPT: usize = 92; /* Protocol not available */
pub const EPROTONOSUPPORT: usize = 93; /* Protocol wrong type for socket */
pub const ENOTSUP: usize = 95; /* Not supported (may be the same value as [EOPNOTSUP] */
pub const EOPNOTSUPP: usize = 95; /* Operation not supported on socket (may be the same value as [ENOTSUP] */
pub const EADDRINUSE: usize = 98; /* Address in use */
pub const EADDRNOTAVAIL: usize = 99; /* Address not available */
pub const EAFNOSUPPORT: usize = 97; /* Address family not supported */
pub const ENETDOWN: usize = 100; /* Network is down */
pub const ENETUNREACH: usize = 101; /* Network unreachable */
pub const ENETRESET: usize = 102; /* Connection aborted by network */
pub const ECONNABORTED: usize = 103; /* Connection aborted */
pub const ECONNRESET: usize = 104; /* Connection reset */
pub const ENOBUFS: usize = 105; /* No buffer space available */
pub const EISCONN: usize = 106; /* Socket is connected */
pub const ENOTCONN: usize = 107; /* The socket is not connected */
pub const ETIMEDOUT: usize = 110; /* Connection timed out */
pub const ECONNREFUSED: usize = 111; /* Connection refused */
pub const EHOSTUNREACH: usize = 113; /* Host is unreachable */
pub const EALREADY: usize = 114; /* Connection already in progress */
pub const EINPROGRESS: usize = 115; /* Operation in progress */
pub const ESTALE: usize = 116; /* Reserved */
pub const EDQUOT: usize = 122; /* Reserved */
pub const ECANCELED: usize = 125; /* Operation Canceled */
pub const EOWNERDEAD: usize = 130; /* Previous owner died */
pub const ENOTRECOVERABLE: usize = 131; /* State not recoverable */

/* errno (Linux) */
/* source: /usr/include/asm-generic/{errno-base, errno}.h */
pub const ENOTBLK: usize = 15; /* Block device required */
pub const ECHRNG: usize = 44; /* Channel number out of range */
pub const EL2NSYNC: usize = 45; /* Level 2 not synchronized */
pub const EL3HLT: usize = 46; /* Level 3 halted */
pub const EL3RST: usize = 47; /* Level 3 reset */
pub const ELNRNG: usize = 48; /* Link number out of range */
pub const EUNATCH: usize = 49; /* Protocol driver not attached */
pub const ENOCSI: usize = 50; /* No CSI structure available */
pub const EL2HLT: usize = 51; /* Level 2 halted */
pub const EBADE: usize = 52; /* Invalid exchange */
pub const EBADR: usize = 53; /* Invalid request descriptor */
pub const EXFULL: usize = 54; /* Exchange full */
pub const ENOANO: usize = 55; /* No anode */
pub const EBADRQC: usize = 56; /* Invalid request code */
pub const EBADSLT: usize = 57; /* Invalid slot */
pub const EDEADLOCK: usize = EDEADLK;
pub const EBFONT: usize = 59; /* Bad font file format */
pub const ENONET: usize = 64; /* Machine is not on the network */
pub const ENOPKG: usize = 65; /* Package not installed */
pub const EREMOTE: usize = 66; /* Object is remote */
pub const EADV: usize = 68; /* Advertise error */
pub const ESRMNT: usize = 69; /* Srmount error */
pub const ECOMM: usize = 70; /* Communication error on send */
pub const EDOTDOT: usize = 73; /* RFS specific error */
pub const ENOTUNIQ: usize = 76; /* Name not unique on network */
pub const EBADFD: usize = 77; /* File descriptor in bad state */
pub const EREMCHG: usize = 78; /* Remote address changed */
pub const ELIBACC: usize = 79; /* Can not access a needed shared library */
pub const ELIBBAD: usize = 80; /* Accessing a corrupted shared library */
pub const ELIBSCN: usize = 81; /* .lib section in a.out corrupted */
pub const ELIBMAX: usize = 82; /* Attempting to link in too many shared libraries */
pub const ELIBEXEC: usize = 83; /* Cannot exec a shared library directly */
pub const ERESTART: usize = 85; /* Interrupted system call should be restarted */
pub const ESTRPIPE: usize = 86; /* Streams pipe error */
pub const EUSERS: usize = 87; /* Too many users */
pub const EPROTOTYPE: usize = 91; /* Protocol wrong type for socket */
pub const ESOCKTNOSUPPORT: usize = 94; /* Socket type not supported */
pub const EPFNOSUPPORT: usize = 96; /* Protocol family not supported */
pub const ESHUTDOWN: usize = 108; /* Cannot send after transport endpoint shutdown */
pub const ETOOMANYREFS: usize = 109; /* Too many references: cannot splice */
pub const EHOSTDOWN: usize = 112; /* Host is down */
pub const EUCLEAN: usize = 117; /* Structure needs cleaning */
pub const ENOTNAM: usize = 118; /* Not a XENIX named type file */
pub const ENAVAIL: usize = 119; /* No XENIX semaphores available */
pub const EISNAM: usize = 120; /* Is a named type file */
pub const EREMOTEIO: usize = 121; /* Remote I/O error */
pub const ENOMEDIUM: usize = 123; /* No medium found */
pub const EMEDIUMTYPE: usize = 124; /* Wrong medium type */
pub const ENOKEY: usize = 126; /* Required key not available */
pub const EKEYEXPIRED: usize = 127; /* Key has expired */
pub const EKEYREVOKED: usize = 128; /* Key has been revoked */
pub const EKEYREJECTED: usize = 129; /* Key was rejected by service */
pub const ERFKILL: usize = 132; /* Operation not possible due to RF-kill */
pub const EHWPOISON: usize = 133; /* Memory page has hardware error */
