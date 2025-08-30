fn main(){
    println!("cargo:rustc-link-search=/home/develop/temperature-humidity-monitor/dht_driver/bin");

    let mut prost_build = prost_build::Config::new();
    prost_build.btree_map(&["."]);
    prost_build::compile_protos(&["proto/dht_sensor.proto"], &["proto", "src"]).unwrap();
}
