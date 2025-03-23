1. Создать программу  для подписки на GRPC Yellowstone Geyser:
Реализовать CLI-программу, которая позволяет:
Настроить подписку на Geyser GRPC для отслеживания новых блоков. При каждом обнаружении блока необходимо выполнить транзакцию перевода SOL на заданный кошелек. Параметры транзакции (ключи и адрес кошелька) должны быть получены из файла config.yaml.

Гейзер: https://grpc.ny.shyft.to/
доки
https://docs.shyft.to/solana-grpc-shredstream/grpc-docs"

_________

2. Создать простое приложение на Rust, взаимодействующее с Solana. Нужно разработать смарт-контракт (программу), который позволяет пользователям вносить депозиты в SOL, отслеживать их баланс и выводить средства.

контракты можно тестировать в Localnet или Devnet, чтобы не требовались деньги на тесты

# How to use

```text
Commands:                                                                                                                                                                                               
  create-wallet  Generate a new Solana wallet (keypair)                                                                                                                                                 
  help           Print this message or the help of the given subcommand(s)                                                                                                                              
                                                                                                                                                                                                        
Options:                                                                                                                                                                                                
  -c, --config-file <CONFIG_FILE>  [default: config.yaml]                                                                                                                                               
  -l, --log-level <LOG_LEVEL>      [default: INFO]                                                                                                                                                      
  -h, --help                       Print help                  
```


# Development

## WSL setup

```shell
sudo apt update && sudo apt install build-essential cmake pkg-config libssl-dev -y
```