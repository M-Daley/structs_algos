

fn main() {
    let mut log = ::lib::TransactionLog::new_empty(20);
    for i in 0..10000 {
        log.append(i, format!("{}{}", "hello", i))
    }

    let res = log.find(9920).unwrap();

    eprintln!("{}", res)
}