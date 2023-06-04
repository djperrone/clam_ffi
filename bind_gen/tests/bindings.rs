use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            class: "InteropClass".to_string(),
            dll_name: "bind_gen".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            ..Config::default()
        },
        bind_gen::my_inventory(),
    )
    .write_file("bindings/csharp/Interop.cs")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_c() -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator};
    
    Generator::new(
        Config {
            ifndef: "bind_gen".to_string(),
            ..Config::default()
        },
        bind_gen::my_inventory(),
    )
    .write_file("bindings/c/example_complex.h")?;

    Ok(())
}
