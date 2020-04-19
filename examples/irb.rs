// mrusty_class!
#[macro_use]
extern crate mrusty;

use mrusty::{Mruby, MrubyImpl, Repl, GnuReadLine};

// run with:
// cargo run --features "gnu-readline" --example irb

fn main() {
    let mruby = Mruby::new();
    
    struct Cont {
        value: i32
    }

    // Cont should not flood the current namespace. We will add it with require.
    mrusty_class!(Cont, "Container", {
        // Converts mruby types automatically & safely.
        def!("initialize", |v: i32| {
            Cont { value: v }
        });

        // Converts slf to Cont.
        def!("value", |mruby, slf: (&Cont)| {
            mruby.fixnum(slf.value)
        });
    });

    // Add file to the context, making it requirable.
    mruby.def_file::<Cont>("cont");
    
    
    let mut repl = Repl::new(mruby);
    repl.rename("repl");
    repl.start(&GnuReadLine);
}
