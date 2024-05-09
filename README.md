## Axumbooru
**Проект не завершен и находится в активной разработке!**

Выполнить для подготовки окружения к разработке
```bash
echo -e "DATABASE_URL=postgres://axumbooru:axumbooru@localhost/axumbooru\nRUST_LOG=debug" > .env
cp booruconfig_default.toml booruconfig.toml
cp docker-compose_templ.yaml docker-compose.yaml
# Запускает Postgresql на 5432,
# szurubooru/client на 80 и phpPgAdmin на 8080
docker compose up -d
cargo install sea-orm-cli
# Не забудь добавить ~/.cargo/bin в PATH
sea migrate
# Собирает и запускает Axumbooru
cargo run
```

Для обратной связи:
[Telegram](https://t.me/shiroyashik), [Discord](https://discord.com/users/563990794361634826).