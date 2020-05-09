use futures::channel::mpsc::{self};
use futures::executor::{self, ThreadPool};
use futures::stream::StreamExt;

fn main() {
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<i32>();

    // Create a future by an async block, where async is responsible for an
    // implementation of Future. At this point no executor has been provided
    // to this future, so it will not be running.
    let fut_values = async {
        // Create another async block, again where the Future implementatione is generated by
        // async. Since this is inside of a parent async block, it will be provided with the
        // executor of the parent block when the parent block is executed.
        //
        // This executor chaining is done by Future::poll whose second argument is a
        // std::task::Context. This represents our executor, and the Future implemented by this
        // async block can be polled using the parent async block's executor.

        let fut_tx_result = async move {
            (0..1000000).for_each(|v| {
                tx.unbounded_send(v).expect("Failed to send");
            })
        };

        // Use the provided thread pool to spawn the generated future
        // responsible for transmission
        pool.spawn_ok(fut_tx_result);

        let fut_values = rx.map(|v| v * 2).collect();

        // Use the executor provided to this async block to wait for the
        // future to complete.
        fut_values.await
    };

    // Actually execute the above future, which will invoke Future::poll and
    // subsequenty chain appropriate Future::poll and methods needing executors
    // to drive all futures. Eventually fut_values will be driven to completion.
    let values: Vec<i32> = executor::block_on(fut_values);

    println!("Values={:?}", values);
}
