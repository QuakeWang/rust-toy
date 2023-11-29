async fn my_background_op(id: u32) -> String {
    let s = format!("Starting background task {}.", id);
    println!("{}", s);
    s
}

#[tokio::main]
async fn main() {
    let ops = vec![1, 2, 3];
    let mut tasks = Vec::with_capacity(ops.len());
    for op in ops {
        // 任务创建后，立即开始执行，用一个 Vec 来持有各个任务的 handler
        tasks.push(tokio::spawn(my_background_op(op)));
    }
    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
    // println!("{:?}", outputs);
}
