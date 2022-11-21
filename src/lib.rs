use reqwest::Error;
use serde_derive::Deserialize; 
use serde_json::json;
use tokio::runtime::Runtime;

pub fn tx_raw_sync(network: String, tx_hex: String) -> Result<String, Error>{
    let rt = Runtime::new().unwrap();
    let future_fn = tx_raw(network, tx_hex);
    rt.block_on(future_fn)
}

pub async fn tx_raw(network: String, tx_hex: String) -> Result<String, Error>{
    let request_url = format!(
        "https://api.whatsonchain.com/v1/bsv/{network}/tx/raw",
        network=network
    );
    let params = json!({ "txhex": tx_hex }).to_string();
    // let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let response = client.post(&request_url)
        .header("Content-Type", "application/json")
        .body(params)
        .send()
        .await?;
    let ret: String = response.json().await?;
    Ok(ret)
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct ScriptHistory{
    tx_hash: String,
    height: u32,
}

pub fn script_history_sync(network: String, script_hash: String) -> Result<Vec<ScriptHistory>, Error>{
    let rt = Runtime::new().unwrap();
    let future_fn = script_history(network, script_hash);
    rt.block_on(future_fn)
}

pub async fn script_history(network: String, script_hash: String) -> Result<Vec<ScriptHistory>, Error>{
    let request_url = format!(
        "https://api.whatsonchain.com/v1/bsv/{network}/script/{script_hash}/history",
        network=network,
        script_hash=script_hash
    );
    let response = reqwest::get(&request_url).await?;
    let ret: Vec<ScriptHistory> = response.json().await?;
    Ok(ret)
}

pub fn script_unspent_sync(network: String, script_hash: String) -> Result<Vec<ScriptHistory>, Error>{
    let rt = Runtime::new().unwrap();
    let future_fn = script_unspent(network, script_hash);
    rt.block_on(future_fn)
}

pub async fn script_unspent(network: String, script_hash: String) -> Result<Vec<ScriptHistory>, Error>{
    let request_url = format!(
        "https://api.whatsonchain.com/v1/bsv/{network}/script/{script_hash}/unspent",
        network=network,
        script_hash=script_hash
    );
    let response = reqwest::get(&request_url).await?;
    let ret: Vec<ScriptHistory> = response.json().await?;
    Ok(ret)
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Unspent{
    height: u32,
    tx_pos: u16,
    tx_hash: String,
    value: u32,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct ScriptsUnspent{
    script: String,
    unspent: Vec<Unspent>,
    error: String,
}

pub fn scripts_unspent_sync(network: String, scripts: &Vec<String>) -> Result<Vec<ScriptsUnspent>, Error>{
    let rt = Runtime::new().unwrap();
    let future_fn = scripts_unspent(network, scripts);
    rt.block_on(future_fn)
}

pub async fn scripts_unspent(network: String, scripts: &Vec<String>) -> Result<Vec<ScriptsUnspent>, Error>{
    let request_url = format!(
        "https://api.whatsonchain.com/v1/bsv/{network}/scripts/unspent",
        network=network
    );
    let params = json!({ "scripts": scripts }).to_string();
    println!("params: {:?}", params);
    
    let client = reqwest::Client::new();
    let response = client.post(&request_url)
        .header("Content-Type", "application/json")
        .body(params)
        .send()
        .await?;
    let ret: Vec<ScriptsUnspent> = response.json().await?;
    Ok(ret)
}

pub fn woc_sync(network: String) -> Result<bool, Error> {
    let rt = Runtime::new().unwrap();
    let future_fn = woc(network);
    rt.block_on(future_fn)
}

pub async fn woc(network: String) -> Result<bool, Error> {
    let request_url = format!(
        "https://api.whatsonchain.com/v1/bsv/{network}/woc",
        network=network
    );
    let response = reqwest::get(&request_url).await?;
    let ret = response.text().await?;
    
    Ok(ret == "Whats On Chain")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new().unwrap();

        // test script_history
        let future_fn = script_history( "main".to_string(), "995ea8d0f752f41cdd99bb9d54cb004709e04c7dc4088bcbbbb9ea5c390a43c3".to_string());
        let result = rt.block_on(future_fn).unwrap();
        let ret = vec![
            ScriptHistory { tx_hash: "52dfceb815ad129a0fd946e3d665f44fa61f068135b9f38b05d3c697e11bad48".to_string(), height: 620539 },
            ScriptHistory { tx_hash: "4ec3b63d764558303eda720e8e51f69bbcfe81376075657313fb587306f8a9b0".to_string(), height: 620539 },
        ];
        assert_eq!(result, ret);

        // test woc
        let future_fn = woc("main".to_string());
        let result = rt.block_on(future_fn).unwrap();
        assert_eq!(result, true);
        
        let future_fn = woc("test".to_string());
        let result = rt.block_on(future_fn).unwrap();
        assert_eq!(result, true);
        
    }
}
