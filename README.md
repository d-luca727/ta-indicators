# ta-indicators
 
With a strong focus on analyzing the crypto market, this service offers a no-frills, quick and handy interface to calculate the most useful TA indicators. It is still at an early stage of development, nonetheless the most used indicators have been already implemented.

# How to run it

1. `git clone https://github.com/Proioxis4/ta-indicators/`
2. add 
```
configuration/base.yaml
configuration/local.yaml
```
3. inside `base.yaml`
```
application:
  port: 8000
crypto_client:
  base_url: "https://coinranking1.p.rapidapi.com"
  auth_token: "COINRANKING_AUTH_KEY" //it is free
```

inside `local.yaml`
```
application:
  host: 127.0.0.1
```
4. `cargo run`
