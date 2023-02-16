#[tokio::main]
async fn main() {
    iter_ipv4().await;
}

async fn iter_ipv4() {
    for i in 1..255 {
        for j in 1..255 {
            for k in 1..255 {
                for l in 1..255 {
                    tokio::spawn(async move {
                        let res = reqwest::get(format!("http://{}.{}.{}.{}", i, j, k, l)).await;

                        match res {
                            Ok(res) => {
                                if res.status().is_success() {
                                    println!("Success: {}.{}.{}.{}", i, j, k, l);
                                }
                            },
                            Err(_) => {},
                        };
                    });
                }
            }
        }
    }
}
