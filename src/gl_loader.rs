use shared_library::dynamic_library::DynamicLibrary;
use std::path::Path;
use std::ptr;

/// port from https://github.com/itadinanta/gfx-gtk

type LibPtr = *const std::os::raw::c_void;

trait ProcLoader {
    fn get_proc_addr(&self, s: &str) -> Option<LibPtr>;
}

struct DlProcLoader {
    lib: Option<shared_library::dynamic_library::DynamicLibrary>,
}

fn fn_from<P>(loader: P) -> impl Fn(&str) -> LibPtr
where
    P: ProcLoader + Sized,
{
    move |s| loader.get_proc_addr(s).unwrap_or_else(|| ptr::null())
}

impl DlProcLoader {
    pub fn open(lib_path: &Path) -> Self {
        DlProcLoader {
            lib: DynamicLibrary::open(Some(lib_path)).ok(),
        }
    }
    pub fn current_module() -> Self {
        DlProcLoader {
            lib: DynamicLibrary::open(None).ok(),
        }
    }
}
impl ProcLoader for DlProcLoader {
    fn get_proc_addr(&self, s: &str) -> Option<LibPtr> {
        self.lib
            .as_ref()
            .and_then(|l| match unsafe { l.symbol(s) } {
                Ok(v) => Some(v as LibPtr),
                Err(_) => None,
            })
    }
}

struct Failover<A, B>(pub A, pub B)
where
    A: ProcLoader,
    B: ProcLoader;

impl<A, B> ProcLoader for Failover<A, B>
where
    A: ProcLoader,
    B: ProcLoader,
{
    fn get_proc_addr(&self, s: &str) -> Option<LibPtr> {
        self.0.get_proc_addr(s).or_else(|| self.1.get_proc_addr(s))
    }
}

pub fn load() {
    let loader = Failover(
        DlProcLoader::current_module(),
        Failover(
            DlProcLoader::open(Path::new("libepoxy-0")),
            Failover(
                DlProcLoader::open(Path::new("libepoxy0")),
                DlProcLoader::open(Path::new("libepoxy")),
            ),
        ),
    );
    epoxy::load_with(fn_from(loader));
    gl::load_with(epoxy::get_proc_addr);
}
