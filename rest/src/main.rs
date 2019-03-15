#![deny(warnings)]
extern crate reqwest;

fn main() -> Result<(), Box<std::error::Error>> {

    let mut res = reqwest::get("https://us-east.manta.joyent.com/rui.loura/public/msan/platform-20180411T194341Z.tgz")?;

    //println!("Status: {}", res.status());
    //println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    std::io::copy(&mut res, &mut std::io::stdout())?;

    //println!("\n\nDone.");
    Ok(())
}
