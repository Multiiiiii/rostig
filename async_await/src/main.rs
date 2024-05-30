use futures::executor::block_on;
struct song;

fn main() {
    let future = do_something();
    block_on(future); //furure is run and "hellox world" is printed

    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    block_on(async_main());
}

async fn do_something() {
    println!("Hello, world!");
}

async fn learn_song() -> song {
    /* */
    todo!()
}
async fn sing_song(song: song) {
    /* */
}
async fn dance() {
    /* */
}

async fn learn_and_sing() {
    // Wait until the song has been learned befor singing it.
    // We use '.await' here rather than 'block_on' to prevent blocking the 
    // thread, which makes it possible to 'dance' at the same time.
    let song =learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    //'join!' is like '.await' but can wait for multible futures concurrently.
    // If we're temporarily blocked in the 'learn_and_sing' future, the 'dance'
    // future will take over the current thread. If 'dance' becomes blocked,
    // 'learn_and_sing' can take back over. If both futures are blocked, then 
    // 'async_main' is blocked and will yield to the executor.
    futures::join!(f1, f2);
}