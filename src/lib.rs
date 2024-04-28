//! Library SIGINT handling

pub mod flag;
pub mod iterator;
pub mod low_level;

    use libc::c_int;

    /// The signal constants.
    ///
    /// Can be mass-imported by `use signal_hook::consts::signal::*`, without polluting the
    /// namespace with other names. Also available in the [`consts`][crate::consts] directly (but
    /// with more constants around).
    pub mod signal {
        pub use libc::{
            SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGFPE, SIGHUP, SIGILL, SIGINT, SIGKILL,
            SIGPIPE, SIGPROF, SIGQUIT, SIGSEGV, SIGSTOP, SIGSYS, SIGTERM, SIGTRAP, SIGTSTP,
            SIGTTIN, SIGTTOU, SIGURG, SIGUSR1, SIGUSR2, SIGVTALRM, SIGWINCH, SIGXCPU, SIGXFSZ,
        };

        pub use libc::SIGIO;

        pub use libc::SIGINFO;
    }

    pub use self::signal::*;

    /// Various signals commonly requesting shutdown of an application.
    pub const TERM_SIGNALS: &[c_int] = &[SIGTERM, SIGQUIT, SIGINT];


pub use signal_hook_registry::SigId;
