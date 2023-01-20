# friendly-barnacle

```sh
cp example.env .env
docker-compose up --build

brew install libpq
echo 'export PATH="/home/linuxbrew/.linuxbrew/opt/libpq/bin:$PATH"' >> ~/.profile
source ~/.profile
psql --version
psql -U app_user -W -h 127.0.0.1 defaultdb
```