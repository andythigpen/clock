fn main() {
    dotenvy::dotenv().ok();
    api::main();
}
