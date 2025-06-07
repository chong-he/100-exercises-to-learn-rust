//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // the idea is Rc cannot be created before an await
    // so if we have await first, then Rc, then is fine
    yield_now().await;
    let non_send = Rc::new(1);

    println!("{}", non_send);

    // another one that works:
    // {
    //    let non_send = Rc::new(1);
    //    println!("{}", non_send);
    // }  <- non_send is dropped here since Rc is dropped after {}, so we can call await after it has been dropped
    // yield_now().await;
}
