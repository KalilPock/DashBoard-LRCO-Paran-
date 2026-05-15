# Quickstart: Dashboard e Sincronia LRCO

## 1. Setup inicial
1. Certifique-se de que o ambiente Rust está configurado.
2. Certifique-se de que o banco de dados `my_project.db` está acessível.
3. Configure as credenciais do LRCO (variáveis de ambiente, se necessário).

## 2. Execução
1. Para rodar a sincronização: `cargo run -- sync`
2. Para rodar o servidor do dashboard: `cargo run -- dashboard`

## 3. Testes
- Rode `cargo test` para verificar a integração entre os serviços e o banco de dados.
