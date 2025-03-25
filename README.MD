Программа отправляет транзакцию на перевод SOL на заданный кошелек при каждом обнаружении нового блока.

# How to use

NOTE: работает только на unix или wsl на windows. 

1. Создайте файл конфигурации `config.yaml` на основе `config.example.yaml`:
   - Скопируйте `config.example.yaml` в `config.yaml`:
     ```bash
     cp config.example.yaml config.yaml
     ```
   - Отредактируйте `config.yaml`, указав свои значения.

2. Установите необходимые зависимости:
   ```bash
   sudo apt update && sudo apt install build-essential cmake pkg-config libssl-dev -y
   ```

3. Запустить.
    ```shell
    cargo run
    ```

## Команды
```text
Commands:                                                                                                                                                                                               
  create-wallet  Generate a new Solana wallet (keypair)                                                                                                                                                 
  help           Print this message or the help of the given subcommand(s)                                                                                                                              
                                                                                                                                                                                                        
Options:                                                                                                                                                                                                
  -c, --config-file <CONFIG_FILE>  [default: config.yaml]                                                                                                                                               
  -l, --log-level <LOG_LEVEL>      [default: INFO]                                                                                                                                                      
  -h, --help                       Print help                  
```
