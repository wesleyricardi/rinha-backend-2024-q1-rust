# Minha participação com Rust na Rinha Backend 2024 q1
Autor: `Wesley Ricardi`

## Repositorio
- [GitHub](https://github.com/wesleyricardi/rinha-backend-2024-q1-rust)

## Stack
- [Rust language](https://www.rust-lang.org/)
- [Actix-web](https://actix.rs/)
- [Sqlx](https://docs.rs/sqlx/latest/sqlx/) com [Postgres](https://www.postgresql.org/)

## Como iniciar:
### Requesitos:
* [git](https://git-scm.com/)
* [docker](https://git-scm.com/)

### Montando a aplicação 
No terminal use os seguintes comandos:
```sh
# Clone a aplicação
git clone https://github.com/wesleyricardi/rinha-backend-2024-q1-rust.git

# Abra a pasta
cd rinha-backend-2024-q1-rust

# Rode o docker compose para montar o container
docker compose up -d
```

### Executando
```sh
# para fazer build debug e rodar
cargo run

# para fazer o build release
cargo build --release
# o executavel está em: target/release/rinha-backend-2

# para abrir o build release
./target/release/rinha-backend-2
```

### Rodando
```sh
curl http://localhost:8080/
# se estiver tudo certo deve printar: hello world
```
Estando tudo certo pode abrir o swagger para testar e consultar as rotas clicando [aqui](http://localhost:8080/swagger/)


### Possiveis problemas
Caso tenha algum erro na montagem verifique se as portas configuradas no docker-compose.yml não estão sendo usadas, a configuração de porta padrão está `8080` para aplicação e `5432` para o banco de dados



## Configuração
### Conexão com o banco de dados
path: `src/config/database`

### Rotas
path: `src/config/services`

## Docs
Com a aplicação em execução
- [Swagger](http://localhost:9999/swagger-ui/)
- [JSON](http://localhost:9999/api-docs/openapi.json)