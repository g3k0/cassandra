use exchange_api::alpaca_sdk::EnvVars;

fn main() {
  println!("Everything ok");
  let env_vars: EnvVars = EnvVars::get_vars();

  println!("{}", env_vars.api_base_url);
  println!("{}", env_vars.api_key_id);
  println!("{}", env_vars.api_secret);
}